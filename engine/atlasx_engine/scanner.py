from pydantic import BaseModel
from typing import Dict, List


class ExtensionStats(BaseModel):
    count: int
    total_size: int
    sample_paths: List[str]


class AtlasXReport(BaseModel):
    scanned_root: str
    generated_at: str
    total_files: int
    total_size: int
    by_extension: Dict[str, ExtensionStats]
