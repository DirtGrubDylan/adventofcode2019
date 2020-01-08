pub mod file_reader;
pub mod intcode_computer;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

fn print_seperator() {
    println!("-------------------------------");
}

fn run_day(day: usize) {
    match day {
        1 => day_1::run_day_1(),
        2 => day_2::run_day_2(),
        3 => day_3::run_day_3(),
        4 => day_4::run_day_4(),
        5 => day_5::run_day_5(),
        _ => (),
    }
}

fn main() {
    print_seperator();

    run_day(2);

    print_seperator();
}
