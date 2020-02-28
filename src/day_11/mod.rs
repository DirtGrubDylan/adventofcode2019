mod robot;

use crate::file_reader::to_string_vector;

use robot::Robot;

pub fn run_day_11() {
    let file_input = to_string_vector("inputs/day_11.txt");

    match file_input {
        Ok(programs) => {
            if let Some(program) = programs.get(0) {
                let program_values: Vec<String> =
                    program.split(",").map(|s| String::from(s)).collect();

                let mut robot = Robot::new(program_values.as_slice());

                run_part_1(&mut robot);
                run_part_2(&robot);
            }
        }
        Err(error) => println!("Error parsing file: {:?}", error),
    }
}

fn run_part_1(robot: &mut Robot) {
    robot.run_program();

    let number_of_painted_panels = robot.get_number_of_painted_panels();

    println!("Day 11 Part 1 Solution: {}", number_of_painted_panels);
}

fn run_part_2(robot: &Robot) {
}
