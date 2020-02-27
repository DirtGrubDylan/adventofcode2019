use std::ops::{Add, Div, Mul, Sub};

use super::Location;

#[derive(Debug, PartialEq, PartialOrd, Hash, Copy, Clone)]
pub struct Point2d<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Into<f64> + Copy,
{
    pub x: T,
    pub y: T,
}

impl<T> Point2d<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Into<f64> + Copy,
{
    pub fn new(x: T, y: T) -> Point2d<T> {
        Point2d { x, y }
    }
}

impl<T> Location for Point2d<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Into<f64> + Copy,
{
    type ValueOutput = T;

    fn manhattan_distance_to(&self, other: &Point2d<T>) -> T {
        let relative_x = other.x - self.x;
        let relative_y = other.y - self.y;

        relative_x + relative_y
    }

    fn distance_to(&self, other: &Point2d<T>) -> f64 {
        let relative_x = other.x - self.x;
        let relative_y = other.y - self.y;

        let temp = (relative_x * relative_x + relative_y * relative_y).into();

        temp.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    const ORIGIN_POINT: Point2d<i32> = Point2d { x: 0, y: 0 };

    #[test]
    fn test_manhattan_distance_to() {
        let point = Point2d::new(5, 5);

        let expected = 10;

        let result = ORIGIN_POINT.manhattan_distance_to(&point);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_distance_to() {
        let point = Point2d::new(3, 4);

        let expected = 5.0;

        let result = ORIGIN_POINT.distance_to(&point);

        assert!((result - expected).abs() < EPSILON);
    }
}
