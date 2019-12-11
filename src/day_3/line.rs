#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Line {
    pub start_range: i32,
    pub end_range: i32,
    pub offset: i32,
    pub direction: Direction,
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

    pub fn distance_from_point_to_end(&self, point: i32) -> i32 {
        match self.direction {
            Direction::Right | Direction::Up => (self.end_range - point).abs(),
            Direction::Left | Direction::Down => (self.start_range - point).abs(),
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
        let line5 = Line::new(0, 8, 2, "R");

        let mut lines = vec![
            line1.clone(),
            line2.clone(),
            line3.clone(),
            line4.clone(),
            line5.clone(),
        ];

        let expected = vec![
            line4.clone(),
            line3.clone(),
            line5.clone(),
            line2.clone(),
            line1.clone(),
        ];

        lines.sort();

        assert_eq!(lines, expected);
    }

    #[test]
    fn test_distance_from_point_to_end_right() {
        let line = Line::new(0, 8, 0, "R");

        let expected = 3;

        let result = line.distance_from_point_to_end(5);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_distance_from_point_to_end_up() {
        let line = Line::new(0, 7, 0, "U");

        let expected = 2;

        let result = line.distance_from_point_to_end(5);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_distance_from_point_to_end_left() {
        let line = Line::new(3, 8, 5, "L");

        let expected = 3;

        let result = line.distance_from_point_to_end(6);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_distance_from_point_to_end_down() {
        let line = Line::new(2, 5, 3, "D");

        let expected = 1;

        let result = line.distance_from_point_to_end(3);

        assert_eq!(result, expected);
    }
}
