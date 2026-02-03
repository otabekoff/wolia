# Wolia

A modern, GPU-accelerated office suite built in Rust.

## 🚀 Overview

Wolia is a platform for building productivity applications with a shared engine:

| App             | Description    | Status            |
| --------------- | -------------- | ----------------- |
| **Wolia Write** | Word processor | 🚧 In Development |
| **Wolia Grid**  | Spreadsheet    | 🚧 In Development |
| **Wolia Deck**  | Presentations  | 🚧 In Development |

## 🏗️ Architecture

```
wolia/
├── apps/                    # End-user applications
│   ├── wolia-write/        # Word processor
│   ├── wolia-grid/         # Spreadsheet
│   └── wolia-deck/         # Presentations
├── engine/                  # Wolia Engine (shared core)
│   ├── core/               # Document model
│   ├── layout/             # Pagination & text wrapping
│   ├── render/             # GPU rendering
│   ├── edit/               # Cursor, undo, IME
│   ├── format/             # File format handling
│   ├── platform/           # OS integration
│   ├── math/               # Geometry utilities
│   ├── assets/             # Font & image loading
│   └── plugin/             # Plugin system
├── engine-modules/         # App-specific engines
│   ├── grid-engine/        # Cells, formulas
│   └── deck-engine/        # Slides, animations
├── formats/                # File format implementations
│   ├── wolia-format/       # Native .wolia
│   ├── docx/               # Microsoft Word
│   ├── xlsx/               # Microsoft Excel
│   ├── pptx/               # Microsoft PowerPoint
│   ├── pdf/                # PDF export
│   └── markdown/           # Markdown import/export
└── plugins/                # Official plugins
    ├── latex/              # Math equations
    ├── diagrams/           # Flowcharts, UML
    └── code-blocks/        # Syntax highlighting
```

## 🔧 Building

### Prerequisites

- Rust 1.85+ (2024 edition)
- System dependencies:
  - **Linux**: `libxkbcommon-dev`, `libwayland-dev`
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Visual Studio Build Tools

### Build Commands

```bash
# Build all apps
cargo build

# Build specific app
cargo build -p wolia-write
cargo build -p wolia-grid
cargo build -p wolia-deck

# Release build
cargo build --release
```

### Run

```bash
# Run Wolia Write
cargo run -p wolia-write

# Run Wolia Grid
cargo run -p wolia-grid

# Run Wolia Deck
cargo run -p wolia-deck
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p wolia-core
cargo test -p grid-engine

# Run with output
cargo test -- --nocapture
```

## 📐 Code Style

```bash
# Format code
cargo fmt

# Lint
cargo clippy
```

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.
