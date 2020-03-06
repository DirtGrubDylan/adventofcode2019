use std::ops::{Add, Div, Mul, Sub};

use super::Location;

#[derive(Debug, Eq, PartialEq, PartialOrd, Hash, Copy, Clone, Ord)]
pub struct Point3d<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Ord
        + Into<f64>
        + Copy,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3d<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Ord
        + Into<f64>
        + Copy,
{
    pub fn new(x: T, y: T, z: T) -> Point3d<T> {
        Point3d { x, y, z }
    }
}

impl<T> Location for Point3d<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Ord
        + Into<f64>
        + Copy,
{
    type ValueOutput = T;

    fn manhattan_distance_to(&self, other: &Point3d<T>) -> T {
        let relative_x = other.x - self.x;
        let relative_y = other.y - self.y;
        let relative_z = other.z - self.z;

        relative_x + relative_y + relative_z
    }

    fn distance_to(&self, other: &Point3d<T>) -> f64 {
        let relative_x = other.x - self.x;
        let relative_y = other.y - self.y;
        let relative_z = other.z - self.z;

        let temp =
            (relative_x * relative_x + relative_y * relative_y + relative_z * relative_z).into();

        temp.sqrt()
    }

    fn add(&self, other: &Point3d<T>) -> Point3d<T> {
        let new_x = self.x + other.x;
        let new_y = self.y + other.y;
        let new_z = self.z + other.z;

        Point3d::new(new_x, new_y, new_z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    const ORIGIN_POINT: Point3d<i32> = Point3d { x: 0, y: 0, z: 0 };

    #[test]
    fn test_manhattan_distance_to() {
        let point = Point3d::new(3, 4, 12);

        let expected = 19;

        let result = ORIGIN_POINT.manhattan_distance_to(&point);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_distance_to() {
        let point = Point3d::new(3, 4, 12);

        let expected = 13.0;

        let result = ORIGIN_POINT.distance_to(&point);

        assert!((result - expected).abs() < EPSILON);
    }

    #[test]
    fn test_add() {
        let first = Point3d::new(3, 4, -3);
        let second = Point3d::new(5, -1, 2);

        let expected = Point3d::new(8, 3, -1);

        let result = first.add(&second);

        assert_eq!(result, expected);
    }
}
