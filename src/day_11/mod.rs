use crate::file_reader::to_string_vector;

pub fn run_day_11() {
    let file_input = to_string_vector("inputs/day_11.txt");

    match file_input {
        Ok(programs) => {
            if let Some(program) = programs.get(0) {
                let program_values: Vec<String> =
                    program.split(",").map(|s| String::from(s)).collect();

                run_part_1();
                run_part_2();
            }
        }
        Err(error) => println!("Error parsing file: {:?}", error),
    }
}

fn run_part_1() {
    unimplemented!();
}

fn run_part_2() {
    unimplemented!();
}
