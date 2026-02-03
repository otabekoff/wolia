# Wolia MVP Roadmap

This document outlines the development plan for achieving a Minimum Viable Product (MVP) for the Wolia office suite.

## Vision

Wolia aims to be a modern, GPU-accelerated office suite that provides fast, beautiful, and intuitive productivity applications. The MVP will focus on delivering core functionality for three applications: Wolia Write (word processor), Wolia Grid (spreadsheet), and Wolia Deck (presentations).

## Current Status (February 2026)

### ✅ Completed

- Basic project structure with workspace organization
- Core engine modules (core, render, layout, math, platform, assets, plugin)
- GPU rendering infrastructure using wgpu
- Window management and event handling
- Basic UI layout for all three applications
- Desktop entry files for Linux
- Application icons
- File format structure (wolia-format, docx, xlsx, pptx, pdf, markdown)
- **Lucide SVG icons integration** (1,669 icons available)

### 🚧 In Progress

- **Input System** (✅ Complete)
  - Keyboard input handling ✅
  - Mouse/touch input handling ✅
  - IME (Input Method Editor) support ✅
  - Clipboard integration (copy/paste) ✅
  - Undo/redo framework ✅
- **Text Rendering System** (In Progress)

## Phase 1: Foundation (Weeks 1-4)

### Core Engine Enhancements

- [x] **Text Rendering System** (Week 1-2) ✅ Complete
  - Implement text layout engine using cosmic-text ✅
  - Font loading and management with fontdb ✅
  - Text shaping with rustybuzz ✅
  - Basic text rendering to GPU ✅
  - Unicode support and bidirectional text ✅

- [x] **Input System** (Week 2) ✅ Complete
  - Keyboard input handling ✅
  - Mouse/touch input handling ✅
  - IME (Input Method Editor) support ✅
  - Clipboard integration (copy/paste) ✅
  - Undo/redo framework ✅

- [x] **Document Model** (Week 2-3)
  - Complete core document structure ✅
  - Text storage with efficient editing ✅
  - Style system (fonts, colors, sizes) ✅
  - Paragraph formatting ✅
  - Basic change tracking for undo/redo ✅

- [x] **Asset Management** (Week 3) ✅ COMPLETE
  - Font loading and caching ✅
  - Image loading and decoding ✅
  - Resource management with LRU eviction ✅
  - Asset pipeline integration ✅
  - Generic AssetCache with reference counting ✅
  - 18 comprehensive tests - all passing ✅

## Phase 2: Wolia Write MVP (Weeks 5-8)

### Essential Features

- [x] **Text Editor** (Week 5-6) ✅ Complete (Phase 1)
  - Cursor movement and selection ✅
  - Basic text input and editing ✅
  - Cut/copy/paste ✅ (IME support)
  - Undo/redo ✅
  - Find and replace (ready for implementation)

- [x] **Text Formatting** (Week 6) ✅ Complete
  - Bold, italic, underline ✅
  - Font family and size selection ✅
  - Text color with RGB/RGBA support ✅
  - Text alignment (left, center, right, justify) ✅
  - Line spacing ✅
  - Strikethrough and background color ✅
  - 8 tests passing

- [x] **Paragraph Formatting** (Week 7) ✅ Complete
  - Paragraph spacing ✅
  - Indentation (left/right/first-line) ✅
  - Bulleted and numbered lists ✅
  - Headings (H1-H6) ✅
  - List styles (Bullet, Numbered, Lettered, Roman) ✅
  - 10 tests passing

- [x] **Document Management** (Week 7-8) ✅ Complete
  - New document ✅
  - Open existing document (with file handling) ✅
  - Save document (with path management) ✅
  - Recent files list (last 10 files tracked) ✅
  - Unsaved changes detection ✅
  - File statistics (word/char/page count) ✅
  - 7 tests passing

- [x] **UI Polish** (Week 8) ✅ Complete
  - Toolbar with formatting buttons ✅ (32 buttons organized by category)
  - Sidebar with document outline ✅ (nested heading navigation)
  - Status bar with word count and page info ✅ (live statistics)
  - Keyboard shortcuts ✅ (defined in toolbar)
  - Context menus (prepared for implementation)
  - 4 PDF export tests passing

## Phase 2.5: Export Features (Post-MVP Polish)

- [x] **PDF Export** ✅ Complete
  - Basic PDF generation ✅
  - PDF structure (catalog, pages, content streams) ✅
  - Export to file ✅
  - 4 tests passing

## Phase 3: Wolia Grid MVP (Weeks 9-12)

### Essential Features

- [ ] **Cell System** (Week 9-10)
  - Cell selection and navigation
  - Cell editing (inline and formula bar)
  - Text, number, and formula input
  - Cell formatting (alignment, number formats)
  - Cut/copy/paste cells

- [ ] **Formula Engine** (Week 10-11)
  - Basic arithmetic operators (+, -, \*, /)
  - Cell references (A1, B2, etc.)
  - Range references (A1:B10)
  - Basic functions:
    - Math: SUM, AVERAGE, MIN, MAX, COUNT
    - Logical: IF, AND, OR
    - Text: CONCATENATE, LEFT, RIGHT, MID
  - Formula error handling

- [ ] **Spreadsheet Features** (Week 11-12)
  - Multiple sheets support
  - Row/column insertion and deletion
  - Row/column resizing
  - Freeze panes
  - Basic charts (bar, line, pie)

