use crate::file_reader::to_string_vector;

pub fn run_day_9() {
    let file_input = to_string_vector("inputs/day_7.txt");

    match file_input {
        Ok(programs) => {
            if let Some(program) = programs.get(0) {
            }
        }
        Err(error) => println!("Error parsing file: {:?}", error),
    }
}
