using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Reflection;
using HarmonyLib;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace ModLoader {
    public static class Loader {
        public const string Version = "0.1.0";

        static readonly List<Mod> mods = new List<Mod>();
        static bool booted;
        static IniFile config;

        public static Logger Log { get; } = new Logger("ModLoader");
        public static IReadOnlyList<Mod> Mods => mods;
        public static Harmony Harmony { get; private set; }

        internal static void Init() {
            config = new IniFile(Paths.Config);
            Log.Info($"ModLoader {Version} managed init, mods dir: {Paths.Mods}");
            if (Native.Available) {
                Console.SetOut(new ConsoleRedirect(new Logger("Console")));
            }
            Harmony = new Harmony("cumodloader");
            var target = AccessTools.Method(typeof(SceneManager), "Internal_SceneLoaded");
            if (target == null) {
                Log.Error("SceneManager.Internal_SceneLoaded not found, mods will not load");
                return;
            }
            Harmony.Patch(target, postfix: new HarmonyMethod(typeof(Loader), nameof(SceneLoadedPostfix)));
        }

        static void SceneLoadedPostfix(Scene scene, LoadSceneMode mode) {
            if (!booted) {
                booted = true;
                try {
                    Boot();
                } catch (Exception e) {
                    Log.Error($"boot failed: {e}");
                }
            }
            Log.Debug($"scene loaded: {scene.name} ({mode})");
            Each(m => m.OnSceneLoaded(scene, mode), nameof(Mod.OnSceneLoaded));
        }

        static void Boot() {
            Log.Info($"Unity {Application.unityVersion}, game {Application.productName} {Application.version}");
            if (config.GetBool("loader", "unity_log", true)) {
                var unity = new Logger("Unity");
                Application.logMessageReceived += (msg, trace, type) => {
                    var level = type == LogType.Log ? LogLevel.Info : type == LogType.Warning ? LogLevel.Warning : LogLevel.Error;
                    unity.Write(level, level == LogLevel.Error && !string.IsNullOrEmpty(trace) ? msg + "\n" + trace.TrimEnd() : msg);
                };
            }
            var host = new GameObject("CUModLoader");
            host.hideFlags = HideFlags.HideAndDontSave;
            UnityEngine.Object.DontDestroyOnLoad(host);
            host.AddComponent<ModHost>();
            try {
                GameConsole.Init(Harmony);
                GameConsole.AddCommand("mods", "Lists loaded mods.", args => {
                    GameConsole.Print($"CUModLoader {Version}, {mods.Count} mod(s):");
                    foreach (var m in mods) {
                        GameConsole.Print($"  {m.Info.Name} {m.Info.Version} ({m.Info.Id}){(m.Faulted ? " <color=red>faulted</color>" : "")}");
                    }
                });
            } catch (Exception e) {
                Log.Error($"game console hook failed: {e}");
            }
            LoadMods();
        }

        static void LoadMods() {
            var disabled = new HashSet<string>(config.GetList("mods", "disabled"), StringComparer.OrdinalIgnoreCase);
            var found = new Dictionary<string, Mod>(StringComparer.OrdinalIgnoreCase);
            foreach (var file in Directory.GetFiles(Paths.Mods, "*.dll", SearchOption.AllDirectories)) {
                Assembly asm;
                try {
                    AssemblyName.GetAssemblyName(file);
                } catch (BadImageFormatException) {
                    continue;
                } catch (Exception) {
                }
                try {
                    asm = Assembly.LoadFrom(file);
                } catch (Exception e) {
                    Log.Error($"cannot load {Path.GetFileName(file)}: {e.Message}");
                    continue;
                }
                foreach (var type in SafeTypes(asm)) {
                    if (type == null || type.IsAbstract || !typeof(Mod).IsAssignableFrom(type)) {
                        continue;
                    }
                    var info = type.GetCustomAttribute<ModInfoAttribute>() ?? new ModInfoAttribute(type.FullName, type.Name, asm.GetName().Version.ToString());
                    if (disabled.Contains(info.Id)) {
                        Log.Info($"skipping disabled mod {info.Id}");
                        continue;
                    }
                    if (found.ContainsKey(info.Id)) {
                        Log.Error($"duplicate mod id {info.Id} in {Path.GetFileName(file)}, skipped");
                        continue;
                    }
                    try {
                        var mod = (Mod)Activator.CreateInstance(type);
                        mod.Info = info;
                        mod.Logger = new Logger(info.Name);
                        mod.Harmony = new Harmony(info.Id);
                        mod.AssemblyPath = file;
                        found[info.Id] = mod;
                    } catch (Exception e) {
                        Log.Error($"cannot create {type.FullName}: {e}");
                    }
                }
            }
            foreach (var mod in Sort(found)) {
                try {
                    mod.OnLoad();
                    mods.Add(mod);
                    Log.Info($"loaded {mod.Info.Name} {mod.Info.Version} ({mod.Info.Id})");
                } catch (Exception e) {
                    Log.Error($"{mod.Info.Id} OnLoad failed: {e}");
                }
            }
            Log.Info($"{mods.Count} mod(s) loaded");
        }

        static IEnumerable<Type> SafeTypes(Assembly asm) {
            try {
                return asm.GetTypes();
            } catch (ReflectionTypeLoadException e) {
                return e.Types.Where(t => t != null);
            }
        }

        static List<Mod> Sort(Dictionary<string, Mod> found) {
            var result = new List<Mod>();
            var state = new Dictionary<string, int>(StringComparer.OrdinalIgnoreCase);
            bool Visit(Mod mod) {
                var id = mod.Info.Id;
                if (state.TryGetValue(id, out var s)) {
                    if (s == 1) {
                        Log.Error($"dependency cycle at {id}");
                    }
                    return s == 2;
                }
                state[id] = 1;
                foreach (var dep in mod.GetType().GetCustomAttributes<ModDependencyAttribute>()) {
                    if (found.TryGetValue(dep.Id, out var other)) {
                        if (!Visit(other) && !dep.Optional) {
                            Log.Error($"{id} skipped: dependency {dep.Id} failed");
                            state[id] = 3;
                            return false;
                        }
                    } else if (!dep.Optional) {
                        Log.Error($"{id} skipped: missing dependency {dep.Id}");
                        state[id] = 3;
                        return false;
                    }
                }
                state[id] = 2;
                result.Add(mod);
                return true;
            }
            foreach (var mod in found.Values.OrderBy(m => m.Info.Id, StringComparer.OrdinalIgnoreCase)) {
                Visit(mod);
            }
            return result;
        }

        internal static void Each(Action<Mod> action, string what) {
            for (var i = 0; i < mods.Count; i++) {
                var mod = mods[i];
                if (mod.Faulted) {
                    continue;
                }
                try {
                    action(mod);
                } catch (Exception e) {
                    mod.Faulted = true;
                    Log.Error($"{mod.Info.Id} {what} threw, mod callbacks disabled: {e}");
                }
            }
        }
    }

    sealed class ModHost : MonoBehaviour {
        void Update() => Loader.Each(m => m.OnUpdate(), nameof(Mod.OnUpdate));
        void LateUpdate() => Loader.Each(m => m.OnLateUpdate(), nameof(Mod.OnLateUpdate));
        void FixedUpdate() => Loader.Each(m => m.OnFixedUpdate(), nameof(Mod.OnFixedUpdate));
        void OnGUI() => Loader.Each(m => m.OnGUI(), nameof(Mod.OnGUI));
        void OnApplicationQuit() => Loader.Each(m => m.OnQuit(), nameof(Mod.OnQuit));
    }
}
