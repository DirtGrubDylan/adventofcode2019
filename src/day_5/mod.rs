use std::io::{self, Write};

use crate::file_reader::to_string_vector;
use crate::day_2::intcode_computer::IntcodeComputer;

pub fn run_day_5() {
    let file_input = to_string_vector("inputs/day_5.txt");

    match file_input {
        Ok(programs) => {
            if let Some(program) = programs.get(0) {
                find_first_solution(&program);
            }
        }
        Err(error) => println!("Error parsing file: {:?}", error),
    }
}

pub fn find_first_solution(program: &str) {
    let program_values: Vec<String> = program.split(",").map(|s| String::from(s)).collect();

    let user_input = get_user_input();
    let intcode_computer = IntcodeComputer::from(program_values.as_slice());

    println!("Diagnosing system ID: {}", user_input);

    let (_, output_values) = intcode_computer.run_program(user_input);

    println!("Part 1 output_values are: {:?}", output_values);
}

pub fn get_user_input() -> i32 {
    let mut input_buffer = String::new();

    print!("Please input a system ID to diagnose: ");

    io::stdout().flush().expect("Could not flush stdout!");

    io::stdin().read_line(&mut input_buffer).expect("Failed to read user input!");

    input_buffer.trim().parse::<i32>().expect("Failed to parse user_input!")
}
