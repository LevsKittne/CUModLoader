[English](../README.md) | [Русский](README.ru.md) | [Українська](README.uk.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Português](README.pt-BR.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [日本語](README.ja.md) | **한국어** | [Türkçe](README.tr.md) | [Polski](README.pl.md)

# CU ModLoader

Casualties Unknown용 모드 로더입니다. `ModLoader\mods` 폴더의 모드를 불러오고, 게임 패치를 위한 Harmony를 제공합니다.

## 설치

1. Releases에서 `CUModLoader-Setup.exe`를 내려받습니다.
2. 게임 폴더(`CasualtiesUnknown.exe`가 있는 곳)에 넣고 실행합니다.
3. 설치 프로그램이 파일을 복사한 뒤 스스로 삭제됩니다.

다른 위치에서 실행하면 Steam을 통해 게임을 찾고 설치 전에 확인합니다.

모드는 `ModLoader\mods`에 넣으세요.

## 제거

게임 폴더에서 `version.dll`과 `ModLoader` 폴더를 삭제합니다.

## 문제 해결

- "파일을 쓸 수 없습니다" — 게임을 종료하세요. 그래도 안 되면 관리자 권한으로 실행하세요.
- Windows SmartScreen — "추가 정보" → "실행" (파일이 서명되지 않았습니다).
- `vcruntime140.dll` 오류 — Microsoft Visual C++ Redistributable (x64)를 설치하세요.
- 로그: `ModLoader\log.txt`

소스 빌드와 모드 제작은 영어 README를 참고하세요.
