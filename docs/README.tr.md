[English](../README.md) | [Русский](README.ru.md) | [Українська](README.uk.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Português](README.pt-BR.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | **Türkçe** | [Polski](README.pl.md)

# CU ModLoader

Casualties Unknown için mod yükleyici. `ModLoader\mods` klasöründeki modları yükler ve oyunu yamalamak için Harmony sağlar.

## Kurulum

1. Releases bölümünden `CUModLoader-Setup.exe` dosyasını indirin.
2. Oyun klasörüne (`CasualtiesUnknown.exe` yanına) koyun ve çalıştırın.
3. Yükleyici dosyaları kopyalar ve kendini siler.

Başka bir yerden çalıştırılırsa oyunu Steam üzerinden bulur ve kurmadan önce sorar.

Modları `ModLoader\mods` klasörüne koyun.

## Kaldırma

Oyun klasöründen `version.dll` dosyasını ve `ModLoader` klasörünü silin.

## Sorun giderme

- "Dosyalar yazılamadı" — oyunu kapatın; olmazsa yükleyiciyi yönetici olarak çalıştırın.
- Windows SmartScreen — "Ek bilgi" → "Yine de çalıştır" (dosyalar imzalı değil).
- `vcruntime140.dll` hatası — Microsoft Visual C++ Redistributable (x64) kurun.
- Günlük: `ModLoader\log.txt`

Kaynak koddan derleme ve mod yazma için İngilizce README'ye bakın.
