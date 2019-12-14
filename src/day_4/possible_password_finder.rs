use super::possible_passwords::PossiblePasswordMap;

#[derive(Debug, PartialEq)]
pub struct PossiblePasswordFinder {
    map: PossiblePasswordMap,
}

impl PossiblePasswordFinder {
    pub fn new(password_length: usize) -> PossiblePasswordFinder {
        PossiblePasswordFinder {
            map: PossiblePasswordMap::new(password_length),
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

    fn number_of_double_digit_passwords_up_to(&self, excluded_upper_bound: i32) -> i32 {
        // for last digit, if greater than next to last than add last_digit - next_to_last in result

        let int_vector_with_magnitudes =
            Self::int_to_vector_with_magnitude_of_ten(excluded_upper_bound);

        if int_vector_with_magnitudes.len() <= 1 {
            return 0;
        }

        let mut result = 0;
        let mut prior_significant_digit = 0;

        for (significant_digit, magnitude_of_ten) in int_vector_with_magnitudes {
            if prior_significant_digit > significant_digit {
                break;
            } else if magnitude_of_ten == 0 {
                result += (significant_digit - prior_significant_digit) as i32;
            } else {
                result += self
                    .map
                    .number_of_double_digit_values_up_to(significant_digit, magnitude_of_ten);
            }

            prior_significant_digit = significant_digit;
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

        let expected = PossiblePasswordFinder { map: map };

        let result = PossiblePasswordFinder::new(3);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_number_of_double_digit_passwords_between() {
        let possible_password_finder = PossiblePasswordFinder::new(3);

        let expected = 61;

        let result = possible_password_finder.number_of_double_digit_passwords_between(223, 778);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_number_of_double_digit_passwords_up_to() {
        let possible_password_finder = PossiblePasswordFinder::new(3);

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

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
        assert_eq!(result_4, expected_4);
        assert_eq!(result_5, expected_5);
        assert_eq!(result_6, expected_6);
        assert_eq!(result_7, expected_7);
    }

    #[test]
    fn test_int_to_vector_with_magnitude_of_ten() {
        let expected = vec![(2, 5), (3, 4), (1, 3), (8, 2), (3, 1), (2, 0)];

        let result = PossiblePasswordFinder::int_to_vector_with_magnitude_of_ten(231832);

        assert_eq!(result, expected);
    }
}
