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

    #[allow(dead_code)]
    pub fn to_char(&self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::Block => '0',
            Tile::HorizontalPaddle => '_',
            Tile::Ball => '*',
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Game {
    map: HashMap<Point2d<i32>, Tile>,
    score: i128,
    number_of_blocks: i32,
    ball_location: Option<Point2d<i32>>,
    paddle_location: Option<Point2d<i32>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            map: HashMap::new(),
            score: 0,
            number_of_blocks: 0,
            ball_location: None,
            paddle_location: None,
        }
    }

    pub fn initialize_map(&mut self, data: &[i128]) {
        if data.len() % 3 != 0 {
            panic!("The data given to initialize the map should be in groups of three!");
        }

        self.number_of_blocks = 0;

        for group in data.chunks(3) {
            if group[0] == -1 {
                self.score = group[2];

                continue;
            }

            let x_position = group[0] as i32;
            let y_position = group[1] as i32;

            let point = Point2d::new(x_position, y_position);
            let tile = Tile::from_i128(group[2]);

            match tile {
                Tile::Block  => self.number_of_blocks += 1,
                Tile::HorizontalPaddle => self.paddle_location = Some(point.clone()),
                Tile::Ball => self.ball_location = Some(point.clone()),
                _ => ()
            };

            if let Some(map_tile) = self.map.get(&point) {
                if *map_tile == Tile::Block && tile == Tile::Empty {
                    self.number_of_blocks -= 1;
                }
            }

            self.map.insert(point, tile);
        }
    }

    pub fn get_number_of_blocks(&self) -> i32 {
        //self.map.values().filter(|tile| (**tile) == Tile::Block).count()
        self.number_of_blocks
    }

    pub fn get_paddle_location(&self) -> Option<Point2d<i32>> {
        self.paddle_location.clone()
    }

    pub fn get_ball_location(&self) -> Option<Point2d<i32>> {
        self.ball_location.clone()
    }
    
    pub fn get_score(&self) -> i128 {
        self.score
    }

    pub fn get_max_number_of_tiles(&self) -> usize {
        self.map.len() * 3 * 100
    }

    #[allow(dead_code)]
    pub fn print_map(&self) {
        let min_x = self.map.keys().min_by_key(|point| point.x).unwrap().x;
        let min_y = self.map.keys().min_by_key(|point| point.y).unwrap().y;
        let max_x = self.map.keys().max_by_key(|point| point.x).unwrap().x + 1;
        let max_y = self.map.keys().max_by_key(|point| point.y).unwrap().y + 1;

        let mut temp_map_vec = Vec::new();

        for row in min_y..max_y {
            let mut temp_string = String::new();

            for column in min_x..max_x {
                let temp_point = Point2d::new(column, row);

                let temp_char = self.map.get(&temp_point).unwrap_or(&Tile::Empty).to_char();

                temp_string.push(temp_char);
            }

            temp_map_vec.push(temp_string);
        }

        print!("\x1B[2J");

        for string in temp_map_vec {
            println!("{}", string);
        }
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
        ]
        .into_iter()
        .collect();

        let result = game.get_map_copy();

        assert_eq!(result, expected);
    }
}
