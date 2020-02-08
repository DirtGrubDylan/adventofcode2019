#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Asteroid {
    pub x_location: i32,
    pub y_location: i32,
    pub number_of_asteroids_in_los: i32,
}

impl Asteroid {
    const fn new(x_location: i32, y_location: i32, number_of_asteroids_in_los: i32) -> Asteroid {
        Asteroid {
            x_location,
            y_location,
            number_of_asteroids_in_los,
        }
    }

    fn create_from_map(x_location: i32, y_location: i32, map_data: &[Vec<char>]) -> Asteroid {
        let temp_asteroid = Asteroid::new(x_location, y_location, 0);

        let number_of_asteroids_in_los = temp_asteroid.number_of_asteroids_in_los(map_data);

        Asteroid {
            x_location,
            y_location,
            number_of_asteroids_in_los,
        }
    }

    fn number_of_asteroids_in_los(&self, map_data: &[Vec<char>]) -> i32 {
        let mut number_of_asteroids_in_los = 0;

        let mut other_asteroids: Vec<Asteroid> = self
            .get_all_other_asteroid_coordinates(map_data)
            .iter()
            .map(|&(x, y)| Asteroid::new(x, y, 0))
            .collect();

        other_asteroids.sort_by_key(|asteroid| self.manhattan_distance_to(asteroid));

        while !other_asteroids.is_empty() {
            let first_asteroid = other_asteroids[0].clone();

            number_of_asteroids_in_los += 1;

            other_asteroids
                .retain(|second_asteroid| !self.on_same_los(&first_asteroid, second_asteroid));
        }

        number_of_asteroids_in_los
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

    fn manhattan_distance_to(&self, other: &Asteroid) -> i32 {
        let (relative_x, relative_y) = self.relative_coordinates_of(other);

        relative_x.abs() + relative_y.abs()
    }

    fn on_same_los(&self, first: &Asteroid, second: &Asteroid) -> bool {
        let first_relative_end_point = self.relative_coordinates_of(first);
        let second_relative_end_point = self.relative_coordinates_of(second);

        let cross_product = first_relative_end_point.0 * second_relative_end_point.1
            - first_relative_end_point.1 * second_relative_end_point.0;

        let x_values_have_same_signum =
            first_relative_end_point.0.signum() == second_relative_end_point.0.signum();

        let y_values_have_same_signum =
            first_relative_end_point.1.signum() == second_relative_end_point.1.signum();

        cross_product == 0 && x_values_have_same_signum && y_values_have_same_signum
    }

    fn relative_coordinates_of(&self, other: &Asteroid) -> (i32, i32) {
        (
            other.x_location - self.x_location,
            other.y_location - self.y_location,
        )
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

    #[test]
    pub fn test_create_from_map() {
        let test_data: Vec<Vec<char>> = TEST_DATA.iter().map(|arr| arr.to_vec()).collect();

        let expected = TEST_DATA_ASTEROIDS[0].clone();

        let result = Asteroid::create_from_map(1, 0, test_data.as_slice());

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_number_of_asteroids_in_los() {
        let test_data: Vec<Vec<char>> = TEST_DATA.iter().map(|arr| arr.to_vec()).collect();
        let asteroid = TEST_DATA_ASTEROIDS[6].clone();

        let expected = 5;

        let result = asteroid.number_of_asteroids_in_los(&test_data);

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
    pub fn test_manhattan_distance_to() {
        let first_asteroid = TEST_DATA_ASTEROIDS[8].clone();
        let second_asteroid = TEST_DATA_ASTEROIDS[0].clone();

        let expected = 6;

        let result = first_asteroid.manhattan_distance_to(&second_asteroid);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_on_same_los() {
        let asteroid = TEST_DATA_ASTEROIDS[8].clone();
        let seen_asteroid = TEST_DATA_ASTEROIDS[4].clone();
        let blocked_asteroid = TEST_DATA_ASTEROIDS[0].clone();

        assert!(asteroid.on_same_los(&seen_asteroid, &blocked_asteroid));
    }

    #[test]
    pub fn test_not_on_same_los() {
        let asteroid = TEST_DATA_ASTEROIDS[6].clone();
        let seen_asteroid = TEST_DATA_ASTEROIDS[1].clone();
        let other_seen_asteroid = TEST_DATA_ASTEROIDS[7].clone();

        assert!(!asteroid.on_same_los(&seen_asteroid, &other_seen_asteroid));
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

        let result = AsteroidMap::new(&test_data);

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_best_monitoring_station_location() {
        let test_data: Vec<Vec<char>> = TEST_DATA.iter().map(|arr| arr.to_vec()).collect();
        let map = AsteroidMap::new(&test_data);

        let expected = TEST_DATA_ASTEROIDS[8].clone();

        let result = map.best_monitoring_station_location();

        assert_eq!(result, expected);
    }

    #[test]
    pub fn test_large_best_monitoring_station_location() {
        let test_data: Vec<Vec<char>> = LARGER_TEST_DATA.iter().map(|arr| arr.to_vec()).collect();
        let map = AsteroidMap::new(&test_data);

        let expected = Asteroid::new(5, 8, 33);

        let result = map.best_monitoring_station_location();

        assert_eq!(result, expected);
    }
}
