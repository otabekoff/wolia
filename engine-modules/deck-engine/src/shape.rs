//! Shape model.

use uuid::Uuid;
use wolia_core::text::Text;
use wolia_math::Rect;

/// A shape on a slide.
#[derive(Debug, Clone)]
pub struct Shape {
    /// Unique ID.
    pub id: Uuid,
    /// Shape type.
    pub kind: ShapeKind,
    /// Bounding box.
    pub bounds: Rect,
    /// Rotation in degrees.
    pub rotation: f32,
    /// Shape style.
    pub style: ShapeStyle,
    /// Is locked.
    pub locked: bool,
    /// Is hidden.
    pub hidden: bool,
}

impl Shape {
    /// Create a new shape.
    pub fn new(kind: ShapeKind, bounds: Rect) -> Self {
        Self {
            id: Uuid::new_v4(),
            kind,
            bounds,
            rotation: 0.0,
            style: ShapeStyle::default(),
            locked: false,
            hidden: false,
        }
    }

    /// Create a text box.
    pub fn text_box(bounds: Rect, text: Text) -> Self {
        Self::new(ShapeKind::TextBox(Box::new(text)), bounds)
    }

    /// Create a rectangle.
    pub fn rectangle(bounds: Rect) -> Self {
        Self::new(ShapeKind::Rectangle, bounds)
    }

    /// Create an ellipse.
    pub fn ellipse(bounds: Rect) -> Self {
        Self::new(ShapeKind::Ellipse, bounds)
    }

    /// Create an image.
    pub fn image(bounds: Rect, src: impl Into<String>) -> Self {
        Self::new(ShapeKind::Image { src: src.into() }, bounds)
    }
}

/// Shape type.
#[derive(Debug, Clone)]
pub enum ShapeKind {
    /// Text box.
    TextBox(Box<Text>),
    /// Rectangle.
    Rectangle,
    /// Rounded rectangle.
    RoundedRectangle { radius: f32 },
    /// Ellipse/circle.
    Ellipse,
    /// Triangle.
    Triangle,
    /// Line.
    Line,
    /// Arrow.
    Arrow,
    /// Image.
    Image { src: String },
    /// Video.
    Video { src: String },
    /// Table.
    Table { rows: usize, cols: usize },
    /// Chart.
    Chart { chart_type: ChartType },
    /// Custom path.
    Path { data: String },
}

/// Chart types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartType {
    Bar,
    Column,
    Line,
    Pie,
    Doughnut,
    Area,
    Scatter,
}

/// Shape styling.
#[derive(Debug, Clone, Default)]
pub struct ShapeStyle {
    /// Fill color.
    pub fill: Option<[u8; 4]>,
    /// Stroke color.
    pub stroke: Option<[u8; 4]>,
    /// Stroke width.
    pub stroke_width: f32,
    /// Shadow.
    pub shadow: Option<Shadow>,
    /// Opacity (0-1).
    pub opacity: f32,
}

/// Shadow settings.
#[derive(Debug, Clone)]
pub struct Shadow {
    /// Color.
    pub color: [u8; 4],
    /// X offset.
    pub offset_x: f32,
    /// Y offset.
    pub offset_y: f32,
    /// Blur radius.
    pub blur: f32,
}
