[English](../README.md) | [Русский](README.ru.md) | [Українська](README.uk.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Português](README.pt-BR.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Türkçe](README.tr.md) | **Polski**

# CU ModLoader

Loader modów do Casualties Unknown. Wczytuje mody z folderu `ModLoader\mods` i udostępnia im Harmony do patchowania gry.

## Instalacja

1. Pobierz `CUModLoader-Setup.exe` z Releases.
2. Umieść go w folderze gry (obok `CasualtiesUnknown.exe`) i uruchom.
3. Instalator skopiuje pliki i sam się usunie.

Uruchomiony z innego miejsca znajdzie grę przez Steam i zapyta przed instalacją.

Mody umieszczaj w `ModLoader\mods`.

## Odinstalowanie

Usuń `version.dll` i folder `ModLoader` z folderu gry.

## Rozwiązywanie problemów

- „Nie udało się zapisać plików” — zamknij grę; jeśli nie pomoże, uruchom instalator jako administrator.
- Windows SmartScreen — „Więcej informacji” → „Uruchom mimo to” (pliki nie są podpisane).
- Błąd `vcruntime140.dll` — zainstaluj Microsoft Visual C++ Redistributable (x64).
- Log: `ModLoader\log.txt`.

Budowanie ze źródeł i tworzenie modów — zobacz angielskie README.
