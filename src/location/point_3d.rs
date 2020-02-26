use std::ops::{Add, Div, Mul, Sub};

use super::Location;

#[derive(Debug, PartialEq, PartialOrd, Hash, Copy, Clone)]
pub struct Point3d<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Into<f64> + Copy,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Location for Point3d<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Into<f64> + Copy,
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
}
