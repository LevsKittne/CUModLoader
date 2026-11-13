param(
    [string]$GameDir = "D:\Program Files (x86)\Steam\steamapps\common\Casualties Unknown",
    [switch]$Install
)

$ErrorActionPreference = "Stop"
$root = $PSScriptRoot
$dist = Join-Path $root "dist"
$core = Join-Path $dist "ModLoader\core"
$mods = Join-Path $dist "ModLoader\mods"
$setup = Join-Path $dist "setup"

if (Test-Path $dist) { Remove-Item $dist -Recurse -Force }
New-Item -ItemType Directory -Force $core, $mods, $setup | Out-Null

cargo build --release --manifest-path (Join-Path $root "Cargo.toml") -p cu-modloader
if ($LASTEXITCODE -ne 0) { throw "cargo build failed" }
Copy-Item (Join-Path $root "target\release\version.dll") $dist

function Build($proj, $out) {
    dotnet build (Join-Path $root $proj) -c Release "-p:GameDir=$GameDir" -o (Join-Path $root $out)
    if ($LASTEXITCODE -ne 0) { throw "$proj build failed" }
}

Build "managed\ModLoader\ModLoader.csproj" "target\managed"
Build "examples\ExampleMod\ExampleMod.csproj" "target\example"

Copy-Item (Join-Path $root "target\managed\ModLoader.dll") $core
Copy-Item (Join-Path $root "libs\*.dll") $core
Copy-Item (Join-Path $root "target\example\ExampleMod.dll") $mods

cargo build --release --manifest-path (Join-Path $root "Cargo.toml") -p installer
if ($LASTEXITCODE -ne 0) { throw "installer build failed" }
Copy-Item (Join-Path $root "target\release\CUModLoader-Setup.exe") $setup

if ($Install) {
    if (-not (Test-Path (Join-Path $GameDir "CasualtiesUnknown.exe"))) { throw "game not found in $GameDir" }
    Copy-Item (Join-Path $dist "*") $GameDir -Recurse -Force -Exclude setup
    Write-Host "installed to $GameDir"
}
