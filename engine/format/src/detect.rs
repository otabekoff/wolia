//! Format detection.

/// Detected file format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectedFormat {
    /// Native Wolia format.
    Wolia,
    /// Microsoft Word (.docx).
    Docx,
    /// Microsoft Excel (.xlsx).
    Xlsx,
    /// Microsoft PowerPoint (.pptx).
    Pptx,
    /// PDF document.
    Pdf,
    /// Markdown.
    Markdown,
    /// Plain text.
    PlainText,
    /// Unknown format.
    Unknown,
}

/// Detect the format of a file by examining its contents.
pub fn detect(data: &[u8]) -> DetectedFormat {
    // Check for Wolia format
    if data.starts_with(b"WOLIA") {
        return DetectedFormat::Wolia;
    }

    // Check for ZIP-based formats (OOXML)
    if data.starts_with(&[0x50, 0x4B, 0x03, 0x04]) {
        // This is a ZIP file, could be docx, xlsx, pptx
        // Would need to inspect contents to determine exact type
        return DetectedFormat::Docx; // Placeholder
    }

    // Check for PDF
    if data.starts_with(b"%PDF") {
        return DetectedFormat::Pdf;
    }

    // Check for UTF-8 BOM or plain text
    if data.starts_with(&[0xEF, 0xBB, 0xBF]) || data.iter().all(|&b| b < 128 || b >= 0xC0) {
        // Could be markdown or plain text
        if looks_like_markdown(data) {
            return DetectedFormat::Markdown;
        }
        return DetectedFormat::PlainText;
    }

    DetectedFormat::Unknown
}

/// Check if data looks like Markdown.
fn looks_like_markdown(data: &[u8]) -> bool {
    let text = match std::str::from_utf8(data) {
        Ok(s) => s,
        Err(_) => return false,
    };

    // Look for common Markdown patterns
    text.contains("# ") || text.contains("## ") || text.contains("```") || text.contains("[](")
}

/// Detect format from file extension.
pub fn detect_from_extension(extension: &str) -> DetectedFormat {
    match extension.to_lowercase().as_str() {
        "wolia" => DetectedFormat::Wolia,
        "docx" => DetectedFormat::Docx,
        "xlsx" => DetectedFormat::Xlsx,
        "pptx" => DetectedFormat::Pptx,
        "pdf" => DetectedFormat::Pdf,
        "md" | "markdown" => DetectedFormat::Markdown,
        "txt" => DetectedFormat::PlainText,
        _ => DetectedFormat::Unknown,
    }
}
