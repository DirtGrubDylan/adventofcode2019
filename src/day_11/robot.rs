use std::collections::HashMap;

use crate::intcode_computer::{IntcodeComputer, IntcodeComputerStatus};
use crate::location::point_2d::Point2d;

#[derive(Debug, PartialEq, Clone, Copy)]
enum PaintColor {
    Black = 0,
    White = 1,
}

impl From<i32> for PaintColor {
    fn from(a: i32) -> PaintColor {
        match a {
            0 => PaintColor::Black,
            1 => PaintColor::White,
            _ => panic!("Unexpected paint conversion of: {}", a),
        }
    }
}

impl From<i128> for PaintColor {
    fn from(a: i128) -> PaintColor {
        match a {
            0 => PaintColor::Black,
            1 => PaintColor::White,
            _ => panic!("Unexpected paint conversion of: {}", a),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl From<i32> for Direction {
    fn from(a: i32) -> Direction {
        match a {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!("Unexpected direction conversion of: {}", a),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Robot {
    brain: IntcodeComputer,
    panel_map: HashMap<Point2d<i32>, PaintColor>,
    facing_direction: Direction,
    current_location: Point2d<i32>,
}

impl Robot {
    pub fn new<T>(computer_program: T) -> Robot
    where
        T: Into<IntcodeComputer>,
    {
        let brain = computer_program.into();
        let panel_map = HashMap::new();
        let facing_direction = Direction::Up;
        let current_location = Point2d::new(0, 0);

        Robot {
            brain,
            panel_map,
            facing_direction,
            current_location,
        }
    }

    pub fn run_program(&mut self) {
        self.brain.execute_program();

        while self.brain.get_status() == IntcodeComputerStatus::WaitingForInput {
            let current_paint_value = *self
                .panel_map
                .get(&self.current_location)
                .unwrap_or(&PaintColor::Black) as i128;

            self.brain.set_input(current_paint_value);

            self.brain.execute_program();

            let brain_outputs = self.brain.get_last_n_outputs(2);

            let (paint_output, direction_output) =
                match (brain_outputs.get(0), brain_outputs.get(1)) {
                    (Some(&paint_output), Some(&direction_output)) => {
                        (paint_output, direction_output)
                    }
                    _ => panic!(
                        "There were only {} number of outputs from brain!",
                        brain_outputs.len()
                    ),
                };

            self.paint_current_location(paint_output);
            self.change_direction(direction_output);
            self.move_forward();
        }
    }

    pub fn get_number_of_painted_panels(&self) -> usize {
        self.panel_map.len()
    }

    fn change_direction(&mut self, direction_output: i128) {
        self.facing_direction = match direction_output {
            0 => (((self.facing_direction as i32) + 3) % 4).into(),
            1 => (((self.facing_direction as i32) + 1) % 4).into(),
            _ => panic!("Unexpected direction output: {}", direction_output),
        };
    }

    fn paint_current_location(&mut self, paint_output: i128) {
        let new_color = paint_output.into();

        self.panel_map.insert(self.current_location, new_color);
    }

    fn move_forward(&mut self) {
        match self.facing_direction {
            Direction::Up => self.current_location.y += 1,
            Direction::Right => self.current_location.x += 1,
            Direction::Down => self.current_location.y -= 1,
            Direction::Left => self.current_location.x -= 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROGRAM: [i32; 37] = [
        3, 33, // input
        1001, 36, -1, 36, // count--
        1006, 36, 32, // jump to terminate if count == 0
        1008, 36, 2, 34, // Add 1 to paint var if count == 5
        1008, 36, 2, 35, // Add 1 to turn var if count == 5
        1, 33, 34, 34, // add paint var to input var for paint output (same as input)
        1, 33, 35, 35, // add turn var to input var for turn output (same as input)
        4, 34, // get paint output
        4, 35, // get turn output
        1005, 36, 0, // jump to beginning if count > 0
        99, // terminate 32
        0,  // input var 33
        0,  // paint var 34
        0,  // turn var 35
        7,  // count var 36
    ];

    #[test]
    fn test_change_direction() {
        let mut robot = Robot::new(PROGRAM.to_vec().as_slice());

        robot.change_direction(0);

        let expected = Direction::Left;

        let result = robot.facing_direction;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_paint_current_location() {
        let mut robot = Robot::new(PROGRAM.to_vec().as_slice());

        robot.change_direction(0);
        robot.move_forward();
        robot.change_direction(0);
        robot.move_forward();
        robot.change_direction(1);
        robot.move_forward();

        robot.paint_current_location(1);

        let expected = PaintColor::White;

        let result = robot.panel_map.get(&(robot.current_location)).unwrap();

        assert_eq!(*result, expected);
    }

    #[test]
    fn test_move_forward() {
        let mut robot = Robot::new(PROGRAM.to_vec().as_slice());

        robot.change_direction(0);
        robot.move_forward();
        robot.change_direction(0);
        robot.move_forward();
        robot.change_direction(1);
        robot.move_forward();

        let expected = Point2d::new(-2, -1);

        let result = robot.current_location;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_run_program() {
        let mut robot = Robot::new(PROGRAM.to_vec().as_slice());

        robot.run_program();

        let expected_panel_map = vec![
            (Point2d::new(0, 0), PaintColor::White),
            (Point2d::new(-1, 0), PaintColor::Black),
            (Point2d::new(-1, -1), PaintColor::Black),
            (Point2d::new(0, -1), PaintColor::Black),
            (Point2d::new(1, 0), PaintColor::Black),
            (Point2d::new(1, 1), PaintColor::Black),
        ]
        .into_iter()
        .collect();
        let expected_facing_direction = Direction::Left;
        let expected_current_location = Point2d::new(0, 1);

        assert_eq!(robot.panel_map, expected_panel_map);
        assert_eq!(robot.facing_direction, expected_facing_direction);
        assert_eq!(robot.current_location, expected_current_location);
    }
}
