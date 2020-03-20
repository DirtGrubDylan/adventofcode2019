mod game;

use std::cmp::Ordering;

use crate::file_reader::to_string_vector;
use crate::intcode_computer::IntcodeComputer;

use game::Game;

pub fn run_day_13() {
    let file_input = to_string_vector("inputs/day_13.txt");

    match file_input {
        Ok(programs) => {
            if let Some(program) = programs.get(0) {
                let program_values: Vec<String> =
                    program.split(",").map(|s| String::from(s)).collect();

                let mut intcode_computer = IntcodeComputer::from(program_values.as_slice());

                // insert two quarters
                intcode_computer.replace_code_in_program(0, 2);

                intcode_computer.execute_program();

                let mut game = Game::new();

                game.initialize_map(&intcode_computer.get_outputs());

                run_part_1(&game);
                run_part_2(&mut game, &mut intcode_computer);
            }
        }
        Err(error) => println!("Error parsing file: {:?}", error),
    }
}

pub fn run_part_1(game: &Game) {
    println!("Day 13 Part 1 Solution: {}", game.get_number_of_blocks());
}

pub fn run_part_2(game: &mut Game, intcode_computer: &mut IntcodeComputer) {
    let mut number_of_blocks = game.get_number_of_blocks();

    while 0 < number_of_blocks {
        let paddle_x_location = game.get_paddle_location().unwrap().x;
        let ball_x_location = game.get_ball_location().unwrap().x;

        let paddle_input = match ball_x_location.cmp(&paddle_x_location) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };

        intcode_computer.set_input(paddle_input);

        intcode_computer.execute_program();


        game.initialize_map(&intcode_computer.get_outputs());

        number_of_blocks = game.get_number_of_blocks();

        // uncomment to watch the game :)
        //game.print_map();
    }

    println!("Day 13 Part 2 Solution: {}", game.get_score());
}
