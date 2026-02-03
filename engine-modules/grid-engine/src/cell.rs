//! Cell model.

use serde::{Deserialize, Serialize};

/// A cell reference (row, column).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CellRef {
    /// Row index (0-based).
    pub row: usize,
    /// Column index (0-based).
    pub col: usize,
}

impl CellRef {
    /// Create a new cell reference.
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    /// Parse from A1 notation (e.g., "B3").
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim().to_uppercase();
        let mut chars = s.chars().peekable();

        // Parse column letters
        let mut col = 0usize;
        while let Some(&c) = chars.peek() {
            if c.is_ascii_alphabetic() {
                col = col * 26 + (c as usize - 'A' as usize + 1);
                chars.next();
            } else {
                break;
            }
        }

        if col == 0 {
            return None;
        }
        col -= 1; // Convert to 0-based

        // Parse row number
        let row_str: String = chars.collect();
        let row: usize = row_str.parse().ok()?;

        if row == 0 {
            return None;
        }

        Some(Self { row: row - 1, col })
    }

    /// Convert to A1 notation.
    pub fn to_a1(&self) -> String {
        let mut col_str = String::new();
        let mut col = self.col + 1;

        while col > 0 {
            col -= 1;
            col_str.insert(0, (b'A' + (col % 26) as u8) as char);
            col /= 26;
        }

        format!("{}{}", col_str, self.row + 1)
    }
}

/// A spreadsheet cell.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cell {
    /// The cell value.
    pub value: CellValue,
    /// Formula (if any).
    pub formula: Option<String>,
    /// Cell style.
    pub style: CellStyle,
}

impl Cell {
    /// Create an empty cell.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Create a cell with a value.
    pub fn with_value(value: CellValue) -> Self {
        Self {
            value,
            formula: None,
            style: CellStyle::default(),
        }
    }

    /// Create a cell with a formula.
    pub fn with_formula(formula: impl Into<String>) -> Self {
        Self {
            value: CellValue::Empty,
            formula: Some(formula.into()),
            style: CellStyle::default(),
        }
    }
}

/// Cell value types.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum CellValue {
    /// Empty cell.
    #[default]
    Empty,
    /// Text value.
    Text(String),
    /// Numeric value.
    Number(f64),
    /// Boolean value.
    Boolean(bool),
    /// Error value.
    Error(String),
    /// Date value (days since epoch).
    Date(i64),
}

impl CellValue {
    /// Check if the value is empty.
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Try to convert to a number.
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Self::Number(n) => Some(*n),
            Self::Boolean(b) => Some(if *b { 1.0 } else { 0.0 }),
            Self::Text(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// Convert to display string.
    pub fn to_display_string(&self) -> String {
        match self {
            Self::Empty => String::new(),
            Self::Text(s) => s.clone(),
            Self::Number(n) => format!("{}", n),
            Self::Boolean(b) => if *b { "TRUE" } else { "FALSE" }.to_string(),
            Self::Error(e) => format!("#{}!", e),
            Self::Date(d) => format!("Date({})", d), // TODO: Format properly
        }
    }
}

/// Cell styling.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CellStyle {
    /// Number format.
    pub number_format: Option<String>,
    /// Font family.
    pub font_family: Option<String>,
    /// Font size.
    pub font_size: Option<f32>,
    /// Bold.
    pub bold: Option<bool>,
    /// Italic.
    pub italic: Option<bool>,
    /// Text color.
    pub color: Option<[u8; 4]>,
    /// Background color.
    pub background: Option<[u8; 4]>,
    /// Horizontal alignment.
    pub h_align: Option<HAlign>,
    /// Vertical alignment.
    pub v_align: Option<VAlign>,
}

/// Horizontal alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HAlign {
    Left,
    Center,
    Right,
}

/// Vertical alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VAlign {
    Top,
    Middle,
    Bottom,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_ref_parse() {
        assert_eq!(CellRef::parse("A1"), Some(CellRef::new(0, 0)));
        assert_eq!(CellRef::parse("B3"), Some(CellRef::new(2, 1)));
        assert_eq!(CellRef::parse("Z10"), Some(CellRef::new(9, 25)));
        assert_eq!(CellRef::parse("AA1"), Some(CellRef::new(0, 26)));
    }

    #[test]
    fn test_cell_ref_to_a1() {
        assert_eq!(CellRef::new(0, 0).to_a1(), "A1");
        assert_eq!(CellRef::new(2, 1).to_a1(), "B3");
        assert_eq!(CellRef::new(9, 25).to_a1(), "Z10");
        assert_eq!(CellRef::new(0, 26).to_a1(), "AA1");
    }
}
