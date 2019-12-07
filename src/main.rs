pub mod file_reader;

mod day_1;

fn print_seperator() {
    println!("-------------------------------");
}

fn run_day(day: usize) {
    match day {
        1 => day_1::run_day_1(),
        _ => (),
    }
}

fn main() {
    print_seperator();

    run_day(1);

    print_seperator();
}
