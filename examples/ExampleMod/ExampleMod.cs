using HarmonyLib;
using ModLoader;
using UnityEngine.SceneManagement;

namespace ExampleMod {
    [ModInfo("example", "Example Mod", "1.1.0", Author = "you")]
    public sealed class ExampleMod : Mod {
        internal static ExampleMod Instance;

        public override void OnLoad() {
            Instance = this;
            Harmony.PatchAll(typeof(ExampleMod).Assembly);
            GameConsole.AddCommand("example_echo", "Prints its arguments.", args => {
                GameConsole.Print(string.Join(" ", args, 1, args.Length - 1));
            }, ("string text", "Text to print."));
            Logger.Info("loaded, try `example_echo hello` in the game console");
        }

        public override void OnSceneLoaded(Scene scene, LoadSceneMode mode) {
            Logger.Debug($"scene {scene.name}");
        }
    }

    [HarmonyPatch(typeof(Body), "Start")]
    static class BodyStartPatch {
        static void Postfix(Body __instance) {
            ExampleMod.Instance?.Logger.Info($"body spawned: {__instance.name}");
        }
    }
}
