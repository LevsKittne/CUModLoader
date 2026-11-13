[English](../README.md) | [Русский](README.ru.md) | **Українська** | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Português](README.pt-BR.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md)

# CU ModLoader

Завантажувач модів для Casualties Unknown. Завантажує моди з папки `ModLoader\mods` і дає їм Harmony для патчів гри.

## Встановлення

1. Завантажте `CUModLoader-Setup.exe` з Releases.
2. Покладіть його в папку гри (поруч із `CasualtiesUnknown.exe`) і запустіть.
3. Інсталятор скопіює файли і видалить себе.

Якщо запустити інсталятор з іншого місця, він сам знайде гру через Steam і спитає, чи встановлювати туди.

Моди кладіть у `ModLoader\mods`.

## Видалення

Видаліть `version.dll` і папку `ModLoader` з папки гри.

## Якщо щось не так

- «Не вдалося записати файли» — закрийте гру; якщо не допомогло, запустіть інсталятор від імені адміністратора.
- Windows SmartScreen — «Докладніше» → «Однаково запустити» (файли не підписані).
- Помилка про `vcruntime140.dll` — встановіть Microsoft Visual C++ Redistributable (x64).
- Лог: `ModLoader\log.txt`.

Збирання з вихідного коду та створення модів — див. англійський README.
