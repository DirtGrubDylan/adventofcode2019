const EPSILON: f64 = 1e-10;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Asteroid {
    pub x_location: i32,
    pub y_location: i32,
    pub number_of_asteroids_in_los: i32,
    pub other_asteroids_grouped_by_relative_polar_coords: Vec<Vec<Asteroid>>,
}

impl Asteroid {
    const fn new(x_location: i32, y_location: i32, number_of_asteroids_in_los: i32) -> Asteroid {
        Asteroid {
            x_location,
            y_location,
            number_of_asteroids_in_los,
            other_asteroids_grouped_by_relative_polar_coords: Vec::new(),
        }
    }

    fn create_from_map(x_location: i32, y_location: i32, map_data: &[Vec<char>]) -> Asteroid {
        let temp_asteroid = Asteroid::new(x_location, y_location, 0);

        let other_asteroids_grouped_by_relative_polar_coords =
            temp_asteroid.number_of_asteroids_in_los(map_data);

        let number_of_asteroids_in_los =
            other_asteroids_grouped_by_relative_polar_coords.len() as i32;

        Asteroid {
            x_location,
            y_location,
            number_of_asteroids_in_los,
            other_asteroids_grouped_by_relative_polar_coords,
        }
    }

    fn number_of_asteroids_in_los(&self, map_data: &[Vec<char>]) -> Vec<Vec<Asteroid>> {
        let mut groups_by_relative_polar_coords: Vec<Vec<Asteroid>> = Vec::new();

        let mut other_asteroids: Vec<Asteroid> = self
            .get_all_other_asteroid_coordinates(map_data)
            .iter()
            .map(|&(x, y)| Asteroid::new(x, y, 0))
            .collect();

        other_asteroids.sort_by(|a, b| {
            let (r1, t1) = self.relative_polar_coordinates_of(&a);
            let (r2, t2) = self.relative_polar_coordinates_of(&b);

            (-t1)
                .partial_cmp(&(-t2))
                .unwrap()
                .then(r1.partial_cmp(&r2).unwrap())
        });

        let (_, mut t1) = self.relative_polar_coordinates_of(&other_asteroids[0]);
        let mut current_group: Vec<Asteroid> = vec![other_asteroids[0].clone()];

        for other_asteroid in other_asteroids.iter().skip(1) {
            let (_, t2) = self.relative_polar_coordinates_of(&other_asteroid);

            if (t1 - t2).abs() < EPSILON {
                current_group.push(other_asteroid.clone());
            } else {
                groups_by_relative_polar_coords.push(current_group);

                t1 = t2;
                current_group = vec![other_asteroid.clone()];
            }
        }

        if !current_group.is_empty() {
            groups_by_relative_polar_coords.push(current_group);
        }

        groups_by_relative_polar_coords
    }

    fn get_all_other_asteroid_coordinates(&self, map_data: &[Vec<char>]) -> Vec<(i32, i32)> {
        let mut other_asteroid_coordinates = Vec::new();

        for (row_index, row) in map_data.iter().enumerate() {
            for (column_index, &data) in row.iter().enumerate() {
                let x_location = column_index as i32;
                let y_location = row_index as i32;

                let is_same_location =
                    (x_location == self.x_location) && (y_location == self.y_location);
                let is_asteroid = data == '#';

                if is_asteroid && !is_same_location {
                    other_asteroid_coordinates.push((x_location, y_location));
                }
            }
        }

        other_asteroid_coordinates
    }

    fn distance_to(&self, other: &Asteroid) -> f64 {
        let (relative_x, relative_y) = self.relative_coordinates_of(other);

        ((relative_x.pow(2) + relative_y.pow(2)) as f64).sqrt()
    }

