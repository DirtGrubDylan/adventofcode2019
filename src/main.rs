pub mod file_reader;

mod day_1;
mod day_2;
mod day_3;

fn print_seperator() {
    println!("-------------------------------");
}

fn run_day(day: usize) {
    match day {
        1 => day_1::run_day_1(),
        2 => day_2::run_day_2(),
        3 => day_3::run_day_3(),
        _ => (),
    }
}

fn main() {
    print_seperator();

    run_day(3);

    print_seperator();
}
