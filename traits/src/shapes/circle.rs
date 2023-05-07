use std::fmt::Display;

use crate::shapes::area::Area;

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return std::f64::consts::PI * self.radius * self.radius;
    }
}
// implementing foreign trait
impl Default for Circle {
    fn default() -> Self { // static default method
        return Circle { x: 0.0, y: 0.0, radius: 0.0 };
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "Circle({}, {}): {}",
            self.x, self.y, self.radius
        );
    }
}
