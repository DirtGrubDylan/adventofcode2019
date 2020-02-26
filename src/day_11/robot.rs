use std::collections::HashMap;

use crate::intcode_computer::{IntcodeComputer, IntcodeComputerStatus};
use crate::location::point_2d::Point2d;

#[derive(Debug)]
enum PaintColor {
    Black,
    White,
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq)]
pub struct Robot {
    brain: IntcodeComputer,
    panel_map: HashMap<Point2d<i32>, PaintColor>,
    facing_direction: Direction,
    current_location: Point2d<i32>,
}
