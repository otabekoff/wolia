# Wolia Engine Documentation

The Wolia Engine is the shared foundation for all Wolia applications.

## Crates Overview

| Crate            | Purpose                             |
| ---------------- | ----------------------------------- |
| `wolia-core`     | Document model, text, styles        |
| `wolia-math`     | Geometry types (Rect, Point, Color) |
| `wolia-layout`   | Text wrapping, pagination           |
| `wolia-render`   | GPU rendering via wgpu              |
| `wolia-edit`     | Cursor, selection, undo/redo        |
| `wolia-format`   | File format traits                  |
| `wolia-platform` | Window management, OS APIs          |
| `wolia-assets`   | Font and image loading              |
| `wolia-plugin`   | Plugin system                       |

---

## wolia-core

The document model is a tree of nodes:

```rust
pub struct Document {
    pub root: Node,
    pub styles: StyleSheet,
    pub metadata: Metadata,
}

pub struct Node {
    pub id: NodeId,
    pub kind: NodeKind,
    pub children: Vec<Node>,
}

pub enum NodeKind {
    Document,
    Paragraph,
    Text(Text),
    Image(Image),
    Table(Table),
    // ...
}
```

### Text

Text is represented as spans with styles:

```rust
pub struct Text {
    pub content: String,
    pub spans: Vec<Span>,
}

pub struct Span {
    pub range: Range<usize>,
    pub style: StyleId,
}
```

### Styles

Styles are stored in a stylesheet and referenced by ID:

```rust
pub struct Style {
    pub font_family: Option<String>,
    pub font_size: Option<f32>,
    pub font_weight: Option<FontWeight>,
    pub color: Option<Color>,
    // ...
}
```

---

## wolia-math

Geometry utilities:

```rust
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Size {
    pub width: f32,
    pub height: f32,
}

pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
```

---

## wolia-layout

The layout engine computes positions for all nodes:

```rust
pub struct LayoutEngine {
    font_manager: FontManager,
}

impl LayoutEngine {
    pub fn layout(&self, document: &Document, constraints: Size) -> LayoutTree;
}

pub struct LayoutTree {
    pub pages: Vec<Page>,
}

pub struct Page {
    pub lines: Vec<Line>,
    pub size: Size,
}
```

### Line Breaking

Uses the Unicode Line Breaking Algorithm with optional hyphenation.

### Pagination

Documents are split into pages based on page size and margins.

---

## wolia-render

GPU rendering via wgpu:

```rust
pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    // ...
}

impl Renderer {
    pub async fn new(window: &Window) -> Self;
    pub fn render(&mut self, layout: &LayoutTree);
}
```

### Text Rendering

Uses `cosmic-text` for shaping and `wgpu` for rasterization.

### Texture Atlas

Glyphs are cached in a texture atlas for efficient rendering.

---

## wolia-edit

Editing operations:

```rust
pub struct EditSession {
    pub document: Document,
    pub cursor: Cursor,
    pub selection: Option<Selection>,
    pub history: History,
}

impl EditSession {
    pub fn insert(&mut self, text: &str);
    pub fn delete(&mut self);
    pub fn undo(&mut self);
    pub fn redo(&mut self);
}
```

### Undo/Redo

Uses an operation-based history:

```rust
pub enum Operation {
    Insert { position: usize, text: String },
    Delete { position: usize, text: String },
    // ...
}
```

---

## wolia-platform

OS abstraction:

```rust
pub struct Window {
    handle: winit::window::Window,
}

pub enum Event {
    KeyDown(KeyCode),
    KeyUp(KeyCode),
    MouseDown(Point),
    MouseUp(Point),
    MouseMove(Point),
    // ...
}
```

---

## wolia-assets

Asset loading:

```rust
pub struct FontManager {
    db: fontdb::Database,
}

impl FontManager {
    pub fn load_font(&mut self, path: &Path) -> FontId;
    pub fn query(&self, family: &str, weight: FontWeight) -> Option<FontId>;
}
```

---

## wolia-plugin

Plugin system:

```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self, context: &PluginContext) -> Result<(), PluginError>;
    fn shutdown(&mut self);
}
```
