mod possible_password_finder;
mod possible_passwords;

use possible_password_finder::PossiblePasswordFinder;

pub fn run_day_4() {
    let possible_password_finder = PossiblePasswordFinder::new(6);

    let mut number_of_possible_passwords =
        possible_password_finder.number_of_double_digit_passwords_between(231832, 767346);

    println!("Day 4 Part 1 Solution: {:?}", number_of_possible_passwords);

    number_of_possible_passwords = possible_password_finder
        .number_of_non_triple_double_digit_passwords_between(231832, 767346);

    println!("Day 4 Part 2 Solution: {:?}", number_of_possible_passwords);
}
