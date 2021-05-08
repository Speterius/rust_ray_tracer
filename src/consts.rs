use crate::{Scalar, Color};

pub const NORM_EPS: Scalar = 1e-10;
pub const PI: Scalar = std::f64::consts::PI;

// Pre-define a few colors.
pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);
pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);
pub const DARKGRAY: Color = Color::new(0.25, 0.25, 0.25);
pub const MIDGRAY: Color = Color::new(0.25, 0.25, 0.25);
pub const LIGHTGRAY: Color = Color::new(0.75, 0.75, 0.75);
pub const SKYBLUE: Color = Color::new(0.5, 0.7, 1.0);
pub const RED: Color = Color::new(0.9, 0.1, 0.1);
pub const BLUE: Color = Color::new(0.1, 0.1, 0.9);
pub const GREEN: Color = Color::new(0.1, 0.9, 0.1);
pub const ORANGE: Color = Color::new(144.0/255.0, 12.0/255.0, 63.0/255.0);
