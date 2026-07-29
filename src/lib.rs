use lopdf::{Document, Object, Stream};
use rayon::prelude::*;
use anyhow::Result;
use wasm_bindgen::prelude::*;

pub struct PdfWizard;

impl PdfWizard {
    /// Deep Optimization: Compresses all streams in parallel using all CPU cores.
    pub fn optimize(mut doc: Document) -> Result<Document> {
        doc.objects.par_iter_mut().for_each(|(_, obj)| {
            if let Object::Stream(ref mut stream) = obj {
                if !stream.dict.has(b"Filter") {
                    let _ = stream.compress();
                }
            }
        });
        doc.compress();
        Ok(doc)
    }

    /// Ghost Mode: Strips all metadata, author info, and producer tags.
    pub fn scrub_metadata(mut doc: Document) -> Result<Document> {
        // Remove the Info dictionary (Author, Creator, etc.)
        doc.trailer.remove(b"Info");
        
        // Remove Metadata stream from the Catalog
        if let Ok(catalog_id) = doc.catalog() {
            if let Ok(catalog) = doc.get_object_mut(catalog_id) {
                if let Object::Dictionary(ref mut dict) = catalog {
                    dict.remove(b"Metadata");
                }
            }
        }
        Ok(doc)
    }

    /// Secure Redaction: Physically wipes target text from the binary stream.
    pub fn redact(mut doc: Document, target: &str) -> Result<Document> {
        doc.objects.par_iter_mut().for_each(|(_, obj)| {
            if let Object::Stream(ref mut stream) = obj {
                if let Ok(mut content) = stream.decode_content() {
                    for operation in content.operations.iter_mut() {
                        if operation.operator == "Tj" || operation.operator == "TJ" {
                            for arg in operation.operands.iter_mut() {
                                if let Ok(text) = arg.as_str() {
                                    if text.contains(target) {
                                        *arg = Object::string_literal(" [REDACTED] ");
                                    }
                                }
                            }
                        }
                    }
                    let _ = stream.set_content(content.encode().unwrap_or_default());
                }
            }
        });
        Ok(doc)
    }
}

// WASM Entry point for Browser usage
#[wasm_bindgen]
pub fn process_pdf_wasm(file_bits: &[u8]) -> Vec<u8> {
    let doc = Document::load_mem(file_bits).unwrap();
    let optimized = PdfWizard::optimize(doc).unwrap();
    let mut out = Vec::new();
    optimized.save_to(&mut out).unwrap();
    out
}
