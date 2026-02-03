//! Formula evaluation utilities and built-in functions.

use crate::cell::CellValue;

/// Built-in formula functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Function {
    // Math functions
    Sum,
    Average,
    Count,
    CountA,
    Max,
    Min,
    Abs,
    Round,
    Floor,
    Ceil,
    Sqrt,
    Power,

    // Logical functions
    If,
    And,
    Or,
    Not,
    True_,
    False_,

    // Text functions
    Concatenate,
    Len,
    Upper,
    Lower,
    Trim,
    Left,
    Right,
    Mid,
    Find,
    Substitute,
    Char,
    Code,

    // Date functions
    Today,
    Now,
}

impl Function {
    /// Parse function name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_uppercase().as_str() {
            "SUM" => Some(Self::Sum),
            "AVERAGE" | "AVG" => Some(Self::Average),
            "COUNT" => Some(Self::Count),
            "COUNTA" => Some(Self::CountA),
            "MAX" => Some(Self::Max),
            "MIN" => Some(Self::Min),
            "ABS" => Some(Self::Abs),
            "ROUND" => Some(Self::Round),
            "FLOOR" => Some(Self::Floor),
            "CEIL" | "CEILING" => Some(Self::Ceil),
            "SQRT" => Some(Self::Sqrt),
            "POWER" | "POW" => Some(Self::Power),
            "IF" => Some(Self::If),
            "AND" => Some(Self::And),
            "OR" => Some(Self::Or),
            "NOT" => Some(Self::Not),
            "TRUE" => Some(Self::True_),
            "FALSE" => Some(Self::False_),
            "CONCATENATE" | "CONCAT" => Some(Self::Concatenate),
            "LEN" | "LENGTH" => Some(Self::Len),
            "UPPER" => Some(Self::Upper),
            "LOWER" => Some(Self::Lower),
            "TRIM" => Some(Self::Trim),
            "LEFT" => Some(Self::Left),
            "RIGHT" => Some(Self::Right),
            "MID" => Some(Self::Mid),
            "FIND" | "SEARCH" => Some(Self::Find),
            "SUBSTITUTE" | "REPLACE" => Some(Self::Substitute),
            "CHAR" => Some(Self::Char),
            "CODE" => Some(Self::Code),
            "TODAY" => Some(Self::Today),
            "NOW" => Some(Self::Now),
            _ => None,
        }
    }

    /// Get the name of the function.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Sum => "SUM",
            Self::Average => "AVERAGE",
            Self::Count => "COUNT",
            Self::CountA => "COUNTA",
            Self::Max => "MAX",
            Self::Min => "MIN",
            Self::Abs => "ABS",
            Self::Round => "ROUND",
            Self::Floor => "FLOOR",
            Self::Ceil => "CEIL",
            Self::Sqrt => "SQRT",
            Self::Power => "POWER",
            Self::If => "IF",
            Self::And => "AND",
            Self::Or => "OR",
            Self::Not => "NOT",
            Self::True_ => "TRUE",
            Self::False_ => "FALSE",
            Self::Concatenate => "CONCATENATE",
            Self::Len => "LEN",
            Self::Upper => "UPPER",
            Self::Lower => "LOWER",
            Self::Trim => "TRIM",
            Self::Left => "LEFT",
            Self::Right => "RIGHT",
            Self::Mid => "MID",
            Self::Find => "FIND",
            Self::Substitute => "SUBSTITUTE",
            Self::Char => "CHAR",
            Self::Code => "CODE",
            Self::Today => "TODAY",
            Self::Now => "NOW",
        }
    }
}

/// Evaluator for simple formulas.
pub struct Evaluator;

impl Evaluator {
    /// Evaluate SUM function.
    pub fn sum(values: Vec<CellValue>) -> CellValue {
        let total: f64 = values.iter().filter_map(|v| v.as_number()).sum();
        CellValue::Number(total)
    }

    /// Evaluate AVERAGE function.
    pub fn average(values: Vec<CellValue>) -> CellValue {
        let numbers: Vec<f64> = values.iter().filter_map(|v| v.as_number()).collect();
        if numbers.is_empty() {
            return CellValue::Number(0.0);
        }
        let sum: f64 = numbers.iter().sum();
        CellValue::Number(sum / numbers.len() as f64)
    }

    /// Evaluate COUNT function.
    pub fn count(values: Vec<CellValue>) -> CellValue {
        let count = values
            .iter()
            .filter(|v| matches!(v, CellValue::Number(_)))
            .count();
        CellValue::Number(count as f64)
    }

