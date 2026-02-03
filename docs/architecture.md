# Wolia Architecture

## Overview

Wolia is built on a shared engine architecture where three applications—**Wolia Write**, **Wolia Grid**, and **Wolia Deck**—share a common rendering engine, document model, and file format. This is the same architecture used by Microsoft Office and Apple iWork.

```
┌─────────────────────────────────────────────────────────────────┐
│                        Applications                              │
├─────────────────┬─────────────────┬─────────────────────────────┤
│  Wolia Write    │   Wolia Grid    │       Wolia Deck            │
│  (Documents)    │  (Spreadsheets) │    (Presentations)          │
├─────────────────┴─────────────────┴─────────────────────────────┤
│                     Engine Modules                               │
├─────────────────────────────────┬───────────────────────────────┤
│          Grid Engine            │        Deck Engine            │
│    (Cells, Formulas, Sheets)    │   (Slides, Animations)        │
├─────────────────────────────────┴───────────────────────────────┤
│                        Wolia Engine                              │
├─────────┬─────────┬─────────┬─────────┬─────────┬───────────────┤
│  Core   │ Layout  │ Render  │  Edit   │ Platform│    Assets     │
│ (Docs)  │ (Text)  │ (GPU)   │ (Undo)  │  (OS)   │   (Fonts)     │
├─────────┴─────────┴─────────┴─────────┴─────────┴───────────────┤
│                       Format Layer                               │
├─────────┬─────────┬─────────┬─────────┬─────────┬───────────────┤
│ .wolia  │  .docx  │  .xlsx  │  .pptx  │  .pdf   │  .md          │
└─────────┴─────────┴─────────┴─────────┴─────────┴───────────────┘
```

## Core Principles

### 1. Shared Engine

All three applications use the same:

- **Document Model** (wolia-core): Nodes, text, styles
- **Layout Engine** (wolia-layout): Text wrapping, pagination
- **Render Pipeline** (wolia-render): GPU-accelerated rendering via wgpu
- **Edit System** (wolia-edit): Cursor, selection, undo/redo
- **Platform Abstraction** (wolia-platform): Window management, events

### 2. App-Specific Engine Modules

Each application has specialized functionality:

- **Grid Engine**: Cell values, formulas, sheet management
- **Deck Engine**: Slides, shapes, animations, transitions

### 3. Single File Format

The native `.wolia` format is a container that supports:

- Documents (.wolia with document payload)
- Spreadsheets (.wolia with spreadsheet payload)
- Presentations (.wolia with presentation payload)

### 4. Interoperability

All formats can be imported/exported:

| App   | Import           | Export            |
| ----- | ---------------- | ----------------- |
| Write | .docx, .md, .txt | .docx, .pdf, .md  |
| Grid  | .xlsx, .csv      | .xlsx, .pdf, .csv |
| Deck  | .pptx            | .pptx, .pdf       |

## Crate Dependency Graph

```
wolia-write ─┬─► wolia-core ─────► wolia-math
             │
             ├─► wolia-layout ───► wolia-core
             │
             ├─► wolia-render ───► wolia-core
             │                      wolia-math
             │
             ├─► wolia-edit ─────► wolia-core
             │
             ├─► wolia-assets ───► fontdb
             │                      image
             │
             └─► wolia-platform ─► winit
                                   raw-window-handle

wolia-grid ──┬─► grid-engine ────► wolia-core
             │
             └─► (same engine crates as above)

wolia-deck ──┬─► deck-engine ────► wolia-core
             │
             └─► (same engine crates as above)
```

## Data Flow

### Document Editing

```
User Input
    │
    ▼
wolia-platform (capture events)
    │
    ▼
wolia-edit (update document model)
    │
    ▼
wolia-core (modify document tree)
    │
    ▼
wolia-layout (recalculate layout)
    │
    ▼
wolia-render (draw to GPU)
    │
    ▼
Screen
```

### File I/O

```
.docx file
    │
    ▼
format-docx (parse XML/ZIP)
    │
    ▼
wolia-core (Document struct)
    │
    ▼
format-docx (serialize XML/ZIP)
    │
    ▼
.docx file
```

## Plugin Architecture

Plugins extend functionality without modifying core code:

```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self, context: &PluginContext) -> Result<(), PluginError>;
    fn shutdown(&mut self);
}
```

Plugins can:

- Register new node types
- Add toolbar items
- Handle custom file formats
- Provide rendering extensions
