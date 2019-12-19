use super::possible_passwords::PossiblePasswordMap;

#[derive(Debug, PartialEq)]
pub struct PossiblePasswordFinder {
    pub map: PossiblePasswordMap,
    password_length: usize,
}

impl PossiblePasswordFinder {
    pub fn new(password_length: usize) -> PossiblePasswordFinder {
        PossiblePasswordFinder {
            map: PossiblePasswordMap::new(password_length),
            password_length: password_length,
        }
    }

    pub fn number_of_double_digit_passwords_between(
        &self,
        lower_bound_including: i32,
        upper_bound_including: i32,
    ) -> i32 {
        self.number_of_double_digit_passwords_up_to(upper_bound_including + 1)
            - self.number_of_double_digit_passwords_up_to(lower_bound_including)
    }

    pub fn number_of_triple_digit_passwords_between(
        &self,
        lower_bound_including: i32,
        upper_bound_including: i32,
    ) -> i32 {
        self.number_of_triple_digit_passwords_up_to(upper_bound_including + 1)
            - self.number_of_triple_digit_passwords_up_to(lower_bound_including)
    }

    pub fn number_of_non_triple_double_digit_passwords_between(
        &self,
        lower_bound_including: i32,
        upper_bound_including: i32,
    ) -> i32 {
        self.number_of_double_digit_passwords_between(lower_bound_including, upper_bound_including)
            - self.number_of_triple_digit_passwords_between(
                lower_bound_including,
                upper_bound_including,
            )
    }

    fn number_of_double_digit_passwords_up_to(&self, excluded_upper_bound: i32) -> i32 {
        let int_vector_with_magnitudes =
            Self::int_to_vector_with_magnitude_of_ten(excluded_upper_bound);

        if int_vector_with_magnitudes.len() <= 1 {
            return 0;
        }

        let mut prior_starting_digit = 0;
        let mut result = 0;

        for (starting_digit, length) in int_vector_with_magnitudes {
            if prior_starting_digit > starting_digit {
                break;
            } else if length == 0 {
                result += (starting_digit - prior_starting_digit) as i32;
            } else if prior_starting_digit == 0 {
                result += self.map.number_of_double_digit_values_between(
                    prior_starting_digit,
                    starting_digit,
                    length,
                );
            } else if prior_starting_digit < starting_digit {
                result += self.map.map[prior_starting_digit][length].increasing_digits_in_range
                    + self.map.number_of_double_digit_values_between(
                        prior_starting_digit + 1,
                        starting_digit,
                        length,
                    );
            }

            prior_starting_digit = starting_digit;
        }

        result
    }

    fn number_of_triple_digit_passwords_up_to(&self, excluded_upper_bound: i32) -> i32 {
        let int_vector_with_magnitudes =
            Self::int_to_vector_with_magnitude_of_ten(excluded_upper_bound);

        if int_vector_with_magnitudes.len() <= 2 {
            return 0;
        }

        let mut prior_starting_digit = 0;
        let mut excluded_triple_digit_length = 0;
        let mut excluded_increasing_digit_length = 0;
        let mut result = 0;

        for (starting_digit, length) in int_vector_with_magnitudes {
            if prior_starting_digit > starting_digit {
                break;
            } else if prior_starting_digit == starting_digit {
                continue;
            }

            if prior_starting_digit == 0 {
                result += self.map.number_of_triple_digit_values_between(
                    prior_starting_digit,
                    starting_digit,
                    length,
                );

                excluded_triple_digit_length = length - 2;
                excluded_increasing_digit_length = length - 1;
                prior_starting_digit = starting_digit;

                continue;
            }

            if (length != 0) && (prior_starting_digit != 0) {
                result += self.number_of_strict_increasing_inclusive(
                    prior_starting_digit,
                    excluded_increasing_digit_length - 1,
                );
            }

            if length == excluded_triple_digit_length {
                break;
            } else if length <= 1 {
                result += self.number_of_strict_increasing_between(
                    prior_starting_digit + 1,
                    starting_digit,
                    length,
                );
            } else {
                let temp_length_range = length.min(excluded_triple_digit_length);

                for temp_length in 0..temp_length_range {
                    result += self.map.map[prior_starting_digit + 1][temp_length]
                        .triple_digits_in_extended_range;
                }

                result += self.map.number_of_triple_digit_values_between(
                    prior_starting_digit + 1,
                    starting_digit,
                    length,
                );

                prior_starting_digit = starting_digit;
                excluded_increasing_digit_length = length - 1;
                excluded_triple_digit_length = length - 2;
            }
        }

        result
    }

    fn number_of_strict_increasing_inclusive(&self, starting_digit: usize, length: usize) -> i32 {
        let mut result = 0;

        for temp_length in 0..(length + 1) {
            result += self.map.map[starting_digit][temp_length].increasing_digits_in_range
                - self.map.map[starting_digit][temp_length].double_digits_in_range;
        }

        result
    }

    fn number_of_strict_increasing_between(
        &self,
        upper_bound_including: usize,
        lower_bound_excluding: usize,
        length: usize,
    ) -> i32 {
        let mut result = 0;

        for temp_digit in upper_bound_including..lower_bound_excluding {
            result += self.map.map[temp_digit][length].increasing_digits_in_range
                - self.map.map[temp_digit][length].double_digits_in_range;
        }

        result
    }

    fn int_to_vector_with_magnitude_of_ten(int_to_convert: i32) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        let mut magnitude_of_ten = 0;

        let mut int_to_convert_so_far = int_to_convert;

        while int_to_convert_so_far > 0 {
            result.push((
                (int_to_convert_so_far % 10) as usize,
                magnitude_of_ten as usize,
            ));

            int_to_convert_so_far /= 10;
            magnitude_of_ten += 1;
        }

