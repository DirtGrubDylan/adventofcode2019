#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Line {
    start_range: i32,
    end_range: i32,
    offset: i32,
    direction: Direction,
}

impl Line {
    pub fn new(start_range: i32, end_range: i32, offset: i32, direction_str: &str) -> Line {
        let direction = match direction_str {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("Unknown `position_str` given: {:?}", direction_str),
        };

        Line {
            start_range: start_range,
            end_range: end_range,
            offset: offset,
            direction: direction,
        }
    }

    pub fn get_relative_end_x_y_position(&self) -> (i32, i32) {
        match self.direction {
            Direction::Right => (self.end_range, self.offset),
            Direction::Left => (self.start_range, self.offset),
            Direction::Up => (self.offset, self.end_range),
            Direction::Down => (self.offset, self.start_range),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let expected = Line {
            start_range: 0,
            end_range: 8,
            offset: 0,
            direction: Direction::Right,
        };

        let result = Line::new(0, 8, 0, "R");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_relative_end_x_y_position_right() {
        let expected = (8, 0);

        let result = Line::new(0, 8, 0, "R").get_relative_end_x_y_position();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_relative_end_x_y_position_down() {
        let expected = (3, 2);

        let result = Line::new(2, 5, 3, "D").get_relative_end_x_y_position();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_relative_end_x_y_position_left() {
        let expected = (3, 5);

        let result = Line::new(3, 8, 5, "L").get_relative_end_x_y_position();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_relative_end_x_y_position_up() {
        let expected = (8, 5);

        let result = Line::new(0, 5, 8, "U").get_relative_end_x_y_position();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_ordering_less_same_position() {
        // line1.start_range < line2.start_range
        let line1 = Line::new(0, 8, 0, "R");
        let line2 = Line::new(3, 8, 0, "R");

        // line3.offset < line4.offset
        let line3 = Line::new(3, 8, 0, "R");
        let line4 = Line::new(3, 8, 5, "R");

        // line5.start_range <= line6.start_range
        // && line5.offset <= line6.offset
        let line5 = Line::new(0, 8, 0, "R");
        let line6 = Line::new(3, 8, 5, "L");

        // line7.start_range < line8.start_range
        // && line7.offset > line8.offset
        let line7 = Line::new(0, 8, 5, "R");
        let line8 = Line::new(3, 8, 0, "L");

        // line9.end_range < line10.end_range
        let line9 = Line::new(0, 5, 8, "R");
        let line10 = Line::new(0, 8, 0, "L");

        assert!(line1 < line2);
        assert!(line3 < line4);
        assert!(line5 < line6);
        assert!(line7 < line8);
        assert!(line9 < line10);
    }

    #[test]
    fn test_ordering_less_different_position() {
        let line1 = Line::new(0, 8, 0, "R");
        let line2 = Line::new(0, 8, 0, "U");

        assert!(line1 < line2);
    }

    #[test]
    fn test_ordering_equal() {
        let line1 = Line::new(0, 8, 0, "R");
        let line2 = Line::new(0, 8, 0, "R");

        let line3 = Line::new(0, 8, 0, "U");
        let line4 = Line::new(0, 8, 0, "U");

        assert_eq!(line1, line2);
        assert_eq!(line3, line4);
    }

    #[test]
    fn test_sorting() {
        let line1 = Line::new(3, 8, 5, "L");
        let line2 = Line::new(2, 5, 3, "D");
        let line3 = Line::new(0, 8, 0, "R");
        let line4 = Line::new(0, 5, 8, "U");

        let mut lines = vec![line1.clone(), line2.clone(), line3.clone(), line4.clone()];

        let expected = vec![line4.clone(), line3.clone(), line2.clone(), line1.clone()];

        lines.sort();

        assert_eq!(lines, expected);
    }
}
