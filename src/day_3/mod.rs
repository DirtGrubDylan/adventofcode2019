mod line;
mod wire;

use crate::file_reader::to_string_vector;
use wire::Wire;

pub fn run_day_3() {
    let file_input = to_string_vector("input/day_3_part_1.txt");

    match file_input {
        Ok(moves) => {},
        Err(error) => panic!("Error parsing file: {:?}", error);
    }
}
