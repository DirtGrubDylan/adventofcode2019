use super::module::Module;

#[derive(Debug, PartialEq)]
pub struct FuelCounterUpper {
    pub modules: Vec<Module>,
}

impl FuelCounterUpper {
    pub fn new(mass_values: &Vec<String>) -> FuelCounterUpper {
        let mut temp_modules = Vec::new();

        for mass_value in mass_values {
            temp_modules.push(Module::from_str(&mass_value));
        }

        FuelCounterUpper {
            modules: temp_modules,
        }
    }

    pub fn total_fuel_required(&self) -> i32 {
        self.modules
            .iter()
            .map(|module: &Module| fuel_required(module))
            .sum()
    }

    pub fn total_fuel_required_recursive(&self) -> i32 {
        self.modules
            .iter()
            .map(|module: &Module| fuel_required_recursive(module.mass))
            .sum()
    }
}

fn fuel_required(module: &Module) -> i32 {
    (module.mass / 3) - 2
}

fn fuel_required_recursive(mass: i32) -> i32 {
    let fuel_needed = (mass / 3) - 2;

    if fuel_needed > 0 {
        fuel_needed + fuel_required_recursive(fuel_needed)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_reader::to_string_vector;

    #[test]
    fn test_new() {
        let mass_values = vec![String::from("5"), String::from("666")];

        let expected = FuelCounterUpper {
            modules: vec![Module::new(5), Module::new(666)],
        };

        let result = FuelCounterUpper::new(&mass_values);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_with_file() {
        let mass_values = to_string_vector("test_inputs/day_1_part_1.txt").unwrap();

        let expected = FuelCounterUpper {
            modules: vec![
                Module::new(12),
                Module::new(14),
                Module::new(1969),
                Module::new(100756),
            ],
        };

        let result = FuelCounterUpper::new(&mass_values);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_total_fuel_required() {
        let expected = 656;

        let result = FuelCounterUpper::new(&vec![String::from("12"), String::from("1969")])
            .total_fuel_required();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_total_fuel_required_with_file() {
        let mass_values = to_string_vector("test_inputs/day_1_part_1.txt").unwrap();

        let expected = 34_241;

        let result = FuelCounterUpper::new(&mass_values).total_fuel_required();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_total_fuel_required_recursive() {
        let expected = 968;

        let result = FuelCounterUpper::new(&vec![String::from("12"), String::from("1969")])
            .total_fuel_required_recursive();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_total_fuel_required_recursive_with_file() {
        let mass_values = to_string_vector("test_inputs/day_1_part_1.txt").unwrap();

        let expected = 51_316;

        let result = FuelCounterUpper::new(&mass_values).total_fuel_required_recursive();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_fuel_required() {
        assert_eq!(fuel_required(&Module::new(12)), 2);
        assert_eq!(fuel_required(&Module::new(1969)), 654);
    }

    #[test]
    fn test_fuel_required_recursive() {
        assert_eq!(fuel_required_recursive(12), 2);
        assert_eq!(fuel_required_recursive(1969), 966);
    }
}
