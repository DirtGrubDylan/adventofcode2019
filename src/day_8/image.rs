use super::layer::Layer;

#[derive(Debug, PartialEq)]
pub struct Image {
    width: usize,
    height: usize,
    layers: Vec<Layer>,
}

impl Image {
    pub fn new(width: usize, height: usize, image_data: &[u32]) -> Image {
        Image {
            width: width,
            height: height,
            layers: image_data
                .chunks(width * height)
                .map(|layer_data| Layer::new(width, layer_data))
                .collect(),
        }
    }

    pub fn layer_with_least_amount_of_pixel_value(
        &self,
        target_pixel_value: u32,
    ) -> Option<&Layer> {
        self.layers
            .iter()
            .min_by_key(|pixel| pixel.amount_of_pixels_with_value(target_pixel_value))
    }

    pub fn render(&self) {
        self.get_flattened_layer().render();
    }

    fn get_flattened_layer(&self) -> Layer {
        let mut flattened_layer = self.layers.first().unwrap().clone();

        for other_layer in self.layers.iter().skip(1) {
            flattened_layer.adjust_with(other_layer);
        }

        flattened_layer
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use super::*;

    const IMAGE_WIDTH: usize = 3;
    const IMAGE_HEIGHT: usize = 2;
    const IMAGE_DATA: [u32; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
    const OTHER_IMAGE_WIDTH: usize = 2;
    const OTHER_IMAGE_HEIGHT: usize = 2;
    const OTHER_IMAGE_DATA: [u32; 16] = [0, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0];

    #[test]
    fn test_new() {
        run_tests(|image| {
            let expected = Image {
                width: IMAGE_WIDTH,
                height: IMAGE_HEIGHT,
                layers: vec![
                    Layer::new(IMAGE_WIDTH, &IMAGE_DATA[0..6]),
                    Layer::new(IMAGE_WIDTH, &IMAGE_DATA[6..12]),
                ],
            };

            assert_eq!(image, expected);
        });
    }

    #[test]
    fn test_layer_with_least_amount_of_pixel_value() {
        run_tests(|image| {
            let expected = image.layers.get(0);

            let result = image.layer_with_least_amount_of_pixel_value(0);

            assert_eq!(result, expected);
        });
    }

    #[test]
    fn test_get_flattened_layer() {
        let image = Image::new(OTHER_IMAGE_WIDTH, OTHER_IMAGE_HEIGHT, &OTHER_IMAGE_DATA);

        let expected = Layer::new(OTHER_IMAGE_WIDTH, &[0, 1, 1, 0]);

        let result = image.get_flattened_layer();

        assert_eq!(result, expected);
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
