
mod point;
pub use point::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect<T> {
    pub p1: Point<T>,
    pub p2: Point<T>,
}

impl<T> Rect<T> {
    pub fn new(p1: Point<T>, p2: Point<T>) -> Self {
        Rect { p1, p2 }
    }
}

