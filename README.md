#  the-pdf-wizard

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![GitHub Repo](https://img.shields.io/badge/Source-GitHub-black?logo=github)](https://github.com/brolookslikeanfish67-hub/the-pdf-wizard)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT))
[![WASM](https://img.shields.io/badge/Runtime-WebAssembly-blueviolet.svg)](#wasm-setup)
[![CLI](https://img.shields.io/badge/Tool-CLI-green.svg)](#cli-setup)

**A high-performance Rust library and CLI tool for parsing, editing, and manipulating PDF files.**  
Built with `lopdf`, `rayon`, and `wasm-bindgen` for zero-copy, multi-core, privacy-first PDF engineering.

>  **Live Project:** [https://github.com/brolookslikeanfish67-hub/the-pdf-wizard](https://github.com/brolookslikeanfish67-hub/the-pdf-wizard)

---

##  Why this exists

Legacy PDF editors are built on 30-year-old C++ architectures. They load entire files into RAM, crash on "binary-bomb" PDFs, and hide your data behind black-box cloud servers.

**the-pdf-wizard** solves this with:
- **Memory-mapped, parallel stream processing** (Rust + Rayon)
- **Physical Secure Redaction** (bytes are overwritten, not just covered)
- **Ghost Mode** (metadata, author history, and GPS tags are stripped)
- **Zero-Knowledge WASM Core** (the engine runs in your browser; nothing leaves your machine)

---

## 🌟 Features

| Feature | Description |
|---------|-------------|
|  **Parallel Optimization** | Multi-threaded `FlateDecode` compression via Rayon |
|  **Secure Redaction** | Physically wipes target text from the binary stream |
|  **Ghost Mode** | Strips `Info` dictionary, `Metadata`, Producer, and Author tags |
|  **WASM Native** | Compile the Rust core to WebAssembly for instant browser editing |
|  **Memory Safe** | 100% Rust—no buffer overflows, no segfaults, no "binary bombs" |
|  **Incremental Updates** | Uses `doc.compress()` to rebuild XRef tables for instant loading |

---

##  Setup Guide

### Prerequisites

| Tool | Version | Install Command |
|------|---------|-----------------|
| Rust | 1.70+ | [rustup.rs](https://rustup.rs) |
| `wasm-pack` | latest | `cargo install wasm-pack` |
| Python (optional) | 3.x | `python -m venv` (for local server) |

---

### 1. Clone the Repository

```bash
git clone https://github.com/brolookslikeanfish67-hub/the-pdf-wizard.git
cd the-pdf-wizard
