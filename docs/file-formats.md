# File Formats

Wolia supports multiple file formats for import, export, and native storage.

## Native Format (.wolia)

The `.wolia` format is a ZIP archive containing:

```
document.wolia
├── manifest.json       # Format version, app type
├── content.json        # Document tree
├── styles.json         # Style definitions
├── assets/            # Embedded images, fonts
│   ├── image001.png
│   └── image002.jpg
└── metadata.json      # Author, dates, etc.
```

### Manifest

```json
{
  "version": "1.0",
  "app": "write",
  "created": "2026-01-01T00:00:00Z",
  "modified": "2026-01-15T12:30:00Z"
}
```

### Content (Documents)

```json
{
  "root": {
    "kind": "document",
    "children": [
      {
        "kind": "paragraph",
        "children": [
          {
            "kind": "text",
            "content": "Hello, World!",
            "spans": [{ "range": [0, 5], "style": "bold" }]
          }
        ]
      }
    ]
  }
}
```

---

## Microsoft Word (.docx)

### Import

The `format-docx` crate parses:

- `word/document.xml` → Document tree
- `word/styles.xml` → Style definitions
- `word/media/*` → Embedded assets

### Export

Generates valid Office Open XML:

- Paragraph formatting
- Character styles
- Tables
- Images
- Headers/Footers

### Limitations

- Complex features (comments, track changes) are preserved but not editable
- Some advanced formatting may be simplified

---

## Microsoft Excel (.xlsx)

### Import

The `format-xlsx` crate parses:

- `xl/worksheets/*.xml` → Sheet data
- `xl/sharedStrings.xml` → String table
- `xl/styles.xml` → Cell styles
- Formulas are converted to Wolia formula format

### Export

Generates valid SpreadsheetML:

- Cell values and formulas
- Number formatting
- Cell styles
- Multiple sheets

### Formula Support

| Excel                  | Wolia                  |
| ---------------------- | ---------------------- |
| `=SUM(A1:A10)`         | `=SUM(A1:A10)`         |
| `=VLOOKUP(...)`        | `=VLOOKUP(...)`        |
| `=IF(A1>0,"Yes","No")` | `=IF(A1>0,"Yes","No")` |

---

## Microsoft PowerPoint (.pptx)

### Import

The `format-pptx` crate parses:

- `ppt/slides/*.xml` → Slide content
- `ppt/slideLayouts/*.xml` → Layouts
- `ppt/slideMasters/*.xml` → Master slides
- `ppt/media/*` → Embedded media

### Export

Generates valid PresentationML:

- Shapes and text boxes
- Images
- Transitions (basic)
- Animations (basic)

---

## PDF Export

PDF export uses `pdf-writer`:

- Vector graphics
- Embedded fonts (subset)
- Images
- Links

### Features

- Page size and orientation
- Headers and footers
- Page numbers
- Table of contents (bookmarks)

---

## Markdown (.md)

### Import

Converts Markdown to document model:

```markdown
# Heading 1

This is **bold** and _italic_.

- List item 1
- List item 2
```

### Export

Converts document model to Markdown:

- Headings
- Bold, italic, code
- Lists (ordered and unordered)
- Links and images
- Code blocks

---

## CSV (.csv)

For spreadsheets:

### Import

- Detects delimiters (comma, semicolon, tab)
- Handles quoted strings
- Converts to sheet data

### Export

- Standard CSV with quoted strings
- Configurable delimiter
