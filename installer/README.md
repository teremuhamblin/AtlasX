Installateur Windows (.exe) pour AtlasX.

---

🧩 1. Où placer les fichiers liés à l’installateur

Voici l’emplacement idéal dans ton projet AtlasX :

`text
atlasx/
├─ installer/
│  ├─ atlasx-installer.iss      ← Script Inno Setup
│  ├─ Output/                   ← Le setup .exe généré (créé automatiquement)
│  └─ icon.ico                  ← (optionnel) icône du logiciel
`

👉 Tu dois créer un dossier installer/ à la racine du projet.  
C’est là que tu mets le script .iss et où Inno Setup générera ton Setup AtlasX.exe.

---

🛠️ 2. Préparer les fichiers à installer

Avant de créer l’installateur, tu dois préparer un dossier dist/ contenant ce que l’installateur va copier sur la machine de l’utilisateur.

Place-le à la racine :

`text
atlasx/
├─ dist/
│  ├─ bin/
│  │  └─ atlasx.exe             ← Le binaire Rust compilé
│  ├─ config/
│  │  └─ atlasx.config.toml
│  ├─ reports/
│  │  └─ templates/
│  │     ├─ index.html
│  │     └─ style.css
│  └─ docs/
│     └─ README.txt
`

👉 Tu dois compiler ton .exe avant :

`bash
cd atlasx/cli
cargo build --release
`

Puis copie :

`
atlasx/cli/target/release/atlasx.exe → atlasx/dist/bin/
`

---

📦 3. Le script complet de l’installateur Windows (.exe)

Place ce fichier dans :

`
atlasx/installer/atlasx-installer.iss
`

Voici le script :

`ini
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
`

---

🚀 4. Générer ton installateur Windows

1. Installe Inno Setup (gratuit).
2. Ouvre atlasx/installer/atlasx-installer.iss.
3. Clique sur Compile.

Tu obtiendras :

`
atlasx/installer/Output/AtlasX-Setup.exe
`

👉 C’est ton installateur Windows officiel.

---

🎯 Résumé ultra-simple

| Élément | Où le mettre |
|--------|--------------|
| Script installateur .iss | atlasx/installer/ |
| Binaire atlasx.exe | atlasx/dist/bin/ |
| Config | atlasx/dist/config/ |
| Templates HTML | atlasx/dist/reports/templates/ |
| README | atlasx/dist/docs/ |
| Installateur généré | atlasx/installer/Output/ |

---
