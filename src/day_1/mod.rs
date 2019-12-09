mod fuel_counter_upper;
mod module;

use crate::file_reader::to_string_vector;
use fuel_counter_upper::FuelCounterUpper;

pub fn run_day_1() {
    let module_masses_result = to_string_vector("inputs/day_1.txt");

    match module_masses_result {
        Ok(module_masses) => {
            let fuel_counter_upper = FuelCounterUpper::new(&module_masses);

            println!(
                "Part 1 Solution is: {:?}",
                fuel_counter_upper.total_fuel_required()
            );
            println!(
                "Part 2 Solution is: {:?}",
                fuel_counter_upper.total_fuel_required_recursive()
            );
        }
        Err(error) => println!("Error parsing file: {:?}", error),
    }
}
