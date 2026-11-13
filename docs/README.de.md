[English](../README.md) | [Русский](README.ru.md) | [Українська](README.uk.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Português](README.pt-BR.md) | **Deutsch** | [Français](README.fr.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md)

# CU ModLoader

Mod-Loader für Casualties Unknown. Lädt Mods aus dem Ordner `ModLoader\mods` und stellt ihnen Harmony zum Patchen des Spiels bereit.

## Installation

1. Lade `CUModLoader-Setup.exe` unter Releases herunter.
2. Lege die Datei in den Spielordner (neben `CasualtiesUnknown.exe`) und starte sie.
3. Der Installer kopiert die Dateien und löscht sich selbst.

Wird er woanders gestartet, findet er das Spiel über Steam und fragt vorher nach.

Mods kommen in `ModLoader\mods`.

## Deinstallation

Lösche `version.dll` und den Ordner `ModLoader` aus dem Spielordner.

## Fehlerbehebung

- „Dateien konnten nicht geschrieben werden“ — Spiel schließen; hilft das nicht, Installer als Administrator starten.
- Windows SmartScreen — „Weitere Informationen“ → „Trotzdem ausführen“ (die Dateien sind nicht signiert).
- Fehler zu `vcruntime140.dll` — Microsoft Visual C++ Redistributable (x64) installieren.
- Log: `ModLoader\log.txt`.

Bauen aus dem Quellcode und Mods schreiben: siehe englisches README.
