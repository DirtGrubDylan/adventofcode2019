mod line;
mod wire;

use crate::file_reader::to_string_vector;
use wire::Wire;

pub fn run_day_3() {
    let file_input = to_string_vector("inputs/day_3.txt");

    match file_input {
        Ok(moves) => {
            run_part_1(&moves);
            run_part_2(&moves);
        }
        Err(error) => panic!("Error parsing file: {:?}", error),
    }
}

fn run_part_1(moves: &[String]) {
    let first_wire = Wire::new(moves[0].as_str());
    let second_wire = Wire::new(moves[1].as_str());

    match first_wire.intersection_distance(&second_wire) {
        Some(distance) => println!("Day 3 Part 1 Solution: {:?}", distance),
        None => println!("Day 3 Part 1 Solution not found!"),
    }
}

fn run_part_2(moves: &[String]) {
    let first_wire = Wire::new(moves[0].as_str());
    let second_wire = Wire::new(moves[1].as_str());

    match first_wire.best_time_intersection_distance(&second_wire) {
        Some(distance) => println!("Day 3 Part 2 Solution: {:?}", distance),
        None => println!("Day 3 Part 2 Solution not found!"),
    }
}
