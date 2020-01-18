mod amplifiers;

use crate::file_reader::to_string_vector;
use amplifiers::AmplifierCircuit;

pub fn run_day_7() {
    let file_input = to_string_vector("inputs/day_7.txt");

    match file_input {
        Ok(programs) => {
            if let Some(program) = programs.get(0) {
                let amplifier_names = ["A", "B", "C", "D", "E"];
                let program_values: Vec<i32> = program
                    .split(",")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();

                let mut amplifier_circuit =
                    AmplifierCircuit::new(&amplifier_names, &program_values);

                run_part_1(&mut amplifier_circuit);
            }
        }
        Err(error) => println!("Error parsing file: {:?}", error),
    }
}

fn run_part_1(amplifier_circuit: &mut AmplifierCircuit) {
    let result = amplifier_circuit.get_largest_output_signal().unwrap();

    println!("Day 7 Part 1: {:?}", result);
}
