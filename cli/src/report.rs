use crate::model::AtlasXReport;
use std::fs;
use std::path::Path;

pub fn write_json(report: &AtlasXReport, out_path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(report).unwrap();
    fs::write(out_path, json)
}

pub fn write_markdown(report: &AtlasXReport, out_path: &str) -> std::io::Result<()> {
    let mut md = String::new();
    md.push_str("# AtlasX\n\n");
    md.push_str(&format!("- Racine scannée : `{}`\n", report.scanned_root));
    md.push_str(&format!("- Généré le : `{}`\n", report.generated_at));
    md.push_str(&format!("- Fichiers totaux : `{}`\n", report.total_files));
    md.push_str(&format!("- Taille totale : `{}` octets\n\n", report.total_size));

    md.push_str("## Par extension\n\n");
    md.push_str("| Extension | Fichiers | Taille totale (octets) |\n");
    md.push_str("|-----------|----------|------------------------|\n");

    for (ext, stats) in &report.by_extension {
        md.push_str(&format!(
            "| `{}` | {} | {} |\n",
            ext, stats.count, stats.total_size
        ));
    }

    fs::write(out_path, md)
}

pub fn write_html_from_template(
    report: &AtlasXReport,
    template_dir: &str,
    out_path: &str,
) -> std::io::Result<()> {
    let template_path = Path::new(template_dir).join("index.html");
    let template = fs::read_to_string(template_path)?;

    let json = serde_json::to_string(report).unwrap();
    let html = template.replace("{{ATLASX_DATA_JSON}}", &json);

    fs::write(out_path, html)
}
