use std::fmt::Display;

use crate::shapes::area::Area;

pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub height: f64,
    pub width: f64,
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        return self.height * self.width;
    }
}

impl Default for Rectangle {
    // static default method
    fn default() -> Self { // there is no &self, means it's a static method, hence we can use this
                           // method with :: not with .
        return Rectangle { x: 0.0, y: 0.0, height: 0.0, width: 0.0 }; 
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // todo!(); // todo!() is like pass in python  
        return write!(
            f,
            "Rectangle({}, {}): {} x {}",
            self.x, self.y, self.height, self.width
        );
    }
}

// implementing an iterator to Rectangle
pub struct RectIter {
    points: Vec<(f64, f64)>,
    idx: usize,
}

impl Iterator for RectIter {
    type Item = (f64, f64); // associated types

    fn next(&mut self) -> Option<Self::Item> {

        let idx = self.idx;
        self.idx += 1;
        return self.points.get(idx).map(|x| *x); // dereferencing

        // if self.idx >= self.points.len() {
        //     return None;
        // }

        // let point = self.points[self.idx];
        // self.idx += 1;
        // return Some(point);
    }

}

// implementing IntoIterator to convert the Rectangle into an iterator
impl IntoIterator for Rectangle {
    type Item = (f64, f64);
    type IntoIter = RectIter;

    // self = Rectangle
    fn into_iter(self) -> Self::IntoIter {
        return RectIter{
            points: vec![
                (self.x, self.y),
                (self.x + self.width, self.y),
                (self.x, self.y + self.height),
                (self.x + self.width , self.y + self.height),
            ],
            idx: 0,
        };
    }
}

impl IntoIterator for &Rectangle {
    type Item = (f64, f64);
    type IntoIter = RectIter;

    // self = Rectangle
    fn into_iter(self) -> Self::IntoIter {
        return RectIter{
            points: vec![
                (self.x, self.y),
                (self.x + self.width, self.y),
                (self.x, self.y + self.height),
                (self.x + self.width , self.y + self.height),
            ],
            idx: 0,
        };
    }
}
