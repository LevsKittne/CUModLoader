[English](../README.md) | [Русский](README.ru.md) | [Українська](README.uk.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Português](README.pt-BR.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | **日本語** | [한국어](README.ko.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md)

# CU ModLoader

Casualties Unknown 用の MOD ローダーです。`ModLoader\mods` フォルダーの MOD を読み込み、ゲームをパッチするための Harmony を提供します。

## インストール

1. Releases から `CUModLoader-Setup.exe` をダウンロードします。
2. ゲームフォルダー（`CasualtiesUnknown.exe` と同じ場所）に置いて実行します。
3. インストーラーがファイルをコピーし、自動的に削除されます。

別の場所から実行した場合は、Steam からゲームを探し、インストール前に確認します。

MOD は `ModLoader\mods` に置いてください。

## アンインストール

ゲームフォルダーから `version.dll` と `ModLoader` フォルダーを削除します。

## トラブルシューティング

- 「ファイルを書き込めませんでした」— ゲームを終了してください。解決しない場合は管理者として実行してください。
- Windows SmartScreen — 「詳細情報」→「実行」（ファイルは署名されていません）。
- `vcruntime140.dll` のエラー — Microsoft Visual C++ Redistributable (x64) をインストールしてください。
- ログ：`ModLoader\log.txt`

ソースからのビルドと MOD の作り方は英語版 README を参照してください。
