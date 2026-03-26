mod scanner;
mod model;
mod report;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "atlasx")]
#[command(about = "Scanner et cartographier les extensions d'un répertoire")]
struct Cli {
    #[arg(short, long, default_value = ".")]
    root: String,

    #[arg(long, default_value = "reports/atlasx-report.json")]
    json: String,

    #[arg(long, default_value = "reports/atlasx-report.md")]
    markdown: String,

    #[arg(long, default_value = "reports/templates")]
    template_dir: String,

    #[arg(long, default_value = "reports/atlasx-report.html")]
    html: String,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let report = scanner::scan_directory(&cli.root);

    report::write_json(&report, &cli.json)?;
    report::write_markdown(&report, &cli.markdown)?;
    report::write_html_from_template(&report, &cli.template_dir, &cli.html)?;

    println!("✅ JSON : {}", cli.json);
    println!("✅ Markdown : {}", cli.markdown);
    println!("✅ HTML : {}", cli.html);

    Ok(())
}
