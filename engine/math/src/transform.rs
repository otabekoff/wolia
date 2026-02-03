//! 2D transforms.

use glam::{Affine2, Mat3, Vec2};
use serde::{Deserialize, Serialize};

/// A 2D transformation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform2D {
    /// The underlying affine transform.
    #[serde(with = "affine2_serde")]
    pub affine: Affine2,
}

impl Transform2D {
    /// Identity transform.
    pub const IDENTITY: Self = Self {
        affine: Affine2::IDENTITY,
    };

    /// Create a translation transform.
    pub fn translate(x: f32, y: f32) -> Self {
        Self {
            affine: Affine2::from_translation(Vec2::new(x, y)),
        }
    }

    /// Create a scale transform.
    pub fn scale(x: f32, y: f32) -> Self {
        Self {
            affine: Affine2::from_scale(Vec2::new(x, y)),
        }
    }

    /// Create a uniform scale transform.
    pub fn uniform_scale(s: f32) -> Self {
        Self::scale(s, s)
    }

    /// Create a rotation transform (angle in radians).
    pub fn rotate(angle: f32) -> Self {
        Self {
            affine: Affine2::from_angle(angle),
        }
    }

    /// Create a rotation transform (angle in degrees).
    pub fn rotate_degrees(degrees: f32) -> Self {
        Self::rotate(degrees.to_radians())
    }

    /// Combine with another transform (self * other).
    pub fn then(&self, other: &Transform2D) -> Self {
        Self {
            affine: self.affine * other.affine,
        }
    }

    /// Transform a point.
    pub fn transform_point(&self, point: Vec2) -> Vec2 {
        self.affine.transform_point2(point)
    }

    /// Transform a vector (ignores translation).
    pub fn transform_vector(&self, vector: Vec2) -> Vec2 {
        self.affine.transform_vector2(vector)
    }

    /// Get the inverse transform.
    pub fn inverse(&self) -> Self {
        Self {
            affine: self.affine.inverse(),
        }
    }

    /// Convert to a 3x3 matrix.
    pub fn to_mat3(&self) -> Mat3 {
        Mat3::from(self.affine)
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl std::ops::Mul for Transform2D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.then(&rhs)
    }
}

mod affine2_serde {
    use glam::Affine2;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize, Deserialize)]
    struct Affine2Repr {
        matrix: [[f32; 2]; 2],
        translation: [f32; 2],
    }

    pub fn serialize<S>(affine: &Affine2, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let repr = Affine2Repr {
            matrix: [affine.matrix2.x_axis.into(), affine.matrix2.y_axis.into()],
            translation: affine.translation.into(),
        };
        repr.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Affine2, D::Error>
    where
        D: Deserializer<'de>,
    {
        let repr = Affine2Repr::deserialize(deserializer)?;
        Ok(Affine2::from_cols_array_2d(&[
            repr.matrix[0],
            repr.matrix[1],
            repr.translation,
        ]))
    }
}
