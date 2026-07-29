use clap::Parser;
use std::fs;
use anyhow::Result;
use the_pdf_wizard::PdfWizard;
use lopdf::Document;

#[derive(Parser, Debug)]
#[command(name = "pdf-wizard", about = "High-performance PDF Toolkit", version)]
struct Args {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: String,
    #[arg(short, long)]
    optimize: bool,
    #[arg(short, long)]
    scrub: bool,
    #[arg(short, long)]
    redact: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("🧙‍♂️ the-pdf-wizard is casting spells...");
    let mut doc = Document::load(&args.input)?;

    if args.optimize { doc = PdfWizard::optimize(doc)?; }
    if args.scrub { doc = PdfWizard::scrub_metadata(doc)?; }
    if let Some(target) = args.redact { doc = PdfWizard::redact(doc, &target)?; }

    doc.save(&args.output)?;
    println!("✨ Success! Saved to {}", args.output);
    Ok(())
}
