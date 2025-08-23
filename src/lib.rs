mod point;
pub use point::Point;

use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect<T> {
    pub p1: Point<T>,
    pub p2: Point<T>,
}

impl<T> Rect<T> {
    pub fn new(p1: Point<T>, p2: Point<T>) -> Self {
        Rect { p1, p2 }
    }

    pub fn from_coords(x1: T, y1: T, x2: T, y2: T) -> Self {
        Rect {
            p1: Point { x: x1, y: y1 },
            p2: Point { x: x2, y: y2 },
        }
    }
}

impl<T> Rect<T>
where
    T: Copy + Ord,
{
    pub fn normalized(self) -> Self {
        let min_x = self.p1.x.min(self.p2.x);
        let max_x = self.p1.x.max(self.p2.x);
        let min_y = self.p1.y.min(self.p2.y);
        let max_y = self.p1.y.max(self.p2.y);

        Rect {
            p1: Point { x: min_x, y: min_y },
            p2: Point { x: max_x, y: max_y },
        }
    }

    pub fn contains(&self, point: Point<T>) -> bool {
        let r = self.normalized();
        point.x >= r.p1.x && point.x <= r.p2.x && point.y >= r.p1.y && point.y <= r.p2.y
    }
}

impl<T> Rect<T>
where
    T: Copy + Ord + Sub<Output = T>,
{
    pub fn width(&self) -> T {
        let r = self.normalized();
        r.p2.x - r.p1.x
    }

    pub fn height(&self) -> T {
        let r = self.normalized();
        r.p2.y - r.p1.y
    }
}

impl<T> Rect<T>
where
    T: Copy + Ord + Add<Output = T> + Sub<Output = T> + std::ops::Mul<Output = T>,
{
    pub fn area(&self) -> T {
        self.width() * self.height()
    }
}

impl<T> Rect<T> {
    pub fn map<U, F>(self, mut f: F) -> Rect<U>
    where
        F: FnMut(T) -> U,
    {
        Rect {
            p1: Point {
                x: f(self.p1.x),
                y: f(self.p1.y),
            },
            p2: Point {
                x: f(self.p2.x),
                y: f(self.p2.y),
            },
        }
    }
}
