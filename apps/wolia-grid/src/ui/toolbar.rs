//! Grid toolbar.

/// Grid toolbar actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridAction {
    // Format
    Bold,
    Italic,
    Underline,

    // Alignment
    AlignLeft,
    AlignCenter,
    AlignRight,

    // Data
    Sort,
    Filter,

    // Insert
    InsertRow,
    InsertColumn,
    InsertChart,
    InsertFunction,

    // Format
    FormatCells,
    ConditionalFormat,

    // Data validation
    DataValidation,
}
