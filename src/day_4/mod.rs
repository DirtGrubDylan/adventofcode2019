mod possible_password_finder;
mod possible_passwords;

use possible_password_finder::PossiblePasswordFinder;

pub fn run_day_4() {
    let possible_password_finder = PossiblePasswordFinder::new(6);

    let number_of_possible_passwords =
        possible_password_finder.number_of_double_digit_passwords_between(231832, 767346);

    println!("Day 4 Part 1 Solution: {:?}", number_of_possible_passwords);

    // for part 2
    // sum of:
    //   * all trips from [starting_digit + 1, length - 1];
    //     * subtract all trips at [starting_digit + 1, length - 2]
    //   * all (from [starting_digit, length - 2]) increasing - double_dig
}