    // Returns the angle from self to other.
    // The angle is based on unit circle:
    //     (0, -1) -> 0 deg
    //     (1, 0) -> -90 deg
    //     (0, 1) -> -180 deg
    //     (-1, 0) -> -270 deg
    fn angle_to(&self, other: &Asteroid) -> f64 {
        let (relative_x, relative_y) = self.relative_coordinates_of(other);

        let (relative_x, relative_y) = (relative_x as f64, -relative_y as f64);

        let mut angle_to = relative_y.atan2(relative_x).to_degrees() - 90.0;

        if angle_to > 0.0 {
            angle_to -= 360.0;
        }

        if angle_to >= 360.0 {
            angle_to += 360.0;
        }

        angle_to
    }

    fn relative_coordinates_of(&self, other: &Asteroid) -> (i32, i32) {
        (
            other.x_location - self.x_location,
            other.y_location - self.y_location,
        )
    }

    fn relative_polar_coordinates_of(&self, other: &Asteroid) -> (f64, f64) {
        (self.distance_to(other), self.angle_to(other))
    }
}

#[derive(Debug, PartialEq)]
pub struct AsteroidMap {
    map: Vec<Asteroid>,
    original_data: Vec<Vec<char>>,
}

impl AsteroidMap {
    pub fn new(map_data: &[Vec<char>]) -> AsteroidMap {
        let mut temp_vec = Vec::new();
        let asteroid_coordinates = Self::get_asteroid_coordinates(map_data);

        for asteroid_coordinate in asteroid_coordinates.iter() {
            let asteroid =
                Asteroid::create_from_map(asteroid_coordinate.0, asteroid_coordinate.1, map_data);

            temp_vec.push(asteroid);
        }

        AsteroidMap {
            map: temp_vec,
            original_data: map_data.to_vec(),
        }
    }

    pub fn best_monitoring_station_location(&self) -> Asteroid {
        self.map
            .iter()
            .max_by_key(|asteroid| asteroid.number_of_asteroids_in_los)
            .unwrap()
            .clone()
    }

    pub fn nth_vaporized_asteroid_from_best_monitoring_station(
        &self,
        n: usize,
    ) -> Option<Asteroid> {
        let mut vaporized_count = 0;
        let mut current_vaporized_asteroid = None;

        let best_monitoring_station = self.best_monitoring_station_location();

        let mut asteroid_groups =
            best_monitoring_station.other_asteroids_grouped_by_relative_polar_coords;
        let number_of_groups = asteroid_groups.len();
        let mut group_index = 0;
        let max_index = asteroid_groups
            .iter()
            .max_by_key(|v| v.len())
            .unwrap()
            .len()
            * asteroid_groups.len();

        while (vaporized_count < n) && (group_index < max_index) {
            let asteroid_group = asteroid_groups
                .get_mut(group_index % number_of_groups)
                .unwrap();

            if !asteroid_group.is_empty() {
                vaporized_count += 1;

                current_vaporized_asteroid = Some(asteroid_group.remove(0));

                if vaporized_count != n {
                    current_vaporized_asteroid = None;
                }
            }

            group_index += 1;
        }

        current_vaporized_asteroid
    }

