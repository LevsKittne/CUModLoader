[English](../README.md) | [Русский](README.ru.md) | [Українська](README.uk.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Português](README.pt-BR.md) | [Deutsch](README.de.md) | **Français** | [日本語](README.ja.md) | [한국어](README.ko.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md)

# CU ModLoader

Chargeur de mods pour Casualties Unknown. Charge les mods du dossier `ModLoader\mods` et leur fournit Harmony pour modifier le jeu.

## Installation

1. Téléchargez `CUModLoader-Setup.exe` depuis Releases.
2. Placez-le dans le dossier du jeu (à côté de `CasualtiesUnknown.exe`) et lancez-le.
3. L'installateur copie les fichiers puis se supprime.

S'il est lancé ailleurs, il trouve le jeu via Steam et demande confirmation.

Placez les mods dans `ModLoader\mods`.

## Désinstallation

Supprimez `version.dll` et le dossier `ModLoader` du dossier du jeu.

## Dépannage

- « Impossible d'écrire les fichiers » — fermez le jeu ; sinon, lancez l'installateur en tant qu'administrateur.
- Windows SmartScreen — « Informations complémentaires » → « Exécuter quand même » (fichiers non signés).
- Erreur concernant `vcruntime140.dll` — installez Microsoft Visual C++ Redistributable (x64).
- Journal : `ModLoader\log.txt`.

Pour compiler depuis les sources et écrire des mods, voir le README en anglais.
