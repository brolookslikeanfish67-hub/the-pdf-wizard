use clap::Parser;
use std::fs;
use anyhow::Result;
use the_pdf_wizard::PdfWizard;
use lopdf::Document;

#[derive(Parser, Debug)]
#[command(name = "the-pdf-wizard", about = "High-performance PDF Toolkit", version)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,

    /// Compress and optimize the PDF
    #[arg(short, long)]
    optimize: bool,

    /// Scrub all metadata (Ghost Mode)
    #[arg(short, long)]
    scrub: bool,

    /// Text to redact
    #[arg(short, long)]
    redact: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!(" the-pdf-wizard is working...");

    let mut doc = Document::load(&args.input)?;

    if args.optimize {
        println!("⚡ Optimizing streams...");
        doc = PdfWizard::optimize(doc)?;
    }

    if args.scrub {
        println!(" Scrubbing metadata...");
        doc = PdfWizard::scrub_metadata(doc)?;
    }

    if let Some(target) = args.redact {
        println!(" Redacting: {}...", target);
        doc = PdfWizard::redact(doc, &target)?;
    }

    doc.save(&args.output)?;
    println!(" Success! Saved to {}", args.output);
    Ok(())
}
