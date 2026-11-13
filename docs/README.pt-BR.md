[English](../README.md) | [Русский](README.ru.md) | [Українська](README.uk.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | **Português** | [Deutsch](README.de.md) | [Français](README.fr.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Türkçe](README.tr.md) | [Polski](README.pl.md)

# CU ModLoader

Carregador de mods para Casualties Unknown. Carrega os mods da pasta `ModLoader\mods` e fornece o Harmony para modificar o jogo.

## Instalação

1. Baixe `CUModLoader-Setup.exe` em Releases.
2. Coloque-o na pasta do jogo (ao lado de `CasualtiesUnknown.exe`) e execute.
3. O instalador copia os arquivos e se exclui.

Se for executado em outro lugar, ele encontra o jogo pela Steam e pergunta antes de instalar.

Coloque os mods em `ModLoader\mods`.

## Desinstalação

Apague `version.dll` e a pasta `ModLoader` da pasta do jogo.

## Solução de problemas

- “Não foi possível gravar os arquivos” — feche o jogo; se não resolver, execute o instalador como administrador.
- Windows SmartScreen — “Mais informações” → “Executar assim mesmo” (os arquivos não são assinados).
- Erro sobre `vcruntime140.dll` — instale o Microsoft Visual C++ Redistributable (x64).
- Log: `ModLoader\log.txt`.

Para compilar a partir do código-fonte e criar mods, veja o README em inglês.
