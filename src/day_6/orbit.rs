use std::collections::HashMap;
use std::panic;

#[derive(Debug)]
pub struct OrbitObject {
    pub name: String,
    pub object_names_directly_orbiting: Vec<String>,
}

#[derive(Debug)]
pub struct OrbitMap {
    object_orbiting_map: HashMap<String, OrbitObject>,
}

impl OrbitMap {
    pub fn new() -> OrbitMap {
        OrbitMap {
            object_orbiting_map: HashMap::new(),
        }
    }

    pub fn add_orbit_description(&mut self, orbit_description: &[String]) {
        unimplemented!()
    }

    pub fn total_number_of_orbits(&self) -> u32 {
        unimplemented!()
    }

    fn add_orbit_description_line(&mut self, orbit_description_line: &str) {
        unimplemented!()
    }

    fn total_number_of_direct_orbits(&self) -> u32 {
        unimplemented!()
    }

    fn total_number_of_indirect_orbits(&self) -> u32 {
        unimplemented!()
    }

    fn number_of_direct_orbits_for(&self, object_name: &str) -> u32 {
        unimplemented!()
    }

    fn number_of_indirect_orbits_for(&self, object_name: &str) -> u32 {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ORBIT_DESCRIPTION: [&'static str; 11] = [
        "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
    ];

    #[test]
    fn test_add_one_orbit_description_line() {
        unimplemented!()
    }

    #[test]
    fn test_add_orbit_description() {
        unimplemented!()
    }

    #[test]
    fn test_number_of_direct_orbits_for() {
        run_test(|orbit_map: OrbitMap| {
            let expected = 42;

            let result = orbit_map.total_number_of_orbits();

            assert_eq!(result, expected)
        })
    }

    #[test]
    fn test_number_of_indirect_orbits_for() {
        run_test(|orbit_map: OrbitMap| {
            let expected = 42;

            let result = orbit_map.total_number_of_orbits();

            assert_eq!(result, expected)
        })
    }

    #[test]
    fn test_total_number_of_direct_orbits() {
        run_test(|orbit_map: OrbitMap| {
            let expected = 11;

            let result = orbit_map.total_number_of_direct_orbits();

            assert_eq!(result, expected)
        })
    }

    #[test]
    fn test_total_number_of_indirect_orbits() {
        run_test(|orbit_map: OrbitMap| {
            let expected = 31;

            let result = orbit_map.total_number_of_indirect_orbits();

            assert_eq!(result, expected)
        })
    }

    #[test]
    fn test_total_number_of_orbits() {
        run_test(|orbit_map: OrbitMap| {
            let expected = 42;

            let result = orbit_map.total_number_of_orbits();

            assert_eq!(result, expected)
        })
    }

    fn run_test<T>(test: T)
    where
        T: FnOnce(OrbitMap) -> () + panic::UnwindSafe,
    {
        let orbit_map = OrbitMap::new();

        let result = panic::catch_unwind(|| test(orbit_map));

        assert!(result.is_ok())
    }
}