- [ ] **Data Management** (Week 12)
  - Sort data
  - Filter data
  - Fill down/across
  - Auto-sum
  - Save/load .wolia spreadsheet format

## Phase 4: Wolia Deck MVP (Weeks 13-16)

### Essential Features

- [ ] **Slide Management** (Week 13-14)
  - Create/delete slides
  - Slide thumbnails panel
  - Slide navigation
  - Slide reordering (drag and drop)
  - Master slide concept

- [ ] **Content Objects** (Week 14-15)
  - Text boxes with formatting
  - Images (insert, resize, position)
  - Shapes (rectangle, circle, line, arrow)
  - Object selection and manipulation
  - Object layering (bring to front, send to back)

- [ ] **Presentation Features** (Week 15)
  - Slide layouts (title, title+content, blank)
  - Background colors and gradients
  - Basic transitions (fade, slide)
  - Presenter notes

- [ ] **Presentation Mode** (Week 16)
  - Full-screen presentation view
  - Slide navigation (arrow keys, mouse)
  - Presenter tools (pointer, notes view)
  - Export to PDF

## Phase 5: Integration and Polish (Weeks 17-20)

### Cross-App Features

- [ ] **File Format Support** (Week 17-18)
  - Import/export Microsoft Office formats:
    - .docx (basic text and formatting)
    - .xlsx (basic cells and formulas)
    - .pptx (basic slides and content)
  - PDF export for all apps
  - Markdown support for Write

- [ ] **Plugin System** (Week 18-19)
  - Plugin architecture implementation
  - LaTeX plugin for math equations
  - Code blocks plugin with syntax highlighting
  - Plugin discovery and loading

- [ ] **UI/UX Refinement** (Week 19-20)
  - Consistent design language across apps
  - Accessibility improvements
  - Keyboard navigation
  - High DPI support
  - Dark theme support

### Testing and Quality

- [ ] **Testing** (Week 19-20)
  - Unit tests for core engine
  - Integration tests for file formats
  - UI tests for critical workflows
  - Performance benchmarks
  - Memory leak detection

### Packaging and Distribution

- [ ] **Packaging** (Week 20)
  - Linux packages (.deb, .rpm, AppImage)
  - macOS application bundle
  - Windows installer
  - Installation scripts
  - Desktop integration testing

## Phase 6: Beta Testing and Refinement (Weeks 21-24)

### Beta Release

- [ ] **Beta Program** (Week 21)
  - Internal testing with team
  - Bug tracking system setup
  - Release notes template
  - Feedback collection process

- [ ] **Bug Fixes** (Week 21-23)
  - Critical bug fixes
  - Performance optimization
  - Memory usage optimization
  - Crash reporting and analysis

- [ ] **Documentation** (Week 22-23)
  - User guides for each app
  - Keyboard shortcuts reference
  - File format documentation
  - Developer documentation for plugins

- [ ] **Final Polish** (Week 24)
  - UI tweaks based on feedback
  - Performance tuning
  - Final bug fixes
  - Release preparation

## Success Criteria for MVP

### Wolia Write

- Create, edit, and save text documents
- Basic formatting (bold, italic, fonts, colors)
- Lists and headings
- Export to PDF
- Stable and responsive editing

### Wolia Grid

- Create and edit spreadsheets with multiple sheets
- Cell formatting and basic formulas
- Data sorting and filtering
- Simple charts
- Export to PDF

### Wolia Deck

- Create presentations with multiple slides
- Add text, images, and shapes
- Basic transitions
- Presentation mode
- Export to PDF

### General

- Fast startup time (< 2 seconds)
- Smooth 60 FPS UI rendering
- Low memory usage (< 500MB per app)
- Stable (no crashes in normal usage)
- Cross-platform support (Linux, macOS, Windows)

## Post-MVP Features (Future)

### Short-term (3-6 months)

- Real-time collaboration
- Cloud sync
- Mobile versions (iOS, Android)
- Advanced formatting (tables in Write, conditional formatting in Grid)
- Animation effects in Deck
- Version history
- Comments and review mode

### Long-term (6-12 months)

- AI-powered features (grammar checking, smart suggestions)
- Advanced charts and graphs
- Mail merge in Write
- Pivot tables in Grid
- Audio/video in Deck
- Web version
- API for third-party integrations

## Development Principles

1. **Performance First**: Every feature must maintain 60 FPS
2. **User-Centric**: Focus on common workflows and ease of use
3. **Cross-Platform**: Test on Linux, macOS, and Windows regularly
4. **Quality Over Features**: Better to have fewer features that work perfectly
5. **Iterative Development**: Release early, gather feedback, improve
6. **Open Source**: Maintain transparency and welcome contributions

## Release Schedule

- **Phase 1-2 Complete**: End of March 2026 (Wolia Write Alpha)
- **Phase 3 Complete**: End of April 2026 (Wolia Grid Alpha)
- **Phase 4 Complete**: End of May 2026 (Wolia Deck Alpha)
- **Phase 5 Complete**: Mid-June 2026 (Integrated Beta)
- **MVP Release**: End of June 2026 (Public Beta)
- **1.0 Release**: End of July 2026 (Stable Release)

---

_This roadmap is a living document and will be updated as development progresses. Dates are estimates and subject to change based on complexity and feedback._
