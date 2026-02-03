# Lucide SVG Icons in Wolia

This project uses **Lucide SVG icons**, a beautiful, consistent icon toolkit with 1600+ icons. All icons are available in the `engine/assets/icons/` directory.

## Overview

- **Total Icons**: 1,669 SVG icons
- **Icon Format**: SVG files with corresponding JSON metadata
- **License**: ISC License (free for commercial and personal use)
- **Source**: [lucide-icons/lucide](https://github.com/lucide-icons/lucide)

## Icon Directory Structure

```
engine/assets/icons/
├── check.svg                    # Icon files (SVG format)
├── check.json                   # Icon metadata
├── arrow-down.svg
├── arrow-down.json
├── ... (1,667 more icon pairs)
```

Each icon has:

- **`.svg` file**: The actual scalable vector icon
- **`.json` file**: Metadata including icon name and components

## Using Icons in Rust Code

### Basic Usage

```rust
use wolia_assets::IconManager;

fn main() {
    let icon_manager = IconManager::new();

    // Get a specific icon
    if let Some(svg) = icon_manager.get("check") {
        println!("Check icon: {}", svg);
    }

    // Search for icons
    let arrows = icon_manager.search("arrow");
    println!("Found arrow icons: {:?}", arrows);

    // List all available icons
    let all_icons = icon_manager.list_all();
    println!("Total icons: {}", icon_manager.count());
}
```

### Icon Manager API

The `IconManager` provides several methods for working with icons:

| Method                 | Purpose                       | Example                                         |
| ---------------------- | ----------------------------- | ----------------------------------------------- |
| `new()`                | Create a new icon manager     | `IconManager::new()`                            |
| `get(name)`            | Get an SVG icon by name       | `manager.get("check")`                          |
| `list_all()`           | Get all available icon names  | `manager.list_all()`                            |
| `count()`              | Get the total number of icons | `manager.count()`                               |
| `search(pattern)`      | Search icons by name pattern  | `manager.search("arrow")`                       |
| `load_from_file(path)` | Load an icon from file        | `manager.load_from_file(Path::new("icon.svg"))` |

## Popular Icons

Here are some commonly used icons available:

### UI Controls

- `check`, `x`, `plus`, `minus`, `equal`
- `arrow-up`, `arrow-down`, `arrow-left`, `arrow-right`
- `chevron-up`, `chevron-down`, `chevron-left`, `chevron-right`
- `menu`, `settings`, `search`, `filter`

### Text Editing

- `bold`, `italic`, `underline`, `strikethrough`
- `align-left`, `align-center`, `align-right`, `align-justify`
- `list`, `list-ordered`
- `indent-decrease`, `indent-increase`

### Document

- `file`, `file-text`, `file-pdf`, `save`
- `document`, `folder`, `archive`
- `print`, `download`, `upload`

### Data/Tables

- `table`, `columns`, `rows`
- `filter`, `sort-ascending`, `sort-descending`
- `eye`, `eye-off`

### Presentation/Slides

- `play`, `pause`, `skip-back`, `skip-forward`
- `volume`, `volume-x`, `volume-mute`
- `maximize`, `minimize`

## Integration in UI Components

For UI applications (wolia-write, wolia-grid, wolia-deck), icons are referenced by name:

```rust
// In a toolbar component
let icon_names = vec![
    "file-plus",      // New document
    "save",            // Save
    "undo",            // Undo
    "redo",            // Redo
    "bold",            // Bold
    "italic",          // Italic
];
```

## Icon Development

### Adding New Icon Usage

When adding a new icon to the UI:

1. **Search for the icon**: Use the search method or visit [lucide.dev/icons](https://lucide.dev/icons/)
2. **Reference by name**: Use the icon filename without the `.svg` extension
3. **Verify rendering**: Test the icon in context

### Finding Icons

Visit the official Lucide website for a searchable database:

- **Main Site**: https://lucide.dev/icons/
- **GitHub Repository**: https://github.com/lucide-icons/lucide

### Icon Naming Convention

Icons follow kebab-case naming:

- `check-circle`
- `arrow-up-right`
- `file-text`
- `layout-grid`

## Styling Icons

When rendering SVG icons, common styling options include:

```rust
// Size variations
width: "16px"   // Small
width: "24px"   // Medium (standard)
width: "32px"   // Large
width: "48px"   // Extra large

// Colors
fill: "currentColor"  // Inherit text color
stroke: "#000000"     // Specific color
opacity: 0.5          // Transparency
```

## License

All Lucide icons are licensed under the **ISC License**, which is permissive and allows both commercial and personal use without attribution required (though attribution is appreciated).

See [LICENSE](https://github.com/lucide-icons/lucide/blob/main/LICENSE) in the lucide repository for details.

## Resources

- **Lucide Official**: https://lucide.dev/
- **Icon Gallery**: https://lucide.dev/icons/
- **GitHub**: https://github.com/lucide-icons/lucide
- **Contributing**: https://github.com/lucide-icons/lucide/blob/main/CONTRIBUTING.md
- **Discord Community**: https://discord.gg/EH6nSts

## Example: Complete Icon Usage

```rust
use wolia_assets::IconManager;

fn render_toolbar() {
    let icon_manager = IconManager::new();

    let toolbar_icons = vec![
        ("file-plus", "New"),
        ("save", "Save"),
        ("undo", "Undo"),
        ("redo", "Redo"),
    ];

    for (icon_name, label) in toolbar_icons {
        if let Some(svg) = icon_manager.get(icon_name) {
            // Render icon with label
            println!("Icon: {} ({})", label, icon_name);
            // Further rendering logic...
        }
    }
}
```

## Tips & Best Practices

1. **Caching**: The `IconManager` automatically caches icons for performance
2. **Error Handling**: Always check `Option<String>` when getting icons
3. **Search First**: Use the search method to discover icons
4. **Consistent Sizing**: Use consistent icon sizes across your UI
5. **Accessibility**: Ensure icons have proper labels or ARIA attributes

---

For questions or issues, visit the [Lucide GitHub repository](https://github.com/lucide-icons/lucide).
