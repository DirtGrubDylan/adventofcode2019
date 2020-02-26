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
