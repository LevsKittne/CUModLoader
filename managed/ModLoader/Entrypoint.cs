using System;
using System.IO;
using System.Reflection;
using System.Runtime.CompilerServices;

namespace ModLoader {
    public static class Entrypoint {
        static bool started;

        public static void Start() {
            if (started) {
                return;
            }
            started = true;
            Paths.Init();
            AppDomain.CurrentDomain.AssemblyResolve += Resolve;
            Boot();
        }

        [MethodImpl(MethodImplOptions.NoInlining)]
        static void Boot() {
            Loader.Init();
        }

        static Assembly Resolve(object sender, ResolveEventArgs args) {
            var name = new AssemblyName(args.Name).Name;
            foreach (var asm in AppDomain.CurrentDomain.GetAssemblies()) {
                if (asm.GetName().Name == name) {
                    return asm;
                }
            }
            foreach (var dir in new[] { Paths.Core, Paths.Mods }) {
                var path = Path.Combine(dir, name + ".dll");
                if (File.Exists(path)) {
                    return Assembly.LoadFrom(path);
                }
            }
            return null;
        }
    }

    public static class Paths {
        public static string Core { get; private set; }
        public static string Root { get; private set; }
        public static string Mods { get; private set; }
        public static string Config { get; private set; }
        public static string ModConfigs { get; private set; }
        public static string Log { get; private set; }
        public static string Game { get; private set; }

        internal static void Init() {
            Core = Path.GetDirectoryName(typeof(Paths).Assembly.Location);
            Root = Path.GetDirectoryName(Core);
            Game = Path.GetDirectoryName(Root);
            Mods = Path.Combine(Root, "mods");
            ModConfigs = Path.Combine(Root, "config");
            Config = Path.Combine(Root, "loader.ini");
            Log = Path.Combine(Root, "log.txt");
            Directory.CreateDirectory(Mods);
            Directory.CreateDirectory(ModConfigs);
        }
    }
}
