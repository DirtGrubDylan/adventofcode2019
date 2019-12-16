#[derive(Debug, PartialEq)]
pub struct PossiblePasswords {
    pub start_range: i32,
    pub end_range: i32,
    pub extended_end_range: i32,
    pub increasing_digits_in_range: i32,
    pub increasing_digits_in_extended_range: i32,
    pub double_digits_in_range: i32,
    pub double_digits_in_extended_range: i32,
    pub triple_digits_in_range: i32,
    pub triple_digits_in_extended_range: i32,
}

impl PossiblePasswords {
    pub fn new(starting_digit: usize, length: usize) -> PossiblePasswords {
        let starting_digit = starting_digit as i32;
        let magnitude_of_ten = length as u32;

        let power_of_ten = 10_i32.pow(magnitude_of_ten);
        let start_range = starting_digit * power_of_ten;

        PossiblePasswords {
            start_range: start_range,
            end_range: start_range + power_of_ten,
            extended_end_range: 10_i32.pow(magnitude_of_ten + 1),
            increasing_digits_in_range: 0,
            increasing_digits_in_extended_range: 0,
            double_digits_in_range: 0,
            double_digits_in_extended_range: 0,
            triple_digits_in_range: 0,
            triple_digits_in_extended_range: 0,
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
        triple_digits_in_range: i32,
        triple_digits_in_extended_range: i32,
    ) -> PossiblePasswords {
        PossiblePasswords {
            start_range: start_range,
            end_range: end_range,
            extended_end_range: extended_end_range,
            increasing_digits_in_range: increasing_digits_in_range,
            increasing_digits_in_extended_range: increasing_digits_in_extended_range,
            double_digits_in_range: double_digits_in_range,
            double_digits_in_extended_range: double_digits_in_extended_range,
            triple_digits_in_range: triple_digits_in_range,
            triple_digits_in_extended_range: triple_digits_in_extended_range,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PossiblePasswordMap {
    // Sorted by range significant digit by magnitute of ten.
    // (e.g. the element (2, 2) represents the range [200, 300))
    pub map: Vec<Vec<PossiblePasswords>>,
}

impl PossiblePasswordMap {
    pub fn new(password_length: usize) -> PossiblePasswordMap {
        let mut temp_map = Vec::new();

        // fill temp map with default values
        for starting_digit in 0..10 {
            let mut temp_row = Vec::new();

            for temp_length in 0..password_length {
                temp_row.push(PossiblePasswords::new(starting_digit, temp_length));
            }

            temp_map.push(temp_row);
        }

        let mut possible_password_map = PossiblePasswordMap { map: temp_map };

        // fill temp map with actual values going backwards
        for starting_digit in (0..10).rev() {
            for temp_length in 0..password_length {
                let actual_possible_passwords =
                    possible_password_map.get_possible_passwords_for(starting_digit, temp_length);

                possible_password_map.map[starting_digit][temp_length] = actual_possible_passwords;
            }
        }

        possible_password_map
    }

    fn get_possible_passwords_for(
        &self,
        starting_digit: usize,
        length: usize,
    ) -> PossiblePasswords {
        let increasing_digits_in_range =
            self.get_number_of_increasing_digits_for(starting_digit, length);

        let increasing_digits_in_extended_range = match starting_digit {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => {
                increasing_digits_in_range
                    + self.map[starting_digit + 1][length].increasing_digits_in_extended_range
            }
            _ => increasing_digits_in_range,
        };

        let double_digits_in_range = self.get_number_of_double_digits_for(starting_digit, length);

        let double_digits_in_extended_range = match starting_digit {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => {
                double_digits_in_range
                    + self.map[starting_digit + 1][length].double_digits_in_extended_range
            }
            _ => double_digits_in_range,
        };

        let triple_digits_in_range = self.get_number_of_triple_digits_for(starting_digit, length);

        let triple_digits_in_extended_range = match starting_digit {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => {
                triple_digits_in_range
                    + self.map[starting_digit + 1][length].triple_digits_in_extended_range
            }
            _ => triple_digits_in_range,
        };

        let mut possible_passwords = PossiblePasswords::new(starting_digit, length);

        possible_passwords.increasing_digits_in_range = increasing_digits_in_range;
        possible_passwords.increasing_digits_in_extended_range =
            increasing_digits_in_extended_range;
        possible_passwords.double_digits_in_range = double_digits_in_range;
        possible_passwords.double_digits_in_extended_range = double_digits_in_extended_range;
        possible_passwords.triple_digits_in_range = triple_digits_in_range;
        possible_passwords.triple_digits_in_extended_range = triple_digits_in_extended_range;

        possible_passwords
    }

    fn get_number_of_increasing_digits_for(&self, starting_digit: usize, length: usize) -> i32 {
        if (starting_digit == 9) || (length == 0) {
            return 1;
        }

        let number_of_increasing_digits_in_extend_range_one_magnitude_less =
            self.map[starting_digit][length - 1].increasing_digits_in_extended_range;

        number_of_increasing_digits_in_extend_range_one_magnitude_less
    }

    fn get_number_of_double_digits_for(&self, starting_digit: usize, length: usize) -> i32 {
        if length == 0 {
            return 0;
        } else if starting_digit == 0 {
            return self.map[starting_digit][length - 1].double_digits_in_extended_range;
        }

        let number_of_digits_contain_starting_digit =
            self.map[starting_digit][length - 1].increasing_digits_in_range;

        if starting_digit < 9 {
            let number_of_double_digits_so_far_above_range_one_magnitude_less =
                self.map[starting_digit + 1][length - 1].double_digits_in_extended_range;

            number_of_digits_contain_starting_digit
                + number_of_double_digits_so_far_above_range_one_magnitude_less
        } else {
            number_of_digits_contain_starting_digit
        }
    }

    pub fn get_number_of_triple_digits_for(&self, starting_digit: usize, length: usize) -> i32 {
        if length < 2 {
            return 0;
        } else if starting_digit == 0 {
            return self.map[starting_digit][length - 1].triple_digits_in_extended_range;
        }

        let all_increasing_digits_with_starting_digit_two_less_length =
            self.map[starting_digit][0..(length - 1)].iter()
                .fold(0, |acc, pp| acc + pp.increasing_digits_in_range - pp.double_digits_in_range);

        if starting_digit < 9 {
            let all_triple_digits_above_starting_digit_one_less_length =
                self.map[starting_digit + 1][0..length].iter()
                    .fold(0, |acc, pp| acc + pp.triple_digits_in_extended_range);

            let flipped_triple_digits_to_double_digits =
                self.map[starting_digit + 1][length - 2].triple_digits_in_extended_range;

            all_increasing_digits_with_starting_digit_two_less_length
                + all_triple_digits_above_starting_digit_one_less_length
                - flipped_triple_digits_to_double_digits
        } else {
            all_increasing_digits_with_starting_digit_two_less_length
        }
    }

    pub fn number_of_double_digit_values_between(
        &self,
        lower_bound_starting_digit: usize,
        upper_bound_starting_digit: usize,
        length: usize,
    ) -> i32 {
        let upper_bound =
            self.map[lower_bound_starting_digit][length].double_digits_in_extended_range;

        let lower_bound =
            self.map[upper_bound_starting_digit][length].double_digits_in_extended_range;

        upper_bound - lower_bound
    }

    pub fn number_of_triple_digit_values_between(
        &self,
        lower_bound_starting_digit: usize,
        upper_bound_starting_digit: usize,
        length: usize,
    ) -> i32 {
        let upper_bound =
            self.map[lower_bound_starting_digit][length].triple_digits_in_extended_range;

        let lower_bound =
            self.map[upper_bound_starting_digit][length].triple_digits_in_extended_range;

        upper_bound - lower_bound
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
            triple_digits_in_range: 0,
            triple_digits_in_extended_range: 0,
        };

        let result = PossiblePasswords::new(2, 2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_possible_password_map() {
        let map = vec![
            vec![
                PossiblePasswords::new_all(0, 1, 10, 1, 10, 0, 0, 0, 0),
                PossiblePasswords::new_all(0, 10, 100, 10, 55, 0, 9, 0, 0),
                PossiblePasswords::new_all(0, 100, 1000, 55, 220, 9, 90, 0, 9),
                PossiblePasswords::new_all(0, 1000, 10000, 220, 715, 90, 459, 9, 90),
            ],
            vec![
                PossiblePasswords::new_all(1, 2, 10, 1, 9, 0, 0, 0, 0),
                PossiblePasswords::new_all(10, 20, 100, 9, 45, 1, 9, 0, 0),
                PossiblePasswords::new_all(100, 200, 1000, 45, 165, 17, 81, 1, 9),
                PossiblePasswords::new_all(1000, 2000, 10000, 165, 495, 109, 369, 17, 81),
            ],
            vec![
                PossiblePasswords::new_all(2, 3, 10, 1, 8, 0, 0, 0, 0),
                PossiblePasswords::new_all(20, 30, 100, 8, 36, 1, 8, 0, 0),
                PossiblePasswords::new_all(200, 300, 1000, 36, 120, 15, 64, 1, 8),
                PossiblePasswords::new_all(2000, 3000, 10000, 120, 330, 85, 260, 15, 64),
            ],
            vec![
                PossiblePasswords::new_all(3, 4, 10, 1, 7, 0, 0, 0, 0),
                PossiblePasswords::new_all(30, 40, 100, 7, 28, 1, 7, 0, 0),
                PossiblePasswords::new_all(300, 400, 1000, 28, 84, 13, 49, 1, 7),
                PossiblePasswords::new_all(3000, 4000, 10000, 84, 210, 64, 175, 13, 49),
            ],
            vec![
                PossiblePasswords::new_all(4, 5, 10, 1, 6, 0, 0, 0, 0),
                PossiblePasswords::new_all(40, 50, 100, 6, 21, 1, 6, 0, 0),
                PossiblePasswords::new_all(400, 500, 1000, 21, 56, 11, 36, 1, 6),
                PossiblePasswords::new_all(4000, 5000, 10000, 56, 126, 46, 111, 11, 36),
            ],
            vec![
                PossiblePasswords::new_all(5, 6, 10, 1, 5, 0, 0, 0, 0),
                PossiblePasswords::new_all(50, 60, 100, 5, 15, 1, 5, 0, 0),
                PossiblePasswords::new_all(500, 600, 1000, 15, 35, 9, 25, 1, 5),
                PossiblePasswords::new_all(5000, 6000, 10000, 35, 70, 31, 65, 9, 25),
            ],
            vec![
                PossiblePasswords::new_all(6, 7, 10, 1, 4, 0, 0, 0, 0),
                PossiblePasswords::new_all(60, 70, 100, 4, 10, 1, 4, 0, 0),
                PossiblePasswords::new_all(600, 700, 1000, 10, 20, 7, 16, 1, 4),
                PossiblePasswords::new_all(6000, 7000, 10000, 20, 35, 19, 34, 7, 16),
            ],
            vec![
                PossiblePasswords::new_all(7, 8, 10, 1, 3, 0, 0, 0, 0),
                PossiblePasswords::new_all(70, 80, 100, 3, 6, 1, 3, 0, 0),
                PossiblePasswords::new_all(700, 800, 1000, 6, 10, 5, 9, 1, 3),
                PossiblePasswords::new_all(7000, 8000, 10000, 10, 15, 10, 15, 5, 9),
            ],
            vec![
                PossiblePasswords::new_all(8, 9, 10, 1, 2, 0, 0, 0, 0),
                PossiblePasswords::new_all(80, 90, 100, 2, 3, 1, 2, 0, 0),
                PossiblePasswords::new_all(800, 900, 1000, 3, 4, 3, 4, 1, 2),
                PossiblePasswords::new_all(8000, 9000, 10000, 4, 5, 4, 5, 3, 4),
            ],
            vec![
                PossiblePasswords::new_all(9, 10, 10, 1, 1, 0, 0, 0, 0),
                PossiblePasswords::new_all(90, 100, 100, 1, 1, 1, 1, 0, 0),
                PossiblePasswords::new_all(900, 1000, 1000, 1, 1, 1, 1, 1, 1),
                PossiblePasswords::new_all(9000, 10000, 10000, 1, 1, 1, 1, 1, 1),
            ],
        ];

        let expected = PossiblePasswordMap { map: map };

        let result = PossiblePasswordMap::new(4);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_possible_passwords_for() {
        let possible_password_map = PossiblePasswordMap::new(3);

        let expected_1 = PossiblePasswords::new_all(200, 300, 1000, 36, 120, 15, 64, 1, 8);
        let result_1 = possible_password_map.get_possible_passwords_for(2, 2);

        let expected_2 = PossiblePasswords::new_all(0, 100, 1000, 55, 220, 9, 90, 0, 9);
        let result_2 = possible_password_map.get_possible_passwords_for(0, 2);

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
    }

    #[test]
    fn test_get_number_of_increasing_digits_for() {
        let possible_password_map = PossiblePasswordMap::new(3);

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
    fn test_get_number_of_double_digits_for() {
        let possible_password_map = PossiblePasswordMap::new(4);

        let expected_1 = 15;
        let result_1 = possible_password_map.get_number_of_double_digits_for(2, 2);

        let expected_2 = 0;
        let result_2 = possible_password_map.get_number_of_double_digits_for(3, 0);

        let expected_3 = 0;
        let result_3 = possible_password_map.get_number_of_double_digits_for(9, 0);

        let expected_4 = 1;
        let result_4 = possible_password_map.get_number_of_double_digits_for(9, 1);

        let expected_5 = 9;
        let result_5 = possible_password_map.get_number_of_double_digits_for(0, 2);

        let expected_6 = 90;
        let result_6 = possible_password_map.get_number_of_double_digits_for(0, 3);

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
        assert_eq!(result_4, expected_4);
        assert_eq!(result_5, expected_5);
        assert_eq!(result_6, expected_6);
    }

    #[test]
    fn test_number_of_double_digit_values_up_to() {
        let possible_password_map = PossiblePasswordMap::new(6);

        let expected_1 = 26;
        // From 0 to 200
        let result_1 = possible_password_map.number_of_double_digit_values_between(0, 2, 2);

        let expected_2 = 0;
        // From 0 to 10
        let result_2 = possible_password_map.number_of_double_digit_values_between(0, 1, 1);

        let expected_3 = 0;
        // From 0 to 0
        let result_3 = possible_password_map.number_of_double_digit_values_between(0, 0, 2);

        let expected_4 = 0;
        // From 0 to 7
        let result_4 = possible_password_map.number_of_double_digit_values_between(0, 7, 0);

        let expected_5 = 5;
        // From 0 to 60
        let result_5 = possible_password_map.number_of_double_digit_values_between(0, 6, 1);

        let expected_6 = 0;
        // From 20 to 20
        let result_6 = possible_password_map.number_of_double_digit_values_between(2, 2, 1);

        let expected_7 = 55;
        // From 200 to 700
        let result_7 = possible_password_map.number_of_double_digit_values_between(2, 7, 2);

        let expected_8 = 2851;
        let result_8 = possible_password_map.number_of_double_digit_values_between(0, 2, 5);

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
        assert_eq!(result_4, expected_4);
        assert_eq!(result_5, expected_5);
        assert_eq!(result_6, expected_6);
        assert_eq!(result_7, expected_7);
        assert_eq!(result_8, expected_8);
    }

    #[test]
    fn test_get_number_of_triple_digits_for() {
        let possible_password_map = PossiblePasswordMap::new(9);

        let expected_1 = 15;
        let result_1 = possible_password_map.get_number_of_triple_digits_for(2, 3);

        let expected_2 = 0;
        let result_2 = possible_password_map.get_number_of_triple_digits_for(3, 0);

        let expected_3 = 0;
        let result_3 = possible_password_map.get_number_of_triple_digits_for(3, 1);

        let expected_4 = 0;
        let result_4 = possible_password_map.get_number_of_triple_digits_for(9, 0);

        let expected_5 = 0;
        let result_5 = possible_password_map.get_number_of_triple_digits_for(9, 1);

        let expected_6 = 0;
        let result_6 = possible_password_map.get_number_of_triple_digits_for(0, 2);

        let expected_7 = 9;
        let result_7 = possible_password_map.get_number_of_triple_digits_for(0, 3);

        let expected_8 = 8;
        let result_8 = possible_password_map.get_number_of_triple_digits_for(7, 4);


        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
        assert_eq!(result_4, expected_4);
        assert_eq!(result_5, expected_5);
        assert_eq!(result_6, expected_6);
        assert_eq!(result_7, expected_7);
        assert_eq!(result_8, expected_8);
    }

    #[test]
    fn test_number_of_triple_digit_values_up_to() {
        let possible_password_map = PossiblePasswordMap::new(6);

        let expected_1 = 1;
        // From 0 to 200
        let result_1 = possible_password_map.number_of_triple_digit_values_between(0, 2, 2);

        let expected_2 = 0;
        // From 0 to 10
        let result_2 = possible_password_map.number_of_triple_digit_values_between(0, 1, 1);

        let expected_3 = 0;
        // From 0 to 0
        let result_3 = possible_password_map.number_of_triple_digit_values_between(0, 0, 2);

        let expected_4 = 0;
        // From 0 to 7
        let result_4 = possible_password_map.number_of_triple_digit_values_between(0, 7, 0);

        let expected_5 = 0;
        // From 0 to 60
        let result_5 = possible_password_map.number_of_triple_digit_values_between(0, 6, 1);

        let expected_6 = 0;
        // From 20 to 20
        let result_6 = possible_password_map.number_of_triple_digit_values_between(2, 2, 1);

        let expected_7 = 5;
        // From 200 to 700
        let result_7 = possible_password_map.number_of_triple_digit_values_between(2, 7, 2);

        let expected_8 = 24;
        let result_8 = possible_password_map.number_of_triple_digit_values_between(6, 8, 4);

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
        assert_eq!(result_4, expected_4);
        assert_eq!(result_5, expected_5);
        assert_eq!(result_6, expected_6);
        assert_eq!(result_7, expected_7);
        assert_eq!(result_8, expected_8);
    }
}
