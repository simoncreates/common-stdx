#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug)]
pub struct Rect {
    pub p1: Point,
    pub p2: Point,
}