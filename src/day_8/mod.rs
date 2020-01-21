mod image;
mod layer;

use crate::file_reader::to_string_vector;
use image::Image;

pub fn run_day_8() {
    let file_input = to_string_vector("inputs/day_8.txt");

    match file_input {
        Ok(images_data) => {
            if let Some(image_text_data) = images_data.get(0) {
                let width = 25;
                let height = 6;
                let image_data: Vec<u32> = image_text_data
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect();

                let image = Image::new(width, height, &image_data);

                run_part_1(&image);
                run_part_2(&image);
            }
        }
        Err(error) => println!("Error parsing file: {:?}", error),
    }
}

fn run_part_1(image: &Image) {
    let layer_with_least_zeroes = image.layer_with_least_amount_of_pixel_value(0).unwrap();

    let number_of_ones_in_layer = layer_with_least_zeroes.amount_of_pixels_with_value(1);
    let number_of_twos_in_layer = layer_with_least_zeroes.amount_of_pixels_with_value(2);

    println!(
        "Day 8 Part 1 Solution: {}",
        number_of_twos_in_layer * number_of_ones_in_layer
    );
}

fn run_part_2(image: &Image) {
    println!("Day 8 Part 2 Solution: ");

    image.render();
}
