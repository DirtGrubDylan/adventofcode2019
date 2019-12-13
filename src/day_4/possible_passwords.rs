#[derive(Debug, PartialEq)]
pub struct PossiblePasswords {
    start_range: i32,
    end_range: i32,
    extended_end_range: i32,
    increasing_digits_in_range: i32,
    increasing_digits_in_extended_range: i32,
    double_digits_in_range: i32,
    double_digits_in_extended_range: i32,
}

impl PossiblePasswords {
    pub fn new(start_range_significant_digit: usize, magnitude_of_ten: usize) -> PossiblePasswords {
        let start_range_significant_digit = start_range_significant_digit as i32;
        let magnitude_of_ten = magnitude_of_ten as u32;

        let power_of_ten = 10_i32.pow(magnitude_of_ten);
        let start_range = start_range_significant_digit * power_of_ten;

        PossiblePasswords {
            start_range: start_range,
            end_range: start_range + power_of_ten,
            extended_end_range: 10_i32.pow(magnitude_of_ten + 1),
            increasing_digits_in_range: 0,
            increasing_digits_in_extended_range: 0,
            double_digits_in_range: 0,
            double_digits_in_extended_range: 0,
        }
    }

    #[allow(dead_code)]
    fn new_all(
        start_range: i32,
        end_range: i32,
        extended_end_range: i32,
        increasing_digits_in_range: i32,
        increasing_digits_in_extended_range: i32,
        double_digits_in_range: i32,
        double_digits_in_extended_range: i32,
    ) -> PossiblePasswords {
        PossiblePasswords {
            start_range: start_range,
            end_range: end_range,
            extended_end_range: extended_end_range,
            increasing_digits_in_range: increasing_digits_in_range,
            increasing_digits_in_extended_range: increasing_digits_in_extended_range,
            double_digits_in_range: double_digits_in_range,
            double_digits_in_extended_range: double_digits_in_extended_range,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PossiblePasswordMap {
    // Sorted by range significant digit by magnitute of ten.
    // (e.g. the element (2, 2) represents the range [200, 300))
    map: Vec<Vec<PossiblePasswords>>,
}

impl PossiblePasswordMap {
    pub fn new(password_length: usize) -> PossiblePasswordMap {
        let mut temp_map = Vec::new();

        // fill temp map with default values
        for start_range_significant_digit in 0..10 {
            let mut temp_row = Vec::new();

            for magnitude_of_ten in 0..password_length {
                temp_row.push(PossiblePasswords::new(
                    start_range_significant_digit,
                    magnitude_of_ten,
                ));
            }

            temp_map.push(temp_row);
        }

        let mut possible_password_map = PossiblePasswordMap { map: temp_map };

        // fill temp map with actual values going backwards
        for start_range_significant_digit in (0..10).rev() {
            for magnitude_of_ten in 0..password_length {
                let actual_possible_passwords = possible_password_map
                    .get_possible_passwords_for(start_range_significant_digit, magnitude_of_ten);

                possible_password_map.map[start_range_significant_digit][magnitude_of_ten] =
                    actual_possible_passwords;
            }
        }

        possible_password_map
    }

    #[allow(dead_code)]
    fn get_possible_passwords_for(
        &self,
        start_range_significant_digit: usize,
        magnitude_of_ten: usize,
    ) -> PossiblePasswords {
        let increasing_digits_in_range = self
            .get_number_of_increasing_digits_for(start_range_significant_digit, magnitude_of_ten);

        let increasing_digits_in_extended_range = match start_range_significant_digit {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => {
                increasing_digits_in_range
                    + self.map[start_range_significant_digit + 1][magnitude_of_ten]
                        .increasing_digits_in_extended_range
            }
            _ => increasing_digits_in_range,
        };

        let double_digits_in_range =
            self.get_number_of_double_digits_for(start_range_significant_digit, magnitude_of_ten);

        let double_digits_in_extended_range = match start_range_significant_digit {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => {
                double_digits_in_range
                    + self.map[start_range_significant_digit + 1][magnitude_of_ten]
                        .double_digits_in_extended_range
            }
            _ => double_digits_in_range,
        };

        let mut possible_passwords =
            PossiblePasswords::new(start_range_significant_digit, magnitude_of_ten);

        possible_passwords.increasing_digits_in_range = increasing_digits_in_range;
        possible_passwords.increasing_digits_in_extended_range =
            increasing_digits_in_extended_range;
        possible_passwords.double_digits_in_range = double_digits_in_range;
        possible_passwords.double_digits_in_extended_range = double_digits_in_extended_range;

        possible_passwords
    }

    fn get_number_of_increasing_digits_for(
        &self,
        start_range_significant_digit: usize,
        magnitude_of_ten: usize,
    ) -> i32 {
        if (start_range_significant_digit == 9) || (magnitude_of_ten == 0) {
            return 1;
        }

        let number_of_increasing_digits_in_extend_range_one_magnitude_less = self.map
            [start_range_significant_digit][magnitude_of_ten - 1]
            .increasing_digits_in_extended_range;

        number_of_increasing_digits_in_extend_range_one_magnitude_less
    }

    fn get_number_of_double_digits_for(
        &self,
        start_range_significant_digit: usize,
        magnitude_of_ten: usize,
    ) -> i32 {
        if magnitude_of_ten == 0 {
            return 0;
        }

        let number_of_digits_contain_start_range_significant_digit = self.map
            [start_range_significant_digit][magnitude_of_ten - 1]
            .increasing_digits_in_range;

        if start_range_significant_digit < 9 {
            let number_of_double_digits_so_far_above_range_one_magnitude_less = self.map
                [start_range_significant_digit + 1][magnitude_of_ten - 1]
                .double_digits_in_extended_range;

            number_of_digits_contain_start_range_significant_digit
                + number_of_double_digits_so_far_above_range_one_magnitude_less
        } else {
            number_of_digits_contain_start_range_significant_digit
        }
    }

    pub fn number_of_double_digit_values_up_to(
        &self,
        upper_bound_significant_digit: usize,
        upper_bound_magnitude_of_ten: usize,
    ) -> i32 {
        let all_double_digits_in_full_range =
            self.map[0][upper_bound_magnitude_of_ten].double_digits_in_extended_range;

        let all_double_digits_in_range_including_and_above = self.map
            [upper_bound_significant_digit][upper_bound_magnitude_of_ten]
            .double_digits_in_extended_range;

        all_double_digits_in_full_range - all_double_digits_in_range_including_and_above
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_possible_passwords() {
        let expected = PossiblePasswords {
            start_range: 200,
            end_range: 300,
            extended_end_range: 1000,
            increasing_digits_in_range: 0,
            increasing_digits_in_extended_range: 0,
            double_digits_in_range: 0,
            double_digits_in_extended_range: 0,
        };

        let result = PossiblePasswords::new(2, 2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_possible_password_map() {
        let map = vec![
            vec![
                PossiblePasswords::new_all(0, 1, 10, 1, 10, 0, 0),
                PossiblePasswords::new_all(0, 10, 100, 10, 55, 1, 10),
                PossiblePasswords::new_all(0, 100, 1000, 55, 210, 19, 100),
            ],
            vec![
                PossiblePasswords::new_all(1, 2, 10, 1, 9, 0, 0),
                PossiblePasswords::new_all(10, 20, 100, 9, 45, 1, 9),
                PossiblePasswords::new_all(100, 200, 1000, 45, 155, 17, 81),
            ],
            vec![
                PossiblePasswords::new_all(2, 3, 10, 1, 8, 0, 0),
                PossiblePasswords::new_all(20, 30, 100, 8, 36, 1, 8),
                PossiblePasswords::new_all(200, 300, 1000, 36, 110, 15, 64),
            ],
            vec![
                PossiblePasswords::new_all(3, 4, 10, 1, 7, 0, 0),
                PossiblePasswords::new_all(30, 40, 100, 7, 28, 1, 7),
                PossiblePasswords::new_all(300, 400, 1000, 28, 74, 13, 49),
            ],
            vec![
                PossiblePasswords::new_all(4, 5, 10, 1, 6, 0, 0),
                PossiblePasswords::new_all(40, 50, 100, 6, 21, 1, 6),
                PossiblePasswords::new_all(400, 500, 1000, 21, 46, 11, 36),
            ],
            vec![
                PossiblePasswords::new_all(5, 6, 10, 1, 5, 0, 0),
                PossiblePasswords::new_all(50, 60, 100, 5, 15, 1, 5),
                PossiblePasswords::new_all(500, 600, 1000, 15, 35, 9, 25),
            ],
            vec![
                PossiblePasswords::new_all(6, 7, 10, 1, 4, 0, 0),
                PossiblePasswords::new_all(60, 70, 100, 4, 10, 1, 4),
                PossiblePasswords::new_all(600, 700, 1000, 10, 20, 7, 16),
            ],
            vec![
                PossiblePasswords::new_all(7, 8, 10, 1, 3, 0, 0),
                PossiblePasswords::new_all(70, 80, 100, 3, 6, 1, 3),
                PossiblePasswords::new_all(700, 800, 1000, 6, 10, 5, 9),
            ],
            vec![
                PossiblePasswords::new_all(8, 9, 10, 1, 2, 0, 0),
                PossiblePasswords::new_all(80, 90, 100, 2, 3, 1, 2),
                PossiblePasswords::new_all(800, 900, 1000, 3, 4, 3, 4),
            ],
            vec![
                PossiblePasswords::new_all(9, 10, 10, 1, 1, 0, 0),
                PossiblePasswords::new_all(90, 100, 100, 1, 1, 1, 1),
                PossiblePasswords::new_all(900, 1000, 1000, 1, 1, 1, 1),
            ],
        ];

        let expected = PossiblePasswordMap { map: map };

        let result = PossiblePasswordMap::new(3);

        assert_eq!(result.map[9][1], expected.map[9][1]);
        // assert_eq!(result.map[0][2], expected.map[0][2]);
        // assert_eq!(result.map[0][3], expected.map[0][3]);
    }

    #[test]
    fn test_get_possible_passwords_for_password_map_success() {
        let map = vec![
            vec![
                PossiblePasswords::new_all(0, 1, 10, 1, 10, 0, 0),
                PossiblePasswords::new_all(0, 10, 100, 10, 55, 1, 10),
            ],
            vec![
                PossiblePasswords::new_all(1, 2, 10, 1, 9, 0, 0),
                PossiblePasswords::new_all(10, 20, 100, 9, 45, 1, 9),
            ],
            vec![
                PossiblePasswords::new_all(2, 3, 10, 1, 8, 0, 0),
                PossiblePasswords::new_all(20, 30, 100, 8, 36, 1, 8),
            ],
            vec![
                PossiblePasswords::new_all(3, 4, 10, 1, 7, 0, 0),
                PossiblePasswords::new_all(30, 40, 100, 7, 28, 1, 7),
                PossiblePasswords::new_all(300, 400, 1000, 28, 74, 13, 49),
            ],
            vec![
                PossiblePasswords::new_all(4, 5, 10, 1, 6, 0, 0),
                PossiblePasswords::new_all(40, 50, 100, 6, 21, 1, 6),
                PossiblePasswords::new_all(400, 500, 1000, 21, 46, 11, 36),
            ],
            vec![
                PossiblePasswords::new_all(5, 6, 10, 1, 5, 0, 0),
                PossiblePasswords::new_all(50, 60, 100, 5, 15, 1, 5),
                PossiblePasswords::new_all(500, 600, 1000, 15, 35, 9, 25),
            ],
            vec![
                PossiblePasswords::new_all(6, 7, 10, 1, 4, 0, 0),
                PossiblePasswords::new_all(60, 70, 100, 4, 10, 1, 4),
                PossiblePasswords::new_all(600, 700, 1000, 10, 20, 7, 16),
            ],
            vec![
                PossiblePasswords::new_all(7, 8, 10, 1, 3, 0, 0),
                PossiblePasswords::new_all(70, 80, 100, 3, 6, 1, 3),
                PossiblePasswords::new_all(700, 800, 1000, 6, 10, 5, 9),
            ],
            vec![
                PossiblePasswords::new_all(8, 9, 10, 1, 2, 0, 0),
                PossiblePasswords::new_all(80, 90, 100, 2, 3, 1, 2),
                PossiblePasswords::new_all(800, 900, 1000, 3, 4, 3, 4),
            ],
            vec![
                PossiblePasswords::new_all(9, 10, 10, 1, 1, 0, 0),
                PossiblePasswords::new_all(90, 100, 100, 1, 1, 1, 1),
                PossiblePasswords::new_all(900, 1000, 1000, 1, 1, 1, 1),
            ],
        ];

        let possible_password_map = PossiblePasswordMap { map: map };

        let expected = PossiblePasswords {
            start_range: 200,
            end_range: 300,
            extended_end_range: 1000,
            increasing_digits_in_range: 36,
            increasing_digits_in_extended_range: 110,
            double_digits_in_range: 15,
            double_digits_in_extended_range: 64,
        };

        let result = possible_password_map.get_possible_passwords_for(2, 2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_number_of_increasing_digits_for_password_map_success() {
        let map = vec![
            vec![
                PossiblePasswords::new_all(0, 1, 10, 1, 10, 0, 0),
                PossiblePasswords::new_all(0, 10, 100, 10, 55, 1, 10),
            ],
            vec![
                PossiblePasswords::new_all(1, 2, 10, 1, 9, 0, 0),
                PossiblePasswords::new_all(10, 20, 100, 9, 45, 1, 9),
            ],
            vec![
                PossiblePasswords::new_all(2, 3, 10, 1, 8, 0, 0),
                PossiblePasswords::new_all(20, 30, 100, 8, 36, 1, 8),
            ],
            vec![
                PossiblePasswords::new_all(3, 4, 10, 1, 7, 0, 0),
                PossiblePasswords::new_all(30, 40, 100, 7, 28, 1, 7),
                PossiblePasswords::new_all(300, 400, 1000, 28, 74, 13, 49),
            ],
            vec![
                PossiblePasswords::new_all(4, 5, 10, 1, 6, 0, 0),
                PossiblePasswords::new_all(40, 50, 100, 6, 21, 1, 6),
                PossiblePasswords::new_all(400, 500, 1000, 21, 46, 11, 36),
            ],
            vec![
                PossiblePasswords::new_all(5, 6, 10, 1, 5, 0, 0),
                PossiblePasswords::new_all(50, 60, 100, 5, 15, 1, 5),
                PossiblePasswords::new_all(500, 600, 1000, 15, 35, 9, 25),
            ],
            vec![
                PossiblePasswords::new_all(6, 7, 10, 1, 4, 0, 0),
                PossiblePasswords::new_all(60, 70, 100, 4, 10, 1, 4),
                PossiblePasswords::new_all(600, 700, 1000, 10, 20, 7, 16),
            ],
            vec![
                PossiblePasswords::new_all(7, 8, 10, 1, 3, 0, 0),
                PossiblePasswords::new_all(70, 80, 100, 3, 6, 1, 3),
                PossiblePasswords::new_all(700, 800, 1000, 6, 10, 5, 9),
            ],
            vec![
                PossiblePasswords::new_all(8, 9, 10, 1, 2, 0, 0),
                PossiblePasswords::new_all(80, 90, 100, 2, 3, 1, 2),
                PossiblePasswords::new_all(800, 900, 1000, 3, 4, 3, 4),
            ],
            vec![
                PossiblePasswords::new_all(9, 10, 10, 1, 1, 0, 0),
                PossiblePasswords::new_all(90, 100, 100, 1, 1, 1, 1),
                PossiblePasswords::new_all(900, 1000, 1000, 1, 1, 1, 1),
            ],
        ];

        let possible_password_map = PossiblePasswordMap { map: map };

        let expected_1 = 36;
        let result_1 = possible_password_map.get_number_of_increasing_digits_for(2, 2);

        let expected_2 = 1;
        let result_2 = possible_password_map.get_number_of_increasing_digits_for(3, 0);

        let expected_3 = 1;
        let result_3 = possible_password_map.get_number_of_increasing_digits_for(9, 0);

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
    }

    #[test]
    fn test_get_number_of_double_digits_for_password_map_success() {
        let map = vec![
            vec![
                PossiblePasswords::new_all(0, 1, 10, 1, 10, 0, 0),
                PossiblePasswords::new_all(0, 10, 100, 10, 55, 1, 10),
            ],
            vec![
                PossiblePasswords::new_all(1, 2, 10, 1, 9, 0, 0),
                PossiblePasswords::new_all(10, 20, 100, 9, 45, 1, 9),
            ],
            vec![
                PossiblePasswords::new_all(2, 3, 10, 1, 8, 0, 0),
                PossiblePasswords::new_all(20, 30, 100, 8, 36, 1, 8),
            ],
            vec![
                PossiblePasswords::new_all(3, 4, 10, 1, 7, 0, 0),
                PossiblePasswords::new_all(30, 40, 100, 7, 28, 1, 7),
                PossiblePasswords::new_all(300, 400, 1000, 28, 74, 13, 49),
            ],
            vec![
                PossiblePasswords::new_all(4, 5, 10, 1, 6, 0, 0),
                PossiblePasswords::new_all(40, 50, 100, 6, 21, 1, 6),
                PossiblePasswords::new_all(400, 500, 1000, 21, 46, 11, 36),
            ],
            vec![
                PossiblePasswords::new_all(5, 6, 10, 1, 5, 0, 0),
                PossiblePasswords::new_all(50, 60, 100, 5, 15, 1, 5),
                PossiblePasswords::new_all(500, 600, 1000, 15, 35, 9, 25),
            ],
            vec![
                PossiblePasswords::new_all(6, 7, 10, 1, 4, 0, 0),
                PossiblePasswords::new_all(60, 70, 100, 4, 10, 1, 4),
                PossiblePasswords::new_all(600, 700, 1000, 10, 20, 7, 16),
            ],
            vec![
                PossiblePasswords::new_all(7, 8, 10, 1, 3, 0, 0),
                PossiblePasswords::new_all(70, 80, 100, 3, 6, 1, 3),
                PossiblePasswords::new_all(700, 800, 1000, 6, 10, 5, 9),
            ],
            vec![
                PossiblePasswords::new_all(8, 9, 10, 1, 2, 0, 0),
                PossiblePasswords::new_all(80, 90, 100, 2, 3, 1, 2),
                PossiblePasswords::new_all(800, 900, 1000, 3, 4, 3, 4),
            ],
            vec![
                PossiblePasswords::new_all(9, 10, 10, 1, 1, 0, 0),
                PossiblePasswords::new_all(90, 100, 100, 1, 1, 1, 1),
                PossiblePasswords::new_all(900, 1000, 1000, 1, 1, 1, 1),
            ],
        ];

        let possible_password_map = PossiblePasswordMap { map: map };

        let expected_1 = 15;
        let result_1 = possible_password_map.get_number_of_double_digits_for(2, 2);

        let expected_2 = 0;
        let result_2 = possible_password_map.get_number_of_double_digits_for(3, 0);

        let expected_3 = 0;
        let result_3 = possible_password_map.get_number_of_double_digits_for(9, 0);

        let expected_4 = 1;
        let result_4 = possible_password_map.get_number_of_double_digits_for(9, 1);

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
        assert_eq!(result_4, expected_4);
    }

    #[test]
    fn test_number_of_double_digit_values_up_to() {
        let possible_password_map = PossiblePasswordMap::new(3);

        let expected_1 = 36;
        // From 0 to 200
        let result_1 = possible_password_map.number_of_double_digit_values_up_to(2, 2);

        let expected_2 = 1;
        // From 0 to 10
        let result_2 = possible_password_map.number_of_double_digit_values_up_to(1, 1);

        let expected_3 = 0;
        // From 0 to 0
        let result_3 = possible_password_map.number_of_double_digit_values_up_to(0, 2);

        let expected_4 = 0;
        // From 0 to 7
        let result_4 = possible_password_map.number_of_double_digit_values_up_to(7, 0);

        let expected_5 = 6;
        // From 0 to 60
        let result_5 = possible_password_map.number_of_double_digit_values_up_to(6, 1);

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
        assert_eq!(result_4, expected_4);
        assert_eq!(result_5, expected_5);
    }
}
