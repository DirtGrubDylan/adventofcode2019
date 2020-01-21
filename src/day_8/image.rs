use super::layer::Layer;

#[derive(Debug, PartialEq)]
pub struct Image {
    width: usize,
    height: usize,
    layers: Vec<Layer>,
}

impl Image {
    pub fn new(width: usize, height: usize, image_data: &[u32]) -> Image {
        unimplemented!()
    }

    pub fn layer_with_least_amount_of_pixel_value(&self, pixel_value: u32) -> &Layer {
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
        run_tests(|image| {
            let expected = Image {
                width: IMAGE_WIDTH,
                height: IMAGE_HEIGHT,
                layers: vec![
                    Layer::new(IMAGE_WIDTH, IMAGE_HEIGHT, &IMAGE_DATA[0..6]),
                    Layer::new(IMAGE_WIDTH, IMAGE_HEIGHT, &IMAGE_DATA[0..6]),
                ],
            };

            assert_eq!(image, expected);
        });
    }

    #[test]
    fn test_layer_with_least_amount_of_pixel_value() {
        run_tests(|image| {
            let expected = image.layers.get(0).unwrap();

            let result = image.layer_with_least_amount_of_pixel_value(0);

            assert_eq!(result, expected);
        });
    }

    fn run_tests<T>(test: T)
    where
        T: FnOnce(Image) -> () + panic::UnwindSafe,
    {
        let image = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT, &IMAGE_DATA);

        let result = panic::catch_unwind(|| test(image));

        assert!(result.is_ok());
    }
}
