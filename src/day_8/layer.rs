#[derive(Debug, PartialEq, Clone)]
pub struct Layer {
    pixels: Vec<Vec<u32>>,
}

impl Layer {
    pub fn new(width: usize, layer_image_data: &[u32]) -> Layer {
        Layer {
            pixels: layer_image_data
                .chunks(width)
                .map(|row| row.to_vec())
                .collect(),
        }
    }

    pub fn amount_of_pixels_with_value(&self, target_pixel_value: u32) -> u32 {
        self.pixels.iter().fold(0, |acc, row| {
            acc + row.iter().fold(0, |acc, &pixel_value| {
                acc + ((pixel_value == target_pixel_value) as u32)
            })
        })
    }

    pub fn adjust_with(&mut self, other_layer: &Layer) {
        for (row_index, row) in self.pixels.iter_mut().enumerate() {
            for (column_index, pixel_value) in row.iter_mut().enumerate() {
                let other_pixel_value = other_layer
                    .pixels
                    .get(row_index)
                    .unwrap()
                    .get(column_index)
                    .unwrap();

                if *pixel_value == 2 {
                    *pixel_value = *other_pixel_value;
                }
            }
        }
    }

    pub fn render(&self) {
        for row in self.pixels.iter() {
            let mut rendered_row = String::new();

            for pixel_value in row.iter() {
                if *pixel_value == 1 {
                    rendered_row.push('#');
                } else {
                    rendered_row.push(' ');
                }
            }

            println!("{}", rendered_row);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use super::*;

    const IMAGE_WIDTH: usize = 3;
    const IMAGE_DATA: [u32; 12] = [0, 2, 1, 2, 0, 2, 2, 2, 0, 1, 1, 0];

    #[test]
    fn test_new() {
        run_tests(|layer| {
            let expected = Layer {
                pixels: vec![vec![2, 2, 0], vec![1, 1, 0]],
            };

            assert_eq!(layer, expected);
        });
    }

    #[test]
    fn test_amount_of_pixels_with_value() {
        run_tests(|layer| {
            let expected = 2;

            let result = layer.amount_of_pixels_with_value(0);

            assert_eq!(result, expected);
        });
    }

    #[test]
    fn test_adjust_with() {
        run_tests(|other_layer| {
            let mut layer = Layer::new(IMAGE_WIDTH, &IMAGE_DATA[0..6]);

            let expected_pixels = vec![vec![0, 2, 1], vec![1, 0, 0]];

            layer.adjust_with(&other_layer);

            assert_eq!(layer.pixels, expected_pixels);
        });
    }

    fn run_tests<T>(test: T)
    where
        T: FnOnce(Layer) -> () + panic::UnwindSafe,
    {
        let layer = Layer::new(IMAGE_WIDTH, &IMAGE_DATA[6..12]);

        let result = panic::catch_unwind(|| test(layer));

        assert!(result.is_ok());
    }
}
