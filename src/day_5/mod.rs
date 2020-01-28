use crate::file_reader::to_string_vector;
use crate::get_user_input;
use crate::intcode_computer::IntcodeComputer;

pub fn run_day_5() {
    let file_input = to_string_vector("inputs/day_5.txt");

    match file_input {
        Ok(programs) => {
            if let Some(program) = programs.get(0) {
                let program_values: Vec<String> =
                    program.split(",").map(|s| String::from(s)).collect();

                print!("Please input a system ID to diagnose: ");
                let user_input = get_user_input();

                let mut intcode_computer = IntcodeComputer::from(program_values.as_slice());

                intcode_computer.set_input(user_input);

                let output = intcode_computer.execute_program();

                println!("System output is: {}", output.unwrap());
            }
        }
        Err(error) => println!("Error parsing file: {:?}", error),
    }
}
