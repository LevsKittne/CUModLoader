[English](../README.md) | **Русский** | [Українська](README.uk.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Português](README.pt-BR.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md)

# CU ModLoader

Загрузчик модов для Casualties Unknown. Загружает моды из папки `ModLoader\mods` и даёт им Harmony для патчей игры.

## Установка

1. Скачайте `CUModLoader-Setup.exe` из Releases.
2. Положите его в папку игры (рядом с `CasualtiesUnknown.exe`) и запустите.
3. Установщик скопирует файлы и удалит себя.

Если запустить установщик из другого места, он сам найдёт игру через Steam и спросит, ставить ли туда.

Моды кладите в `ModLoader\mods`.

## Удаление

Удалите `version.dll` и папку `ModLoader` из папки игры.

## Если что-то не так

- «Не удалось записать файлы» — закройте игру; если не помогло, запустите установщик от имени администратора.
- Windows SmartScreen — «Подробнее» → «Выполнить в любом случае» (файлы не подписаны).
- Ошибка про `vcruntime140.dll` — установите Microsoft Visual C++ Redistributable (x64).
- Лог: `ModLoader\log.txt`.

Сборка из исходников и создание модов — см. английский README.
