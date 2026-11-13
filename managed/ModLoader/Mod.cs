using System;
using System.IO;
using HarmonyLib;
using UnityEngine.SceneManagement;

namespace ModLoader {
    [AttributeUsage(AttributeTargets.Class, AllowMultiple = false)]
    public sealed class ModInfoAttribute : Attribute {
        public string Id { get; }
        public string Name { get; }
        public string Version { get; }
        public string Author { get; set; } = "";

        public ModInfoAttribute(string id, string name, string version) {
            Id = id;
            Name = name;
            Version = version;
        }
    }

    [AttributeUsage(AttributeTargets.Class, AllowMultiple = true)]
    public sealed class ModDependencyAttribute : Attribute {
        public string Id { get; }
        public bool Optional { get; set; }

        public ModDependencyAttribute(string id) {
            Id = id;
        }
    }

    public abstract class Mod {
        public ModInfoAttribute Info { get; internal set; }
        public Logger Logger { get; internal set; }
        public Harmony Harmony { get; internal set; }
        public string AssemblyPath { get; internal set; }
        public bool Faulted { get; internal set; }

        IniFile config;

        public IniFile Config => config ?? (config = new IniFile(Path.Combine(Paths.ModConfigs, Info.Id + ".ini")));

        public virtual void OnLoad() {
        }

        public virtual void OnSceneLoaded(Scene scene, LoadSceneMode mode) {
        }

        public virtual void OnUpdate() {
        }

        public virtual void OnLateUpdate() {
        }

        public virtual void OnFixedUpdate() {
        }

        public virtual void OnGUI() {
        }

        public virtual void OnQuit() {
        }
    }
}
