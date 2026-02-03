# Lucide SVG Icons

This directory contains all 1,669 Lucide SVG icons used throughout the Wolia office suite.

## Overview

- **Source**: [lucide-icons/lucide](https://github.com/lucide-icons/lucide)
- **License**: ISC License (permissive, free for commercial and personal use)
- **Icons Count**: 1,669 SVG icons
- **Format**: Each icon has a `.svg` file and corresponding `.json` metadata file

## Directory Structure

```
icons/
├── check.svg                    # Scalable vector icon
├── check.json                   # Icon metadata
├── arrow-down.svg
├── arrow-down.json
├── ... (1,667 more icon pairs)
```

## File Naming Convention

All icons follow kebab-case naming convention:

- `check`
- `arrow-down`
- `arrow-up-right`
- `file-text`
- `layout-grid`

## Usage

### In Rust Code

```rust
use wolia_assets::IconManager;

let manager = IconManager::new();

// Get a specific icon
if let Some(svg) = manager.get("check") {
    println!("Icon: {}", svg);
}

// Search for icons
let arrows = manager.search("arrow");

// List all icons
let all = manager.list_all();
```

### Finding Icons

Visit the official icon gallery to explore all available icons:

- **Web**: https://lucide.dev/icons/

### Icon Categories

Icons are organized by functionality:

#### Common UI

- `check`, `x`, `plus`, `minus`, `equal`
- `arrow-up`, `arrow-down`, `arrow-left`, `arrow-right`
- `menu`, `settings`, `search`

#### Text Formatting

- `bold`, `italic`, `underline`, `strikethrough`
- `align-left`, `align-center`, `align-right`
- `list`, `list-ordered`

#### Document

- `file`, `file-text`, `save`, `print`
- `download`, `upload`, `copy`

#### Data

- `table`, `columns`, `rows`
- `filter`, `sort-ascending`, `sort-descending`

#### Media

- `play`, `pause`, `volume`, `volume-mute`
- `image`, `camera`, `video`

## Integration Notes

### Performance

- Icons are lazy-loaded and cached by `IconManager`
- Minimal memory footprint with just-in-time loading

### Styling

When rendering icons, consider:

- **Size**: Use 16px, 24px, 32px, or 48px sizes
- **Color**: Typically `currentColor` for theme integration
- **Stroke Width**: Icons are designed for 2px strokes

### Accessibility

- Always pair icons with text labels for important UI elements
- Use `aria-hidden="true"` for purely decorative icons
- Ensure sufficient contrast ratio (WCAG AA minimum)

## Updating Icons

To update icons from the upstream Lucide repository:

```bash
# From the project root
rm -rf engine/assets/icons/*
git clone --depth 1 --filter=blob:none --sparse https://github.com/lucide-icons/lucide.git /tmp/lucide-update
cd /tmp/lucide-update
git sparse-checkout set icons
cp -r icons/* ../../engine/assets/icons/
cd ../..
rm -rf /tmp/lucide-update
```

## References

- **Official Website**: https://lucide.dev/
- **GitHub Repository**: https://github.com/lucide-icons/lucide
- **Contributing Guide**: https://github.com/lucide-icons/lucide/blob/main/CONTRIBUTING.md
- **License**: https://github.com/lucide-icons/lucide/blob/main/LICENSE

## License

All icons in this directory are licensed under the **ISC License**, which is permissive and allows:

- ✅ Commercial use
- ✅ Personal use
- ✅ Modification
- ✅ Distribution

Attribution is appreciated but not required.

---

Last updated: February 4, 2026
Icons source: lucide-icons/lucide main branch
