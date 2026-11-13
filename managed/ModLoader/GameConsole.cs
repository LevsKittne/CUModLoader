using System;
using System.Collections.Generic;
using System.Reflection;
using HarmonyLib;

namespace ModLoader {
    public static class GameConsole {
        static readonly List<Command> pending = new List<Command>();
        static readonly HashSet<string> owned = new HashSet<string>(StringComparer.OrdinalIgnoreCase);
        static MethodInfo logToConsole;
        static MethodInfo tryExecute;

        internal static void Init(Harmony harmony) {
            logToConsole = AccessTools.Method(typeof(ConsoleScript), "LogToConsole");
            tryExecute = AccessTools.Method(typeof(ConsoleScript), "TryExecuteCommand");
            var register = AccessTools.Method(typeof(ConsoleScript), nameof(ConsoleScript.RegisterAllCommands));
            harmony.Patch(register, postfix: new HarmonyMethod(typeof(GameConsole), nameof(RegisterPostfix)));
        }

        static void RegisterPostfix() {
            foreach (var cmd in pending) {
                Add(cmd);
            }
            pending.Clear();
        }

        public static void AddCommand(string name, string description, Action<string[]> action, params (string, string)[] args) {
            var cmd = new Command(name, description, a => action(a), null, args);
            if (ConsoleScript.Commands.Count > 0) {
                Add(cmd);
            } else {
                pending.Add(cmd);
            }
        }

        static void Add(Command cmd) {
            if (!owned.Contains(cmd.name) && ConsoleScript.Commands.Exists(c => string.Equals(c.name, cmd.name, StringComparison.OrdinalIgnoreCase))) {
                Loader.Log.Warning($"console command {cmd.name} already exists in the game, mod command skipped");
                return;
            }
            owned.Add(cmd.name);
            ConsoleScript.Commands.RemoveAll(c => string.Equals(c.name, cmd.name, StringComparison.OrdinalIgnoreCase));
            ConsoleScript.Commands.Add(cmd);
        }

        public static bool Execute(string line) {
            var console = ConsoleScript.instance;
            if (console == null || tryExecute == null || string.IsNullOrWhiteSpace(line)) {
                return false;
            }
            var args = line.Trim().Split(new[] { ' ' }, StringSplitOptions.RemoveEmptyEntries);
            tryExecute.Invoke(console, new object[] { args, true });
            return true;
        }

        public static void Print(string text) {
            var console = ConsoleScript.instance;
            if (console == null || logToConsole == null) {
                return;
            }
            logToConsole.Invoke(console, new object[] { text });
        }
    }
}
