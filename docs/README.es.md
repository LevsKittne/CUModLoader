[English](../README.md) | [Русский](README.ru.md) | [Українська](README.uk.md) | [简体中文](README.zh-CN.md) | **Español** | [Português](README.pt-BR.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md)

# CU ModLoader

Cargador de mods para Casualties Unknown. Carga los mods de la carpeta `ModLoader\mods` y les da Harmony para parchear el juego.

## Instalación

1. Descarga `CUModLoader-Setup.exe` desde Releases.
2. Ponlo en la carpeta del juego (junto a `CasualtiesUnknown.exe`) y ejecútalo.
3. El instalador copia los archivos y se elimina solo.

Si lo ejecutas desde otro lugar, busca el juego a través de Steam y pregunta antes de instalar.

Pon los mods en `ModLoader\mods`.

## Desinstalación

Borra `version.dll` y la carpeta `ModLoader` de la carpeta del juego.

## Solución de problemas

- «No se pudieron escribir los archivos»: cierra el juego; si no funciona, ejecuta el instalador como administrador.
- Windows SmartScreen: «Más información» → «Ejecutar de todas formas» (los archivos no están firmados).
- Error sobre `vcruntime140.dll`: instala Microsoft Visual C++ Redistributable (x64).
- Registro: `ModLoader\log.txt`.

Para compilar desde el código fuente y crear mods, consulta el README en inglés.
