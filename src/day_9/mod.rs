use crate::file_reader::to_string_vector;

use crate::get_user_input;
use crate::intcode_computer::IntcodeComputer;

pub fn run_day_9() {
    let file_input = to_string_vector("inputs/day_9.txt");

    match file_input {
        Ok(programs) => {
            if let Some(program) = programs.get(0) {
                let program_values: Vec<String> =
                    program.split(",").map(|s| String::from(s)).collect();

                print!("Please input for the BOOST program: ");
                let user_input = get_user_input();

                let mut intcode_computer = IntcodeComputer::new(program_values.as_slice());

                intcode_computer.set_input(user_input);
                intcode_computer.execute_program();

                println!("System outputs are: {:?}", intcode_computer.get_outputs());
            }
        }
        Err(error) => println!("Error parsing file: {:?}", error),
    }
}