    fn get_asteroid_coordinates(map_data: &[Vec<char>]) -> Vec<(i32, i32)> {
        let mut other_asteroid_coordinates = Vec::new();

        for (row_index, row) in map_data.iter().enumerate() {
            for (column_index, &data) in row.iter().enumerate() {
                let x_location = column_index as i32;
                let y_location = row_index as i32;

                if data == '#' {
                    other_asteroid_coordinates.push((x_location, y_location));
                }
            }
        }

        other_asteroid_coordinates
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [[char; 5]; 5] = [
        ['.', '#', '.', '.', '#'],
        ['.', '.', '.', '.', '.'],
        ['#', '#', '#', '#', '#'],
        ['.', '.', '.', '.', '#'],
        ['.', '.', '.', '#', '#'],
    ];
    const TEST_DATA_ASTEROIDS: [Asteroid; 10] = [
        Asteroid::new(1, 0, 7),
        Asteroid::new(4, 0, 7),
        Asteroid::new(0, 2, 6),
        Asteroid::new(1, 2, 7),
        Asteroid::new(2, 2, 7),
        Asteroid::new(3, 2, 7),
        Asteroid::new(4, 2, 5),
        Asteroid::new(4, 3, 7),
        Asteroid::new(3, 4, 8),
        Asteroid::new(4, 4, 7),
    ];

    const LARGER_TEST_DATA: [[char; 10]; 10] = [
        ['.', '.', '.', '.', '.', '.', '#', '.', '#', '.'],
        ['#', '.', '.', '#', '.', '#', '.', '.', '.', '.'],
        ['.', '.', '#', '#', '#', '#', '#', '#', '#', '.'],
        ['.', '#', '.', '#', '.', '#', '#', '#', '.', '.'],
        ['.', '#', '.', '.', '#', '.', '.', '.', '.', '.'],
        ['.', '.', '#', '.', '.', '.', '.', '#', '.', '#'],
        ['#', '.', '.', '#', '.', '.', '.', '.', '#', '.'],
        ['.', '#', '#', '.', '#', '.', '.', '#', '#', '#'],
        ['#', '#', '.', '.', '.', '#', '.', '.', '#', '.'],
        ['.', '#', '.', '.', '.', '.', '#', '#', '#', '#'],
    ];

    const LARGEST_TEST_DATA: [[char; 20]; 20] = [
        [
            '.', '#', '.', '.', '#', '#', '.', '#', '#', '#', '.', '.', '.', '#', '#', '#', '#',
            '#', '#', '#',
        ],
        [
            '#', '#', '.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '.', '.',
            '#', '#', '.',
        ],
        [
            '.', '#', '.', '#', '#', '#', '#', '#', '#', '.', '#', '#', '#', '#', '#', '#', '#',
            '#', '.', '#',
        ],
        [
            '.', '#', '#', '#', '.', '#', '#', '#', '#', '#', '#', '#', '.', '#', '#', '#', '#',
            '.', '#', '.',
        ],
        [
            '#', '#', '#', '#', '#', '.', '#', '#', '.', '#', '.', '#', '#', '.', '#', '#', '#',
            '.', '#', '#',
        ],
        [
            '.', '.', '#', '#', '#', '#', '#', '.', '.', '#', '.', '#', '#', '#', '#', '#', '#',
            '#', '#', '#',
        ],
        [
            '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
            '#', '#', '#',
        ],
        [
            '#', '.', '#', '#', '#', '#', '.', '.', '.', '.', '#', '#', '#', '.', '#', '.', '#',
            '.', '#', '#',
        ],
        [
            '#', '#', '.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
            '#', '#', '#',
        ],
        [
            '#', '#', '#', '#', '#', '.', '#', '#', '.', '#', '#', '#', '.', '.', '#', '#', '#',
            '#', '.', '.',
        ],
        [
            '.', '.', '#', '#', '#', '#', '#', '#', '.', '.', '#', '#', '.', '#', '#', '#', '#',
            '#', '#', '#',
        ],
        [
            '#', '#', '#', '#', '.', '#', '#', '.', '#', '#', '#', '#', '.', '.', '.', '#', '#',
            '.', '.', '#',
        ],
        [
            '.', '#', '#', '#', '#', '#', '.', '.', '#', '.', '#', '#', '#', '#', '#', '#', '.',
            '#', '#', '#',
        ],
        [
            '#', '#', '.', '.', '.', '#', '.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
            '.', '.', '.',
        ],
        [
            '#', '.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#', '#', '#', '#',
            '#', '#', '#',
        ],
        [
            '.', '#', '#', '#', '#', '.', '#', '.', '#', '#', '#', '.', '#', '#', '#', '.', '#',
            '.', '#', '#',
        ],
        [
            '.', '.', '.', '.', '#', '#', '.', '#', '#', '.', '#', '#', '#', '.', '.', '#', '#',
            '#', '#', '#',
        ],
        [
            '.', '#', '.', '#', '.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '.',
            '#', '#', '#',
        ],
        [
            '#', '.', '#', '.', '#', '.', '#', '#', '#', '#', '#', '.', '#', '#', '#', '#', '.',
            '#', '#', '#',
        ],
        [
            '#', '#', '#', '.', '#', '#', '.', '#', '#', '#', '#', '.', '#', '#', '.', '#', '.',
            '.', '#', '#',
        ],
    ];

    #[test]
    pub fn test_create_from_map() {
        let test_data: Vec<Vec<char>> = TEST_DATA.iter().map(|arr| arr.to_vec()).collect();

        let expected = TEST_DATA_ASTEROIDS[0].clone();

        let mut result = Asteroid::create_from_map(1, 0, test_data.as_slice());

        result.other_asteroids_grouped_by_relative_polar_coords = Vec::new();

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_number_of_asteroids_in_los() {
        let test_data: Vec<Vec<char>> = TEST_DATA.iter().map(|arr| arr.to_vec()).collect();
        let asteroid = TEST_DATA_ASTEROIDS[6].clone();

        let expected = 5;

        let result = asteroid.number_of_asteroids_in_los(&test_data).len();

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_get_all_other_asteroid_coordinates() {
        let test_data: Vec<Vec<char>> = TEST_DATA.iter().map(|arr| arr.to_vec()).collect();
        let asteroid = TEST_DATA_ASTEROIDS[6].clone();

        let expected = vec![
            (1, 0),
            (4, 0),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (4, 3),
            (3, 4),
            (4, 4),
        ];

        let result = asteroid.get_all_other_asteroid_coordinates(&test_data);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_distance_to() {
        let first_asteroid = TEST_DATA_ASTEROIDS[9].clone();
        let second_asteroid = TEST_DATA_ASTEROIDS[0].clone();

        let expected = 5.0;

        let result = first_asteroid.distance_to(&second_asteroid);

        assert!(result - expected < EPSILON);
    }

    #[test]
    fn test_angle_to_90_degrees() {
        let asteroid_0 = TEST_DATA_ASTEROIDS[0].clone();
        let asteroid_1 = TEST_DATA_ASTEROIDS[1].clone();
        let asteroid_4 = TEST_DATA_ASTEROIDS[4].clone();
        let asteroid_5 = TEST_DATA_ASTEROIDS[5].clone();
        let asteroid_8 = TEST_DATA_ASTEROIDS[8].clone();
        let asteroid_9 = TEST_DATA_ASTEROIDS[9].clone();

        let expected_0 = 0.0;
        let result_0 = asteroid_8.angle_to(&asteroid_5);

        let expected_1 = -14.036243468;
        let result_1 = asteroid_8.angle_to(&asteroid_1);

        let expected_2 = -90.0;
        let result_2 = asteroid_8.angle_to(&asteroid_9);

        let expected_3 = -180.0;
        let result_3 = asteroid_5.angle_to(&asteroid_8);

        let expected_4 = -270.0;
        let result_4 = asteroid_5.angle_to(&asteroid_4);

        let expected_5 = -315.0;
        let result_5 = asteroid_5.angle_to(&asteroid_0);

        let expected_6 = -333.434948823;
        let result_6 = asteroid_8.angle_to(&asteroid_4);

        let expected_7 = -333.434948823;
        let result_7 = asteroid_8.angle_to(&asteroid_0);

        println!("Result 0: {}", result_0);
        println!("Result 1: {}", result_1);
        println!("Result 2: {}", result_2);
        println!("Result 3: {}", result_3);
        println!("Result 4: {}", result_4);
        println!("Result 5: {}", result_5);
        println!("Result 6: {}", result_6);
        println!("Result 7: {}", result_7);

        assert!(result_0 - expected_0 < EPSILON);
        assert!(result_1 - expected_1 < EPSILON);
        assert!(result_2 - expected_2 < EPSILON);
        assert!(result_3 - expected_3 < EPSILON);
        assert!(result_4 - expected_4 < EPSILON);
        assert!(result_5 - expected_5 < EPSILON);
        assert!(result_6 - expected_6 < EPSILON);
        assert!(result_7 - expected_7 < EPSILON);
    }

    #[test]
    pub fn test_relative_coordinates_of() {
        let first_asteroid = TEST_DATA_ASTEROIDS[4].clone();
        let second_asteroid = TEST_DATA_ASTEROIDS[1].clone();

        let expected = (2, -2);

        let result = first_asteroid.relative_coordinates_of(&second_asteroid);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_new_asteroid_map() {
        let test_asteroids: Vec<Asteroid> = TEST_DATA_ASTEROIDS
            .iter()
            .map(|asteroid| asteroid.clone())
            .collect();
        let test_data: Vec<Vec<char>> = TEST_DATA.iter().map(|arr| arr.to_vec()).collect();

        let expected = AsteroidMap {
            map: test_asteroids,
            original_data: test_data.clone(),
        };

        let mut result = AsteroidMap::new(&test_data);

        for result_asteroid in result.map.iter_mut() {
            result_asteroid.other_asteroids_grouped_by_relative_polar_coords = Vec::new();
        }

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_best_monitoring_station_location() {
        let test_data: Vec<Vec<char>> = TEST_DATA.iter().map(|arr| arr.to_vec()).collect();
        let map = AsteroidMap::new(&test_data);

        let expected = TEST_DATA_ASTEROIDS[8].clone();

        let mut result = map.best_monitoring_station_location();

        result.other_asteroids_grouped_by_relative_polar_coords = Vec::new();

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_large_best_monitoring_station_location() {
        let test_data: Vec<Vec<char>> = LARGER_TEST_DATA.iter().map(|arr| arr.to_vec()).collect();
        let map = AsteroidMap::new(&test_data);

        let expected = Asteroid::new(5, 8, 33);

        let mut result = map.best_monitoring_station_location();

        result.other_asteroids_grouped_by_relative_polar_coords = Vec::new();

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_largest_best_monitoring_station_location() {
        let test_data: Vec<Vec<char>> = LARGEST_TEST_DATA.iter().map(|arr| arr.to_vec()).collect();
        let map = AsteroidMap::new(&test_data);

        let expected = Asteroid::new(11, 13, 210);

        let mut result = map.best_monitoring_station_location();

        result.other_asteroids_grouped_by_relative_polar_coords = Vec::new();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_largest_nth_vaporized_asteroid_from_best_monitoring_station() {
        let test_data: Vec<Vec<char>> = LARGEST_TEST_DATA.iter().map(|arr| arr.to_vec()).collect();
        let map = AsteroidMap::new(&test_data);

        let expected_coords_3 = (12, 2);
        let expected_coords_200 = (8, 2);
        let expected_coords_299 = (11, 1);
        let expected_300 = None;

        let result_3 = map
            .nth_vaporized_asteroid_from_best_monitoring_station(3)
            .unwrap();
        let result_coords_3 = (result_3.x_location, result_3.y_location);
        let result_200 = map
            .nth_vaporized_asteroid_from_best_monitoring_station(200)
            .unwrap();
        let result_coords_200 = (result_200.x_location, result_200.y_location);
        let result_299 = map
            .nth_vaporized_asteroid_from_best_monitoring_station(299)
            .unwrap();
        let result_coords_299 = (result_299.x_location, result_299.y_location);
        let result_300 = map.nth_vaporized_asteroid_from_best_monitoring_station(300);

        assert_eq!(result_coords_3, expected_coords_3);
        assert_eq!(result_coords_200, expected_coords_200);
        assert_eq!(result_coords_299, expected_coords_299);
        assert_eq!(result_300, expected_300);
    }
}
