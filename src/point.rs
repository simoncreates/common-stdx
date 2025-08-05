
use std::ops::{Add, Sub, Mul, Neg};
use num_traits::Zero;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl<T> Point<T>
where
    T: Copy + Zero + PartialEq,
{
    pub fn zero() -> Self {
        Point {
            x: T::zero(),
            y: T::zero(),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.x == T::zero() && self.y == T::zero()
    }
}

impl<T> Add for Point<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Point<T>;
    fn add(self, rhs: Point<T>) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Point<T>;
    fn sub(self, rhs: Point<T>) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul<T> for Point<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Point<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Neg for Point<T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Point<T>;
    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> Point<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T>,
{
    pub fn dot(self, other: Point<T>) -> T {
        self.x * other.x + self.y * other.y
    }
}

#[cfg(feature = "float")]
impl<T> Point<T>
where
    T: Copy + Sub<Output = T> + Into<f64>,
{
    pub fn distance(self, other: Point<T>) -> f64 {
        let dx: f64 = (self.x - other.x).into();
        let dy: f64 = (self.y - other.y).into();
        (dx * dx + dy * dy).sqrt()
    }

    pub fn magnitude(self) -> f64 {
        let x: f64 = self.x.into();
        let y: f64 = self.y.into();
        (x * x + y * y).sqrt()
    }
}

