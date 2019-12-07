#[derive(Debug, PartialEq)]
pub struct Module {
    pub mass: i32,
}

impl Module {
    pub fn new(mass: i32) -> Module {
        Module { mass: mass }
    }

    pub fn from_str(mass_str: &str) -> Module {
        Module::new(mass_str.parse::<i32>().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let expected = Module { mass: 5 };

        let result = Module::new(5);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_str() {
        let mass_string = String::from("666");

        let expected = Module { mass: 666 };

        let result = Module::from_str(&mass_string);

        assert_eq!(result, expected);
    }
}
