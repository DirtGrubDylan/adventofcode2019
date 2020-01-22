use crate::file_reader::to_string_vector;
use crate::intcode_computer::IntcodeComputer;

pub fn run_day_2() {
    let file_input = to_string_vector("inputs/day_2.txt");

    match file_input {
        Ok(programs) => {
            if let Some(program) = programs.get(0) {
                find_first_solution(&program);
                find_second_solution(&program, 19690720, 99, 99);
            }
        }
        Err(error) => println!("Error parsing file: {:?}", error),
    }
}

fn find_first_solution(program: &str) {
    let program_values: Vec<String> = program.split(",").map(|s| String::from(s)).collect();

    let mut intcode_computer = IntcodeComputer::from(program_values.as_slice());

    intcode_computer.replace_code_in_program(1, 12);
    intcode_computer.replace_code_in_program(2, 2);

    intcode_computer.execute_program();

    let first_value = intcode_computer.get_current_memory()[0];

    println!("Part 1 Solution is: {:?}", first_value);
}

fn find_second_solution(program: &str, target: i32, largest_noun: i32, largest_verb: i32) {
    let program_values: Vec<String> = program.split(",").map(|s| String::from(s)).collect();

    let mut intcode_computer = IntcodeComputer::from(program_values.as_slice());

    let mut solution: Result<i32, &str> = Err("Couldn't find a solution!");

    let mut noun = 0;

    while (noun <= largest_noun) && solution.is_err() {
        let mut verb_upper_bound = largest_verb;
        let mut verb_lower_bound = 0;

        let mut verb = int_between(verb_lower_bound, verb_upper_bound);

        while verb_lower_bound < verb && verb < verb_upper_bound && solution.is_err() {
            intcode_computer.replace_code_in_program(1, noun);
            intcode_computer.replace_code_in_program(2, verb);

            intcode_computer.execute_program();

            let output = intcode_computer.get_current_memory()[0];

            if output == target {
                solution = Ok(100 * noun + verb);
            } else if output > target {
                verb_upper_bound = verb;
                verb = int_between(verb_lower_bound, verb_upper_bound);
            } else {
                verb_lower_bound = verb;
                verb = int_between(verb_lower_bound, verb_upper_bound);
            }

            intcode_computer.reset();
        }

        noun += 1;
    }

    match solution {
        Ok(value) => println!("Part 2 Solution is: {:?}", value),
        Err(error) => panic!(error),
    }
}

fn int_between(lower_bound: i32, upper_bound: i32) -> i32 {
    let new_lower_bound = lower_bound as f64;
    let new_upper_bound = upper_bound as f64;

    let between_value = (new_upper_bound + new_lower_bound) / 2.0;

    between_value.round() as i32
}
