//! PDF generator implementation.

use crate::error::Error;
use std::io::Write;
use wolia_core::Document;

const PDF_HEADER: &[u8] = b"%PDF-1.4\n";

/// PDF object representing basic elements.
#[derive(Debug, Clone)]
struct PdfObject {
    /// Object ID.
    id: u32,
    /// Object content.
    content: String,
}

impl PdfObject {
    /// Create a new PDF object.
    fn new(id: u32, content: String) -> Self {
        Self { id, content }
    }

    /// Serialize the object for the PDF file.
    fn serialize(&self) -> String {
        format!("{} 0 obj\n{}\nendobj\n", self.id, self.content)
    }
}

/// PDF generator for Wolia documents.
pub struct PdfGenerator {
    /// PDF objects.
    objects: Vec<PdfObject>,
    /// Object offsets in the file.
    offsets: Vec<u64>,
    /// Current object ID counter.
    next_id: u32,
}

impl PdfGenerator {
    /// Create a new PDF generator.
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            offsets: Vec::new(),
            next_id: 1,
        }
    }

    /// Generate PDF from a document.
    pub fn generate(&mut self, _document: &Document) -> Result<Vec<u8>, Error> {
        // Create PDF catalog
        self.create_catalog()?;

        // Create PDF page structure
        self.create_pages()?;

        // Create a blank page
        self.create_page()?;

        // Create content stream
        self.create_content_stream()?;

        // Serialize to bytes
        self.serialize()
    }

    /// Create PDF catalog object.
    fn create_catalog(&mut self) -> Result<(), Error> {
        let id = self.next_id;
        self.next_id += 1;

        let content = format!("<< /Type /Catalog /Pages {} 0 R >>", id + 1);

        self.objects.push(PdfObject::new(id, content));
        Ok(())
    }

    /// Create PDF pages object.
    fn create_pages(&mut self) -> Result<(), Error> {
        let id = self.next_id;
        self.next_id += 1;

        let content = format!("<< /Type /Pages /Kids [{}] /Count 1 >>", id + 1);

        self.objects.push(PdfObject::new(id, content));
        Ok(())
    }

    /// Create a page object.
    fn create_page(&mut self) -> Result<(), Error> {
        let id = self.next_id;
        self.next_id += 1;

        let content = format!(
            "<<\n  /Type /Page\n  /Parent {} 0 R\n  /MediaBox [0 0 612 792]\n  /Contents {} 0 R\n>>",
            id - 1,
            id + 1
        );

        self.objects.push(PdfObject::new(id, content));
        Ok(())
    }

    /// Create a content stream with basic PDF content.
    fn create_content_stream(&mut self) -> Result<(), Error> {
        let id = self.next_id;
        self.next_id += 1;

        // Create a simple content stream with text
        let stream_content = "BT\n/F1 12 Tf\n100 700 Td\n(Wolia Document) Tj\nET\n";
        let stream_length = stream_content.len();

        let content = format!(
            "<< /Length {} >>\nstream\n{}\nendstream",
            stream_length, stream_content
        );

        self.objects.push(PdfObject::new(id, content));
        Ok(())
    }

    /// Serialize the PDF to bytes.
    fn serialize(&mut self) -> Result<Vec<u8>, Error> {
        let mut output = Vec::new();

        // Write PDF header
        output.write_all(PDF_HEADER).map_err(Error::Io)?;

        // Track offsets for xref
        self.offsets.clear();
        for obj in &self.objects {
            self.offsets.push(output.len() as u64);
            output
                .write_all(obj.serialize().as_bytes())
                .map_err(Error::Io)?;
        }

        // Write xref table
        let xref_offset = output.len();
        let xref_content = self.generate_xref()?;
        output
            .write_all(xref_content.as_bytes())
            .map_err(Error::Io)?;

        // Write trailer
        let trailer = format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n",
            self.objects.len() + 1,
            xref_offset
        );
        output.write_all(trailer.as_bytes()).map_err(Error::Io)?;

        Ok(output)
    }

    /// Generate the xref table.
    fn generate_xref(&self) -> Result<String, Error> {
        let mut xref = String::from("xref\n");
        xref.push_str(&format!("0 {}\n", self.objects.len() + 1));
        xref.push_str("0000000000 65535 f \n");

        for offset in &self.offsets {
            xref.push_str(&format!("{:010} 00000 n \n", offset));
        }

        Ok(xref)
    }
}

impl Default for PdfGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdf_object_creation() {
        let obj = PdfObject::new(1, "<< /Type /Catalog >>".to_string());
        assert_eq!(obj.id, 1);
        assert!(obj.serialize().contains("endobj"));
    }

    #[test]
    fn test_generator_creation() {
        let generator = PdfGenerator::new();
        assert_eq!(generator.next_id, 1);
        assert!(generator.objects.is_empty());
    }

    #[test]
    fn test_xref_generation() {
        let generator = PdfGenerator::new();
        let xref = generator.generate_xref().unwrap();
        assert!(xref.contains("xref"));
        assert!(xref.contains("f"));
    }
}
