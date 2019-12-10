use super::line::{Direction, Line};

#[derive(Debug, PartialEq)]
struct Wire {
    pub lines: Vec<Line>,
    horizontal_lines: Vec<Line>,
    vertical_lines: Vec<Line>,
}

impl Wire {
    pub fn new<A>(args: A) -> Wire
    where
        A: Into<Wire>,
    {
        args.into()
    }

    pub fn intersection_distance(&self, other: &Wire) -> Option<i32> {
        other
            .lines
            .iter()
            .map(|temp_line| self.line_intersection_distance(temp_line))
            .filter(|distance| distance.is_some())
            .map(|distance| distance.unwrap())
            .min()
    }

    fn line_intersection_distance(&self, line: &Line) -> Option<i32> {
        // If line is vertical, iterate the horizontal lines
        // If line is horizontal, iterate the vertical lines
        let lines_iter = match line.direction {
            Direction::Up | Direction::Down => self.horizontal_lines.iter(),
            Direction::Right | Direction::Left => self.vertical_lines.iter(),
        };

        let minimum_line_intersection = lines_iter
            // filter if line.offset is in temp_line range
            .filter(|temp_line| {
                temp_line.start_range <= line.offset && line.offset <= temp_line.end_range
            })
            // filter if horizontal_line.offset is in vertical_line range
            .filter(|temp_line| {
                line.start_range <= temp_line.offset && temp_line.offset <= line.end_range
            })
            // make sure both offsets are not 0
            .filter(|temp_line| line.offset != 0 && temp_line.offset != 0)
            // find min by horizontal_line.offset
            .min_by(|line1, line2| line1.offset.cmp(&line2.offset));

        match minimum_line_intersection {
            Some(temp_line) => Some(line.offset + temp_line.offset),
            None => None,
        }
    }
}

impl From<&[String]> for Wire {
    fn from(a: &[String]) -> Wire {
        let mut lines: Vec<Line> = Vec::new();
        let mut horizontal_lines: Vec<Line> = Vec::new();
        let mut vertical_lines: Vec<Line> = Vec::new();

        let mut current_end_x_y_endpoint: (i32, i32) = (0, 0);

        for s in a {
            let (direction_str, distance_str) = &s.split_at(1);
            let distance = distance_str.parse::<i32>().unwrap();

            match direction_str {
                &"R" => {
                    let next_x_endpoint = current_end_x_y_endpoint.0 + distance;

                    let line = Line::new(
                        current_end_x_y_endpoint.0,
                        next_x_endpoint,
                        current_end_x_y_endpoint.1,
                        direction_str,
                    );

                    lines.push(line.clone());
                    horizontal_lines.push(line);

                    current_end_x_y_endpoint.0 = next_x_endpoint;
                }
                &"L" => {
                    let next_x_endpoint = current_end_x_y_endpoint.0 - distance;

                    let line = Line::new(
                        next_x_endpoint,
                        current_end_x_y_endpoint.0,
                        current_end_x_y_endpoint.1,
                        direction_str,
                    );

                    lines.push(line.clone());
                    horizontal_lines.push(line);

                    current_end_x_y_endpoint.0 = next_x_endpoint;
                }
                &"U" => {
                    let next_y_endpoint = current_end_x_y_endpoint.1 + distance;

                    let line = Line::new(
                        current_end_x_y_endpoint.1,
                        next_y_endpoint,
                        current_end_x_y_endpoint.0,
                        direction_str,
                    );

                    lines.push(line.clone());
                    vertical_lines.push(line);

                    current_end_x_y_endpoint.1 = next_y_endpoint;
                }
                &"D" => {
                    let next_y_endpoint = current_end_x_y_endpoint.1 - distance;

                    let line = Line::new(
                        next_y_endpoint,
                        current_end_x_y_endpoint.1,
                        current_end_x_y_endpoint.0,
                        direction_str,
                    );

                    lines.push(line.clone());
                    vertical_lines.push(line);

                    current_end_x_y_endpoint.1 = next_y_endpoint;
                }
                _ => panic!("Uknown move found: {:?}", s),
            }
        }

        horizontal_lines.sort();
        vertical_lines.sort();

        Wire {
            lines: lines,
            horizontal_lines: horizontal_lines,
            vertical_lines: vertical_lines,
        }
    }
}

