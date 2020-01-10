pub mod orbit;

use crate::file_reader::to_string_vector;
use orbit::OrbitMap;

pub fn run_day_6() {
    let file_input = to_string_vector("inputs/day_6.txt");

    match file_input {
        Ok(orbit_description) => {
            run_part_1(&orbit_description);
        }
        Err(error) => panic!("Error parsing file: {:?}", error),
    }
}

fn run_part_1(orbit_description: &[String]) {
    let mut orbit_map = OrbitMap::new();

    orbit_map.add_orbit_description(orbit_description);

    let total_number_of_direct_and_indirect_orbits = orbit_map.total_number_of_orbits();

    println!(
        "Day 6 Part 1 Solution: {}",
        total_number_of_direct_and_indirect_orbits
    );
}
