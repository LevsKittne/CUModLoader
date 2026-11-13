param([string]$GameDir = "D:\Program Files (x86)\Steam\steamapps\common\Casualties Unknown")

$dll = Join-Path $GameDir "version.dll"
if (Test-Path $dll) { Remove-Item $dll -Force; Write-Host "removed $dll (ModLoader folder kept)" }
