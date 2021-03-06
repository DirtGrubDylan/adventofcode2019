mod asteroids;

use crate::file_reader::to_string_vector;
use asteroids::AsteroidMap;

pub fn run_day_10() {
    let file_input = to_string_vector("inputs/day_10.txt");

    match file_input {
        Ok(input_lines) => {
            let map_data: Vec<Vec<char>> =
                input_lines.iter().map(|s| s.chars().collect()).collect();

            let asteroid_map = AsteroidMap::new(&map_data);

            run_part_1(&asteroid_map);
            run_part_2(&asteroid_map);
        }

        Err(error) => println!("Error parsing file: {:?}", error),
    }
}

fn run_part_1(asteroid_map: &AsteroidMap) {
    let best_asteroid_for_monitoring_station = asteroid_map.best_monitoring_station_location();

    println!(
        "Day 10 Part 1 Solution: {}",
        best_asteroid_for_monitoring_station.number_of_asteroids_in_los
    );
}
fn run_part_2(asteroid_map: &AsteroidMap) {
    let nth_vaporized_asteroid = asteroid_map
        .nth_vaporized_asteroid_from_best_monitoring_station(200)
        .unwrap();

    let result = nth_vaporized_asteroid.x_location * 100 + nth_vaporized_asteroid.y_location;

    println!("Day 10 Part 2 Solution: {}", result);
}
