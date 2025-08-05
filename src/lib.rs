mod point;

pub use point::Point;


#[derive(Debug)]
pub struct Rect {
    pub p1: Point,
    pub p2: Point,
}