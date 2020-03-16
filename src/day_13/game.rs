use std::collections::HashMap;

use crate::location::point_2d::Point2d;

#[derive(Debug, PartialEq, Clone)]
pub enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl Tile {
    pub fn from_i128(value: i128) -> Tile {
        match value {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            _ => panic!("Unknown Tile value: {}", value),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Game {
    map: HashMap<Point2d<i32>, Tile>,
}

impl Game {
    pub fn new() -> Game {
        Game { map: HashMap::new() }
    }

    pub fn initialize_map(&mut self, data: &[i128]) {
        if data.len() % 3 != 0 {
            panic!("The data given to initialize the map should be in groups of three!");
        }

        for group in data.chunks(3) {
            let x_position = group[0] as i32;
            let y_position = group[1] as i32;

            let point = Point2d::new(x_position, y_position);
            let tile = Tile::from_i128(group[2]);

            self.map.insert(point, tile);
        }
    }

    pub fn get_map_copy(&self) -> HashMap<Point2d<i32>, Tile> {
        self.map.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAP_OUTPUT: [i128; 6] = [1, 2, 3, 6, 5, 4];

    #[test]
    fn test_initialize_map() {
        let mut game = Game::new();

        game.initialize_map(&MAP_OUTPUT);

        let expected = vec![
            (Point2d::new(1, 2), Tile::HorizontalPaddle),
            (Point2d::new(6, 5), Tile::Ball),
        ].into_iter().collect();

        let result = game.get_map_copy();

        assert_eq!(result, expected);
    }
}