impl From<&str> for Wire {
    fn from(a: &str) -> Wire {
        let moves: Vec<String> = a.split(",").map(|s| String::from(s)).collect();

        Wire::new(moves.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_from_string_vec() {
        let moves = vec![
            String::from("R8"),
            String::from("U5"),
            String::from("L5"),
            String::from("D3"),
        ];

        let lines = vec![
            Line::new(0, 8, 0, "R"),
            Line::new(0, 5, 8, "U"),
            Line::new(3, 8, 5, "L"),
            Line::new(2, 5, 3, "D"),
        ];
        let horizontal_lines = vec![Line::new(0, 8, 0, "R"), Line::new(3, 8, 5, "L")];
        let vertical_lines = vec![Line::new(0, 5, 8, "U"), Line::new(2, 5, 3, "D")];

        let expected = Wire {
            lines: lines,
            horizontal_lines: horizontal_lines,
            vertical_lines: vertical_lines,
        };

        let result = Wire::new(moves.as_slice());

        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_from_str() {
        let moves =
            String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72,U62,R66,U55,R34,D71,R55,D58,R83");

        let lines = vec![
            Line::new(0, 75, 0, "R"),
            Line::new(-30, 0, 75, "D"),
            Line::new(75, 158, -30, "R"),
            Line::new(-30, 53, 158, "U"),
            Line::new(146, 158, 53, "L"),
            Line::new(4, 53, 146, "D"),
            Line::new(146, 217, 4, "R"),
            Line::new(4, 11, 217, "U"),
            Line::new(145, 217, 11, "L"),
            Line::new(11, 73, 145, "U"),
            Line::new(145, 211, 73, "R"),
            Line::new(73, 128, 211, "U"),
            Line::new(211, 245, 128, "R"),
            Line::new(57, 128, 245, "D"),
            Line::new(245, 300, 57, "R"),
            Line::new(-1, 57, 300, "D"),
            Line::new(300, 383, -1, "R"),
        ];
        let horizontal_lines = vec![
            Line::new(0, 75, 0, "R"),
            Line::new(75, 158, -30, "R"),
            Line::new(145, 211, 73, "R"),
            Line::new(145, 217, 11, "L"),
            Line::new(146, 158, 53, "L"),
            Line::new(146, 217, 4, "R"),
            Line::new(211, 245, 128, "R"),
            Line::new(245, 300, 57, "R"),
            Line::new(300, 383, -1, "R"),
        ];
        let vertical_lines = vec![
            Line::new(-30, 0, 75, "D"),
            Line::new(-30, 53, 158, "U"),
            Line::new(-1, 57, 300, "D"),
            Line::new(4, 11, 217, "U"),
            Line::new(4, 53, 146, "D"),
            Line::new(11, 73, 145, "U"),
            Line::new(57, 128, 245, "D"),
            Line::new(73, 128, 211, "U"),
        ];

        let expected = Wire {
            lines: lines,
            horizontal_lines: horizontal_lines,
            vertical_lines: vertical_lines,
        };

        let result = Wire::new(moves.as_str());

        assert_eq!(result, expected);
    }

    #[test]
    fn test_wires_intersect() {
        let wire1 = Wire::new("R8,U5,L5,D3");
        let wire2 = Wire::new("U7,R6,D4,L4");
        let expected1 = Some(6);

        let wire3 = Wire::new("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let wire4 = Wire::new("U62,R66,U55,R34,D71,R55,D58,R83");
        let expected2 = Some(159);

        assert_eq!(wire1.intersection_distance(&wire2), expected1);
        assert_eq!(wire3.intersection_distance(&wire4), expected2);
    }

    #[test]
    fn test_wires_dont_intersect() {
        let wire1 = Wire::new("R8,U5,L5,D3");
        let wire2 = Wire::new("U7,R6,U4,L4");

        assert!(wire1.intersection_distance(&wire2).is_none());
    }

    #[test]
    fn test_line_doesnt_intersect() {
        let wire = Wire::new("R8,U5,L5,D3");

        let line1 = Line::new(0, 7, 0, "U");
        let line2 = Line::new(0, 6, 7, "R");

        assert!(wire.line_intersection_distance(&line1).is_none());
        assert!(wire.line_intersection_distance(&line2).is_none());
    }

    #[test]
    fn test_vertical_line_intersects() {
        let moves =
            String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72,U62,R66,U55,R34,D71,R55,D58,R83");

        let wire = Wire::new(moves.as_str());

        let line = Line::new(10, 74, 147, "D");

        let expected = Some(158);

        let result = wire.line_intersection_distance(&line);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_vertical_line_doesnt_intersect() {
        let moves =
            String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72,U62,R66,U55,R34,D71,R55,D58,R83");

        let wire = Wire::new(moves.as_str());

        let line = Line::new(15, 50, 147, "D");

        let result = wire.line_intersection_distance(&line);

        assert!(result.is_none());
    }

    #[test]
    fn test_horizontal_line_intersects() {
        let moves =
            String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72,U62,R66,U55,R34,D71,R55,D58,R83");

        let wire = Wire::new(moves.as_str());

        let line = Line::new(145, 301, 10, "R");

        let expected = Some(156);

        let result = wire.line_intersection_distance(&line);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_horizontal_line_doesnt_intersect() {
        let moves =
            String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72,U62,R66,U55,R34,D71,R55,D58,R83");

        let wire = Wire::new(moves.as_str());

        let line = Line::new(145, 301, -31, "R");

        let result = wire.line_intersection_distance(&line);

        assert!(result.is_none());
    }

    #[test]
    fn test_origin_line_doesnt_intersect() {
        let wire = Wire::new("R8,U5,L5,D3");

        let line = Line::new(0, 7, 0, "U");

        let result = wire.line_intersection_distance(&line);

        assert!(result.is_none());
    }
}
