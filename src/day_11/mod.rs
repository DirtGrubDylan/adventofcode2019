mod robot;

use crate::file_reader::to_string_vector;
use crate::location::point_2d::Point2d;

use robot::{PaintColor, Robot};

pub fn run_day_11() {
    let file_input = to_string_vector("inputs/day_11.txt");

    match file_input {
        Ok(programs) => {
            if let Some(program) = programs.get(0) {
                let program_values: Vec<String> =
                    program.split(",").map(|s| String::from(s)).collect();

                let mut robot = Robot::new(program_values.as_slice());

                run_part_1(&mut robot);
                run_part_2(&mut robot);
            }
        }
        Err(error) => println!("Error parsing file: {:?}", error),
    }
}

fn run_part_1(robot: &mut Robot) {
    robot.reset();

    robot.run_program();

    let number_of_painted_panels = robot.get_painted_panels().len();

    println!("Day 11 Part 1 Solution: {}", number_of_painted_panels);
}

fn run_part_2(robot: &mut Robot) {
    robot.reset();

    robot.set_starting_panel_color(PaintColor::White);

    robot.run_program();

    let painted_panels = robot.get_painted_panels();

    let max_x = match painted_panels
        .keys()
        .into_iter()
        .max_by_key(|point| point.x.abs())
    {
        Some(point) => point.x.abs() + 1,
        None => panic!("There were no max points!!!!"),
    };

    let max_y = match painted_panels
        .keys()
        .into_iter()
        .max_by_key(|point| point.y.abs())
    {
        Some(point) => point.y.abs() + 1,
        None => panic!("There were no max points!!!!"),
    };


    let mut display = Vec::new();

    for y in 0..(max_y * 2) {
        let mut temp_string = String::new();

        for x in 0..(max_x * 2) {
            let temp_point = Point2d::new(x - max_x, y - max_y);

            let temp_char = match painted_panels.get(&temp_point) {
                Some(PaintColor::White) => '#',
                _ => ' ',
            };

            temp_string.push(temp_char);
        }

        display.push(String::from(temp_string.trim_start().trim()));
    }

    println!("Day 11 Part 2 Solution:");

    for s in display.iter().rev() {
        if !s.is_empty() {
            println!("{}", s);
        }
    }
}
