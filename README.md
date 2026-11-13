**English** | [Русский](docs/README.ru.md) | [Українська](docs/README.uk.md) | [简体中文](docs/README.zh-CN.md) | [Español](docs/README.es.md) | [Português](docs/README.pt-BR.md) | [Deutsch](docs/README.de.md) | [Français](docs/README.fr.md) | [日本語](docs/README.ja.md) | [한국어](docs/README.ko.md) | [Türkçe](docs/README.tr.md) | [Polski](docs/README.pl.md)

# CU ModLoader

Mod loader for **Casualties Unknown** (Unity, Mono). Install mods for Casualties Unknown with one click, or write your own with Harmony.

`version.dll` (Rust) is a proxy for the system library: it starts inside the game, bootstraps the Mono domain and loads the managed `ModLoader.dll`, which loads every mod from `ModLoader\mods` and gives it Harmony for patching.

Multiplayer / co-op mod built on it: [Casualties: Together](https://github.com/LevsKittne/CasualtiesTogether).

## Installation

1. Download `CUModLoader-Setup.exe` from [Releases](https://github.com/LevsKittne/CUModLoader/releases).
2. Put it into the game folder (next to `CasualtiesUnknown.exe`) and run it.
3. The installer copies the files and deletes itself.

Run from another place, it finds the game through Steam and asks before installing. The installer speaks English, Russian, Ukrainian, Chinese, Spanish, Portuguese, German, French, Japanese, Korean, Turkish and Polish (picked from the Windows language). `/silent` installs without dialogs.

After installation:

```
version.dll
ModLoader\
  core\        loader, Harmony, Mono.Cecil, MonoMod
  mods\        mods (*.dll, subfolders allowed)
  config\      mod settings
  loader.ini   loader settings
  log.txt
```

## Uninstall

Delete `version.dll` and the `ModLoader` folder from the game folder. To disable mods temporarily set `enabled = false` in `ModLoader\loader.ini`.

## Troubleshooting

- "Could not write files": close the game; if that does not help, run the installer as administrator.
- Windows SmartScreen: "More info" → "Run anyway" (the files are not signed).
- Error about `vcruntime140.dll`: install Microsoft Visual C++ Redistributable (x64).
- Log: `ModLoader\log.txt`.

## loader.ini

```ini
[loader]
enabled = true
console = true
unity_log = true

[mods]
disabled =
```

## Writing a mod

Create a `net472` class library that references `ModLoader.dll` and `Assembly-CSharp.dll`. A full example is in `examples/ExampleMod`.

```csharp
using ModLoader;

[ModInfo("example", "Example Mod", "1.0.0", Author = "you")]
public sealed class ExampleMod : Mod {
    public override void OnLoad() {
    }
}
```

Hooks: `OnLoad`, `OnSceneLoaded`, `OnUpdate`, `OnLateUpdate`, `OnFixedUpdate`, `OnGUI`, `OnQuit`. Per-mod settings: `Config` (`ModLoader\config\<id>.ini`). Put the built dll into `ModLoader\mods`.

## Building

Requires Rust (stable, MSVC), .NET SDK and the installed game (references to `Assembly-CSharp` and `UnityEngine` come from it).

```powershell
.\build.ps1 -GameDir "C:\...\Casualties Unknown"
.\build.ps1 -Install
```

Output goes to `dist\`, the installer to `dist\setup\`.

## Third-party

`libs\`: Harmony, Mono.Cecil, MonoMod (MIT).
