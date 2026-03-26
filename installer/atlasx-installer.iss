[Setup]
AppName=AtlasX
AppVersion=0.1.0
DefaultDirName={pf}\AtlasX
DefaultGroupName=AtlasX
OutputBaseFilename=AtlasX-Setup
Compression=lzma
SolidCompression=yes

[Languages]
Name: "french"; MessagesFile: "compiler:Languages\French.isl"

[Files]
Source: "..\dist\bin\atlasx.exe"; DestDir: "{app}\bin"; Flags: ignoreversion
Source: "..\dist\config\atlasx.config.toml"; DestDir: "{app}\config"; Flags: ignoreversion
Source: "..\dist\reports\templates\*"; DestDir: "{app}\reports\templates"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "..\dist\docs\*"; DestDir: "{app}\docs"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\AtlasX"; Filename: "{app}\bin\atlasx.exe"
Name: "{commondesktop}\AtlasX"; Filename: "{app}\bin\atlasx.exe"; Tasks: desktopicon

[Tasks]
Name: "desktopicon"; Description: "Créer un raccourci sur le bureau"; GroupDescription: "Raccourcis :"

[Run]
Filename: "{app}\docs\README.txt"; Description: "Afficher le README"; Flags: postinstall shellexec skipifsilent
