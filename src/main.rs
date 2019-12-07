pub mod file_reader;

mod day_1;
mod day_2;

fn print_seperator() {
    println!("-------------------------------");
}

fn run_day(day: usize) {
    match day {
        1 => day_1::run_day_1(),
        2 => day_2::run_day_2(),
        _ => (),
    }
}

fn main() {
    print_seperator();

    run_day(2);

    print_seperator();
}