    /// Evaluate COUNTA function.
    pub fn counta(values: Vec<CellValue>) -> CellValue {
        let count = values
            .iter()
            .filter(|v| !matches!(v, CellValue::Empty))
            .count();
        CellValue::Number(count as f64)
    }

    /// Evaluate MAX function.
    pub fn max(values: Vec<CellValue>) -> CellValue {
        let max = values
            .iter()
            .filter_map(|v| v.as_number())
            .fold(f64::NEG_INFINITY, f64::max);
        if max.is_infinite() {
            CellValue::Error("No numeric values".to_string())
        } else {
            CellValue::Number(max)
        }
    }

    /// Evaluate MIN function.
    pub fn min(values: Vec<CellValue>) -> CellValue {
        let min = values
            .iter()
            .filter_map(|v| v.as_number())
            .fold(f64::INFINITY, f64::min);
        if min.is_infinite() {
            CellValue::Error("No numeric values".to_string())
        } else {
            CellValue::Number(min)
        }
    }

    /// Evaluate ABS function.
    pub fn abs(value: CellValue) -> CellValue {
        match value.as_number() {
            Some(n) => CellValue::Number(n.abs()),
            None => value,
        }
    }

    /// Evaluate ROUND function.
    pub fn round(value: CellValue, decimals: i32) -> CellValue {
        match value.as_number() {
            Some(n) => {
                let multiplier = 10f64.powi(decimals);
                CellValue::Number((n * multiplier).round() / multiplier)
            }
            None => value,
        }
    }

    /// Evaluate SQRT function.
    pub fn sqrt(value: CellValue) -> CellValue {
        match value.as_number() {
            Some(n) if n >= 0.0 => CellValue::Number(n.sqrt()),
            Some(_) => CellValue::Error("Negative square root".to_string()),
            None => value,
        }
    }

    /// Evaluate CONCATENATE function.
    pub fn concatenate(values: Vec<CellValue>) -> CellValue {
        let result = values
            .iter()
            .map(|v| v.to_display_string())
            .collect::<Vec<_>>()
            .join("");
        CellValue::Text(result)
    }

    /// Evaluate LEN function.
    pub fn len(value: CellValue) -> CellValue {
        CellValue::Number(value.to_display_string().len() as f64)
    }

    /// Evaluate UPPER function.
    pub fn upper(value: CellValue) -> CellValue {
        CellValue::Text(value.to_display_string().to_uppercase())
    }

    /// Evaluate LOWER function.
    pub fn lower(value: CellValue) -> CellValue {
        CellValue::Text(value.to_display_string().to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_from_name() {
        assert_eq!(Function::from_name("SUM"), Some(Function::Sum));
        assert_eq!(Function::from_name("sum"), Some(Function::Sum));
        assert_eq!(Function::from_name("AVERAGE"), Some(Function::Average));
        assert_eq!(Function::from_name("AVG"), Some(Function::Average));
    }

    #[test]
    fn test_sum_evaluation() {
        let values = vec![
            CellValue::Number(1.0),
            CellValue::Number(2.0),
            CellValue::Number(3.0),
        ];
        let result = Evaluator::sum(values);
        assert_eq!(result, CellValue::Number(6.0));
    }

    #[test]
    fn test_average_evaluation() {
        let values = vec![
            CellValue::Number(10.0),
            CellValue::Number(20.0),
            CellValue::Number(30.0),
        ];
        let result = Evaluator::average(values);
        assert_eq!(result, CellValue::Number(20.0));
    }

    #[test]
    fn test_count_evaluation() {
        let values = vec![
            CellValue::Number(1.0),
            CellValue::Text("hello".to_string()),
            CellValue::Number(3.0),
            CellValue::Empty,
        ];
        let result = Evaluator::count(values);
        assert_eq!(result, CellValue::Number(2.0));
    }

    #[test]
    fn test_counta_evaluation() {
        let values = vec![
            CellValue::Number(1.0),
            CellValue::Text("hello".to_string()),
            CellValue::Empty,
        ];
        let result = Evaluator::counta(values);
        assert_eq!(result, CellValue::Number(2.0));
    }

    #[test]
    fn test_concatenate_evaluation() {
        let values = vec![
            CellValue::Text("Hello".to_string()),
            CellValue::Text(" ".to_string()),
            CellValue::Text("World".to_string()),
        ];
        let result = Evaluator::concatenate(values);
        assert_eq!(result, CellValue::Text("Hello World".to_string()));
    }

    #[test]
    fn test_sqrt_evaluation() {
        let value = CellValue::Number(16.0);
        let result = Evaluator::sqrt(value);
        assert_eq!(result, CellValue::Number(4.0));
    }
}
