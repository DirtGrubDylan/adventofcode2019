use crate::file_reader::to_string_vector;
use crate::intcode_computer::IntcodeComputer;

pub fn run_day_13() {
    let file_input = to_string_vector("inputs/day_13.txt");

    match file_input {
        Ok(programs) => {
            if let Some(program) = programs.get(0) {
                let program_values: Vec<String> =
                    program.split(",").map(|s| String::from(s)).collect();

                let mut intcode_computer = IntcodeComputer::from(program_values.as_slice());

                run_part_1();
                run_part_2();
            }
        }
        Err(error) => println!("Error parsing file: {:?}", error),
    }
}

pub fn run_part_1() {
    unimplemented!();
}

pub fn run_part_2() {
    unimplemented!();
}
