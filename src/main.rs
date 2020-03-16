use std::io::{self, Write};

pub mod file_reader;
pub mod intcode_computer;
pub mod location;

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

fn print_seperator() {
    println!("-------------------------------");
}

fn run_day(day: i128) {
    match day {
        1 => day_1::run_day_1(),
        2 => day_2::run_day_2(),
        3 => day_3::run_day_3(),
        4 => day_4::run_day_4(),
        5 => day_5::run_day_5(),
        6 => day_6::run_day_6(),
        7 => day_7::run_day_7(),
        8 => day_8::run_day_8(),
        9 => day_9::run_day_9(),
        10 => day_10::run_day_10(),
        11 => day_11::run_day_11(),
        12 => day_12::run_day_12(),
        13 => day_13::run_day_13(),
        _ => unimplemented!("I haven't done that day yet :("),
    }
}

pub fn get_user_input() -> i128 {
    let mut input_buffer = String::new();

    io::stdout().flush().expect("Could not flush stdout!");

    io::stdin()
        .read_line(&mut input_buffer)
        .expect("Failed to read user input!");

    input_buffer
        .trim()
        .parse::<i128>()
        .expect("Failed to parse user_input!")
}

pub fn gcd(a: u128, b: u128) -> u128 {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (x.min(y), x.max(y));
            gcd((y - x) >> 1, x)
        }
        _ => panic!("GCD made it to an unreachable state!"),
    }
}

pub fn lcm(a: u128, b: u128) -> u128 {
    a * b / gcd(a, b)
}

fn main() {
    print_seperator();

    print!("Please choose a day to run (1-25): ");

    let input = get_user_input();

    print_seperator();

    run_day(input);

    print_seperator();
}
