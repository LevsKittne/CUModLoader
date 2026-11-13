# CU ModLoader

Загрузчик модов для **Casualties Unknown** (Unity, Mono).

`version.dll` (Rust) подменяет системную библиотеку, поднимает Mono-домен игры и загружает управляемый `ModLoader.dll`. Тот в свою очередь грузит все моды из `ModLoader\mods` и даёт им Harmony для патчей.

## Установка

1. Скачайте `CUModLoader-Setup.exe` из Releases.
2. Положите его в папку игры (рядом с `CasualtiesUnknown.exe`) и запустите.
3. Установщик поставит файлы и удалит себя.

Если запустить его из другого места, он сам найдёт игру через Steam.

После установки в папке игры:

```
version.dll
ModLoader\
  core\        загрузчик, Harmony, Mono.Cecil, MonoMod
  mods\        моды (*.dll, можно в подпапках)
  config\      настройки модов
  loader.ini   настройки загрузчика
  log.txt
```

Удаление: удалите `version.dll` и папку `ModLoader`.

## loader.ini

```ini
[loader]
enabled = true
console = true
unity_log = true

[mods]
disabled =
```

## Как написать мод

Сделайте библиотеку под `net472` со ссылками на `ModLoader.dll` и `Assembly-CSharp.dll`. Готовый пример лежит в `examples/ExampleMod`.

```csharp
using ModLoader;

[ModInfo("example", "Example Mod", "1.0.0", Author = "you")]
public sealed class ExampleMod : Mod {
    public override void OnLoad() {
    }
}
```

Хуки: `OnLoad`, `OnSceneLoaded`, `OnUpdate`, `OnLateUpdate`, `OnFixedUpdate`, `OnGUI`, `OnQuit`. Свои настройки мод хранит через `Config`, это файл `ModLoader\config\<id>.ini`.

## Сборка

Нужны Rust (stable, MSVC), .NET SDK и установленная игра. Сборка без игры не пройдёт: ссылки на `Assembly-CSharp` и `UnityEngine` берутся из неё.

```powershell
.\build.ps1 -GameDir "C:\...\Casualties Unknown"
.\build.ps1 -Install
```

Результат кладётся в `dist\`, установщик — в `dist\setup\`.

## Сторонние библиотеки

`libs\`: Harmony, Mono.Cecil, MonoMod (MIT).
