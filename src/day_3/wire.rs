use super::line::Line;

#[derive(Debug)]
struct Wire {
    horizontal_lines: Vec<Line>,
    vertical_lines: Vec<Line>,
    pub lines: Vec<Line>,
}