        result.reverse();

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let map = PossiblePasswordMap::new(3);

        let expected = PossiblePasswordFinder {
            map: map,
            password_length: 3,
        };

        let result = PossiblePasswordFinder::new(3);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_number_of_double_digit_passwords_between() {
        let possible_password_finder = PossiblePasswordFinder::new(6);

        let expected_1 = 56;
        let result_1 = possible_password_finder.number_of_double_digit_passwords_between(223, 778);

        let expected_2 = 454;
        let result_2 =
            possible_password_finder.number_of_double_digit_passwords_between(402328, 864247);

        let expected_3 = 1330;
        let result_3 =
            possible_password_finder.number_of_double_digit_passwords_between(231832, 767356);

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
    }

    #[test]
    fn test_number_of_triple_digit_passwords_between() {
        let possible_password_finder = PossiblePasswordFinder::new(6);

        let expected_1 = 5;
        let result_1 = possible_password_finder.number_of_triple_digit_passwords_between(223, 778);

        let expected_2 = 166;
        let result_2 =
            possible_password_finder.number_of_triple_digit_passwords_between(402328, 864247);

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
    }

    #[test]
    fn test_number_of_non_triple_double_digit_passwords_between() {
        let possible_password_finder = PossiblePasswordFinder::new(6);

        let expected_1 = 51;
        let result_1 =
            possible_password_finder.number_of_non_triple_double_digit_passwords_between(223, 778);

        let expected_2 = 288;
        let result_2 = possible_password_finder
            .number_of_non_triple_double_digit_passwords_between(402328, 864247);

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
    }

    #[test]
    fn test_number_of_double_digit_passwords_up_to() {
        let possible_password_finder = PossiblePasswordFinder::new(6);

        let expected_1 = 0;
        let result_1 = possible_password_finder.number_of_double_digit_passwords_up_to(0);

        let expected_2 = 9;
        let result_2 = possible_password_finder.number_of_double_digit_passwords_up_to(100);

        let expected_3 = 27;
        let result_3 = possible_password_finder.number_of_double_digit_passwords_up_to(223);

        let expected_4 = 67;
        let result_4 = possible_password_finder.number_of_double_digit_passwords_up_to(557);

        let expected_5 = 81;
        let result_5 = possible_password_finder.number_of_double_digit_passwords_up_to(761);

        let expected_6 = 81;
        let result_6 = possible_password_finder.number_of_double_digit_passwords_up_to(777);

        let expected_7 = 89;
        let result_7 = possible_password_finder.number_of_double_digit_passwords_up_to(999);

        let expected_8 = 2851;
        let result_8 = possible_password_finder.number_of_double_digit_passwords_up_to(200000);

        let expected_9 = 1179;
        let result_9 = possible_password_finder.number_of_double_digit_passwords_up_to(030000);

        let expected_10 = 284;
        let result_10 = possible_password_finder.number_of_double_digit_passwords_up_to(003000);

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
        assert_eq!(result_4, expected_4);
        assert_eq!(result_5, expected_5);
        assert_eq!(result_6, expected_6);
        assert_eq!(result_7, expected_7);
        assert_eq!(result_8, expected_8);
        assert_eq!(result_9, expected_9);
        assert_eq!(result_10, expected_10);
    }

    #[test]
    fn test_number_of_triple_digit_passwords_up_to() {
        let possible_password_finder = PossiblePasswordFinder::new(6);

        let expected_1 = 0;
        let result_1 = possible_password_finder.number_of_triple_digit_passwords_up_to(0);

        let expected_2 = 0;
        let result_2 = possible_password_finder.number_of_triple_digit_passwords_up_to(100);

        let expected_3 = 4;
        let result_3 = possible_password_finder.number_of_triple_digit_passwords_up_to(554);

        let expected_4 = 81;
        let result_4 = possible_password_finder.number_of_triple_digit_passwords_up_to(7761);

        let expected_5 = 84;
        let result_5 = possible_password_finder.number_of_triple_digit_passwords_up_to(7789);

        let expected_6 = 418;
        let result_6 = possible_password_finder.number_of_triple_digit_passwords_up_to(79999);

        let expected_7 = 1295;
        let result_7 = possible_password_finder.number_of_triple_digit_passwords_up_to(999999);

        let expected_8 = 1051;
        let result_8 = possible_password_finder.number_of_triple_digit_passwords_up_to(345678);

        let expected_9 = 79;
        let result_9 = possible_password_finder.number_of_triple_digit_passwords_up_to(6789);

        let expected_10 = 1125;
        let result_10 = possible_password_finder.number_of_triple_digit_passwords_up_to(402328);

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
        assert_eq!(result_4, expected_4);
        assert_eq!(result_5, expected_5);
        assert_eq!(result_6, expected_6);
        assert_eq!(result_7, expected_7);
        assert_eq!(result_8, expected_8);
        assert_eq!(result_9, expected_9);
        assert_eq!(result_10, expected_10);
    }

    #[test]
    fn test_number_of_strict_increasing_inclusive() {
        let possible_password_finder = PossiblePasswordFinder::new(6);

        let expected = 4;
        let result = possible_password_finder.number_of_strict_increasing_inclusive(7, 3);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_number_of_strict_increasing_between() {
        let possible_password_finder = PossiblePasswordFinder::new(6);

        let expected = 7;
        let result = possible_password_finder.number_of_strict_increasing_between(5, 7, 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_int_to_vector_with_magnitude_of_ten() {
        let expected = vec![(2, 5), (3, 4), (1, 3), (8, 2), (3, 1), (2, 0)];

        let result = PossiblePasswordFinder::int_to_vector_with_magnitude_of_ten(231832);

        assert_eq!(result, expected);
    }
}
