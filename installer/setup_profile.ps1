param([string]$AppDir)

$exe = Join-Path $AppDir 'linker.exe'

# $PROFILE のディレクトリとファイルを作成
$profileDir = Split-Path $PROFILE
if ($profileDir -and !(Test-Path $profileDir)) {
    New-Item -Path $profileDir -ItemType Directory -Force | Out-Null
}
if (!(Test-Path $PROFILE)) {
    New-Item -Path $PROFILE -ItemType File -Force | Out-Null
}

# 既に l 関数が登録済みなら何もしない（再インストール時の重複防止）
$existing = Get-Content $PROFILE -Raw -ErrorAction SilentlyContinue
if ($existing -match 'function l\b') { exit 0 }

# $PROFILE に追記
$snippet = & $exe init --shell powershell
Add-Content -Path $PROFILE -Value "`r`n$snippet"
