mod moons;

use crate::file_reader::to_string_vector;
use crate::location::point_3d::Point3d;

use moons::System;

const INVALID_CHARS: [char; 7] = ['<', '=', ' ', '>', 'x', 'y', 'z'];

pub fn run_day_12() {
    let file_input = to_string_vector("inputs/day_12.txt");

    match file_input {
        Ok(locations_vec_str) => {
            let locations: Vec<Point3d<i32>> = locations_vec_str
                .iter()
                .map(|location_str| location_str_to_point(&location_str))
                .collect();

            let mut moon_system = System::new();

            for location in locations {
                moon_system.add_moon_at(location);
            }

            run_part_1(&mut moon_system);
            run_part_2(&mut moon_system);
        }
        Err(error) => println!("Error parsing file: {:?}", error),
    }
}

fn run_part_1(moon_system: &mut System) {
    moon_system.reset();

    let total_energy_of_system = moon_system.iter_mut().skip(999).next();

    match total_energy_of_system {
        Some(energy) => println!("Day 12 Part 1 Solution: {}", energy),
        None => panic!("The moon system reached entropy!!!!!"),
    }
}

fn run_part_2(moon_system: &mut System) {
    moon_system.reset();

    let steps_to_origin = moon_system.steps_to_get_moons_at_original_positions_and_velocities();

    println!("Day 12 Part 2 Solution: {}", steps_to_origin);
}

fn location_str_to_point(location_str: &str) -> Point3d<i32> {
    let temp: Vec<i32> = location_str
        .replace(&INVALID_CHARS[..], "")
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

    let (x, y, z) = match (temp.get(0), temp.get(1), temp.get(2)) {
        (Some(&x), Some(&y), Some(&z)) => (x, y, z),
        _ => panic!(
            "Location string {} was not correctly formatted!",
            location_str
        ),
    };

    Point3d::new(x, y, z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_str_to_point() {
        let location_str = String::from("<x=9, y=-16, z=-3>");

        let expected = Point3d::new(9, -16, -3);

        let result = location_str_to_point(&location_str);

        assert_eq!(result, expected);
    }
}
