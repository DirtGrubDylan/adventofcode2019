mod nanofactory;

use crate::file_reader::to_string_vector;

use nanofactory::Nanofactory;

pub fn run_day_14() {
    let file_input = to_string_vector("inputs/day_14.txt");

    match file_input {
        Ok(reactions) => {
            let mut nanofactory = Nanofactory::new();

            for reaction in reactions {
                nanofactory.add_reaction_from_str(&reaction);
            }

            run_part_1(&nanofactory);
            run_part_2(&nanofactory);
        }
        Err(error) => println!("Error parsing file: {:?}", error),
    }
}

pub fn run_part_1(nanofactory: &Nanofactory) {
    println!(
        "Day 14 Part 1 Solution: {}",
        nanofactory.number_of_ore_to_make_n_fuel(1)
    );
}

pub fn run_part_2(nanofactory: &Nanofactory) {
    let ore_to_use: u64 = 1_000_000_000_000;

    let number_of_ore_to_make_1_fuel = nanofactory.number_of_ore_to_make_n_fuel(1);

    let mut floor = ore_to_use / number_of_ore_to_make_1_fuel;

    let mut ciel = floor;

    while nanofactory.number_of_ore_to_make_n_fuel(ciel) <= ore_to_use {
        ciel *= 10;
    }

    let mut mid = (ciel + floor) / 2;

    while floor < ciel {
        let mid_ore = nanofactory.number_of_ore_to_make_n_fuel(mid);

        if mid_ore < ore_to_use {
            floor = mid + 1;
        } else if mid_ore == ore_to_use {
            break;
        } else {
            ciel = mid - 1;
        }

        mid = (ciel + floor) / 2;
    }

    println!("Day 14 Part 2 Solution: {}", mid);
}
