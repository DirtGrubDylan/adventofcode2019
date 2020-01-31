use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Asteroid {
    x_location: i32,
    y_location: i32,
    number_of_asteroids_in_los: i32,
}

impl Asteroid {
    const fn new(x_location: i32, y_location: i32, number_of_asteroids_in_los: i32) -> Asteroid {
        Asteroid {
            x_location,
            y_location,
            number_of_asteroids_in_los,
        }
    }

    pub fn create_from_map(x_location: i32, y_location: i32, map_data: &[Vec<char>]) -> Asteroid {
        Asteroid {
            x_location,
            y_location,
            number_of_asteroids_in_los: 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AsteroidMap {
    map: HashSet<Asteroid>,
    original_data: Vec<Vec<char>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_LOCATION_SOLUTION: (i32, i32) = (3, 4);
    const TEST_DATA_ASTEROIDS_DETECTED_SOLUTION: i32 = 8;
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

    const LARGER_TEST_DATA_LOCATION_SOLUTION: (i32, i32) = (5, 8);
    const LARGER_TEST_DATA_ASTEROIDS_DETECTED_SOLUTION: i32 = 33;
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
}
