[English](../README.md) | [Русский](README.ru.md) | [Українська](README.uk.md) | **简体中文** | [Español](README.es.md) | [Português](README.pt-BR.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md)

# CU ModLoader

Casualties Unknown 的模组加载器（Mod Loader）。从 `ModLoader\mods` 文件夹加载模组，并为模组提供 Harmony 用于修改游戏。

## 安装

1. 从 Releases 下载 `CUModLoader-Setup.exe`。
2. 将其放入游戏文件夹（与 `CasualtiesUnknown.exe` 同目录）并运行。
3. 安装程序会复制文件并自动删除自身。

如果在其他位置运行，安装程序会通过 Steam 自动查找游戏，并在安装前询问。

将模组放入 `ModLoader\mods`。

## 卸载

从游戏文件夹中删除 `version.dll` 和 `ModLoader` 文件夹。

## 常见问题

- “无法写入文件”——请关闭游戏；如果仍然失败，请以管理员身份运行安装程序。
- Windows SmartScreen——点击“更多信息”→“仍要运行”（文件未签名）。
- 提示缺少 `vcruntime140.dll`——请安装 Microsoft Visual C++ Redistributable (x64)。
- 日志：`ModLoader\log.txt`。

从源码构建和编写模组请参阅英文 README。
