import sys
from pathlib import Path

EXPECTED = [
    "README.md",
    "cli/src/main.rs",
    "engine/atlasx_engine/suggestions.py",
    "reports/templates/index.html",
]

def main(root: str) -> int:
    base = Path(root)
    missing = [rel for rel in EXPECTED if not (base / rel).exists()]

    if missing:
        print("❌ Fichiers manquants :")
        for m in missing:
            print(" -", m)
        return 1

    print("✅ Structure minimale AtlasX OK")
    return 0

if __name__ == "__main__":
    sys.exit(main(sys.argv[1] if len(sys.argv) > 1 else "."))
