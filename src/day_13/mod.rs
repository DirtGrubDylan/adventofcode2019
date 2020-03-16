mod game;

use crate::file_reader::to_string_vector;
use crate::intcode_computer::IntcodeComputer;

use game::{Game, Tile};

pub fn run_day_13() {
    let file_input = to_string_vector("inputs/day_13.txt");

    match file_input {
        Ok(programs) => {
            if let Some(program) = programs.get(0) {
                let program_values: Vec<String> =
                    program.split(",").map(|s| String::from(s)).collect();

                let mut intcode_computer = IntcodeComputer::from(program_values.as_slice());

                intcode_computer.execute_program();

                let mut game = Game::new();

                game.initialize_map(&intcode_computer.get_outputs());

                run_part_1(&game);
                run_part_2(&mut game);
            }
        }
        Err(error) => println!("Error parsing file: {:?}", error),
    }
}

pub fn run_part_1(game: &Game) {
    let mut number_of_blocks = 0;

    for tile in game.get_map_copy().values() {
        if *tile == Tile::Block {
            number_of_blocks += 1;
        }
    }

    println!("Day 13 Part 1 Solution: {}", number_of_blocks);
}

pub fn run_part_2(game: &mut Game) {
    unimplemented!();
}
