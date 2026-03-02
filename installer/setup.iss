; Linker - CLI Jump Tool
; Inno Setup 6 Script

#define AppName    "Linker"
#define AppVersion "0.1.0"
#define AppExe     "linker.exe"
#define AppPublisher "mashiro3"

[Setup]
AppId={{6B3A7E91-2D4C-4F85-A1B3-C5D7E9F21B4A}
AppName={#AppName}
AppVersion={#AppVersion}
AppVerName={#AppName} {#AppVersion}
AppPublisher={#AppPublisher}
DefaultDirName={autopf}\{#AppName}
DefaultGroupName={#AppName}
DisableProgramGroupPage=yes

OutputDir=output
OutputBaseFilename=linker-installer-{#AppVersion}
SetupIconFile=icon.ico

; 328x628 = Inno Setup 6 高DPI (2x) ウィザードサイドイメージ
WizardImageFile=installer_left2x.bmp

LicenseFile=license.txt

WizardStyle=modern
Compression=lzma2/ultra64
SolidCompression=yes
PrivilegesRequired=admin
UsedUserAreasWarning=no
ChangesEnvironment=yes
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0
DisableWelcomePage=no

[Languages]
Name: "japanese"; MessagesFile: "compiler:Languages\Japanese.isl"
Name: "english";  MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "addpath";      Description: "システム PATH に追加する";                          GroupDescription: "環境変数:"
Name: "addpsprofile"; Description: "PowerShell プロファイルにシェル関数 'l' を追加する"; GroupDescription: "シェル設定:"

#ifndef BinDir
  #define BinDir "..\target\release"
#endif

[Files]
Source: "{#BinDir}\{#AppExe}"; DestDir: "{app}"; Flags: ignoreversion

[Registry]
; システム PATH への追加 (重複チェック付き)
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; \
  ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; \
  Check: NeedsAddPath(ExpandConstant('{app}')); Tasks: addpath
; プロファイル設定済みマーク (アンインストール時に自動削除される)
Root: HKCU; Subkey: "Software\Linker"; ValueType: none; ValueName: "ProfileSetup"; \
  Flags: dontcreatekey uninsdeletevalue; Tasks: addpsprofile

[Code]
{ 'l' シェル関数のスニペット。init.rs の PowerShell スニペットと同一に保つこと }
function GetSnippet(): String;
begin
  Result :=
    '# Add the following to your PowerShell profile ($PROFILE):' + #13#10 +
    'function l {' + #13#10 +
    '    $result = & linker @args' + #13#10 +
    '    $code = $LASTEXITCODE' + #13#10 +
    '    if ($code -eq 2) {' + #13#10 +
    '        Set-Location ($result -join "")' + #13#10 +
    '    } elseif ($code -eq 0) {' + #13#10 +
    '        $result' + #13#10 +
    '    } else {' + #13#10 +
    '        $result | ForEach-Object { Write-Error $_ }' + #13#10 +
    '    }' + #13#10 +
    '}';
end;

{ ProfilePath に 'l' 関数を追記する。
  重複防止: HKCU\Software\Linker\ProfileSetup が存在すればスキップ }
procedure WriteToProfile(const ProfilePath: String);
var
  ProfileDir: String;
  Sentinel: String;
  OK: Boolean;
begin
  { 設定済みマークがあれば再追記しない }
  if RegQueryStringValue(HKEY_CURRENT_USER,
      'Software\Linker', 'ProfileSetup', Sentinel) then Exit;

  ProfileDir := ExtractFilePath(ProfilePath);
  if ProfileDir <> '' then
    OK := ForceDirectories(ProfileDir);
  if not FileExists(ProfilePath) then
    OK := SaveStringToFile(ProfilePath, '', False);
  OK := SaveStringToFile(ProfilePath, GetSnippet() + #13#10, True);
  OK := RegWriteStringValue(HKEY_CURRENT_USER,
      'Software\Linker', 'ProfileSetup', '1');
end;

{ PATH に対象ディレクトリが含まれていなければ True を返す }
function NeedsAddPath(Param: String): Boolean;
var
  OrigPath: String;
begin
  if not RegQueryStringValue(
    HKEY_LOCAL_MACHINE,
    'SYSTEM\CurrentControlSet\Control\Session Manager\Environment',
    'Path', OrigPath) then
  begin
    Result := True;
    Exit;
  end;
  Result := Pos(';' + Uppercase(Param) + ';',
                ';' + Uppercase(OrigPath) + ';') = 0;
end;

procedure CurStepChanged(CurStep: TSetupStep);
var
  UserProfile: String;
begin
  if (CurStep <> ssPostInstall) or not WizardIsTaskSelected('addpsprofile') then Exit;
  UserProfile := GetEnv('USERPROFILE');
  if UserProfile = '' then Exit;
  { PS7 と PS5 の両方のプロファイルに書き込む }
  WriteToProfile(UserProfile + '\Documents\PowerShell\Microsoft.PowerShell_profile.ps1');
  WriteToProfile(UserProfile + '\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1');
end;
