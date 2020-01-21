#[derive(Debug, PartialEq)]
pub struct Layer {
    pixels: Vec<Vec<u32>>,
}

impl Layer {
    pub fn new(width: usize, height: usize, layer_image_data: &[u32]) -> Layer {
        unimplemented!()
    }

    pub fn amount_of_pixels_with_value(&self, pixel_value: u32) -> u32 {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use super::*;

    const IMAGE_WIDTH: usize = 3;
    const IMAGE_HEIGHT: usize = 2;
    const IMAGE_DATA: [u32; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2];

    #[test]
    fn test_new() {
        run_tests(|layer| {
            let expected = Layer {
                pixels: vec![vec![7, 8, 9], vec![0, 1, 2]],
            };

            assert_eq!(layer, expected);
        });
    }

    #[test]
    fn test_amount_of_pixels_with_value() {
        run_tests(|layer| {
            let expected = 1;

            let result = layer.amount_of_pixels_with_value(0);

            assert_eq!(result, expected);
        });
    }

    fn run_tests<T>(test: T)
    where
        T: FnOnce(Layer) -> () + panic::UnwindSafe,
    {
        let layer = Layer::new(IMAGE_WIDTH, IMAGE_HEIGHT, &IMAGE_DATA[6..12]);

        let result = panic::catch_unwind(|| test(layer));

        assert!(result.is_ok());
    }
}
