from typing import List, Dict
from .scanner import AtlasXReport


def suggest_normalization(report: AtlasXReport) -> List[str]:
    suggestions = []

    if "[no-ext]" in report.by_extension:
        suggestions.append(
            "Certains fichiers n'ont pas d'extension (`[no-ext]`). "
            "Envisage de les renommer pour clarifier leur rôle."
        )

    for ext, stats in report.by_extension.items():
        if stats.count == 1 and stats.total_size < 1024:
            suggestions.append(
                f"L'extension `{ext}` n'apparaît qu'une fois et fait moins de 1 Ko. "
                "Vérifie si ce fichier est réellement nécessaire."
            )

    return suggestions


def suggest_grouping(report: AtlasXReport) -> Dict[str, List[str]]:
    groups: Dict[str, List[str]] = {
        "code": [],
        "assets": [],
        "docs": [],
        "archives": [],
        "autres": [],
    }

    mapping = {
        "code": ["rs", "go", "py", "js", "ts", "java", "php", "sh", "ps1"],
        "assets": ["png", "jpg", "jpeg", "svg", "gif", "ico"],
        "docs": ["md", "txt", "rst", "pdf"],
        "archives": ["zip", "tar", "gz", "rar", "7z"],
    }

    for ext in report.by_extension.keys():
        placed = False
        for group, exts in mapping.items():
            if ext in exts:
                groups[group].append(ext)
                placed = True
                break
        if not placed:
            groups["autres"].append(ext)

    return groups
