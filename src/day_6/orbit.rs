use std::collections::{HashMap, HashSet};
use std::panic;

#[derive(Debug, PartialEq, Clone)]
pub struct OrbitObject {
    pub name: String,
    pub object_name_this_is_orbiting: Option<String>,
    pub object_names_directly_orbiting_this: HashSet<String>,
}

impl OrbitObject {
    pub fn new(
        name: &str,
        object_name_this_is_orbiting: Option<&str>,
        object_names_directly_orbiting_this: &[&str],
    ) -> OrbitObject {
        let temp_set = object_names_directly_orbiting_this
            .iter()
            .map(|&s| String::from(s))
            .collect();

        let temp_object_name_this_is_orbiting =
            object_name_this_is_orbiting.map(|s| String::from(s));

        OrbitObject {
            name: String::from(name),
            object_name_this_is_orbiting: temp_object_name_this_is_orbiting,
            object_names_directly_orbiting_this: temp_set,
        }
    }

    pub fn add_new_object_name_that_directly_orbits(
        &mut self,
        new_directly_orbiting_object_name: &str,
    ) {
        self.object_names_directly_orbiting_this
            .insert(String::from(new_directly_orbiting_object_name));
    }

    pub fn set_object_name_this_is_orbiting(&mut self, object_name_this_is_orbiting: &str) {
        if self.object_name_this_is_orbiting.is_none() {
            self.object_name_this_is_orbiting = Some(String::from(object_name_this_is_orbiting));
        }
    }
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
        orbit_description
            .iter()
            .for_each(|s| self.add_orbit_description_line(&s));
    }

    pub fn total_number_of_orbits(&self) -> u32 {
        self.object_orbiting_map.keys().fold(0, |acc, object_name| {
            acc + self.number_of_orbits_for(object_name)
        })
    }

    pub fn number_of_objects_between(
        &self,
        first_object_name: &str,
        second_object_name: &str,
    ) -> Option<u32> {
        let mut orbit_names_distance_path_for_first_object =
            self.orbit_names_distance_path_for(first_object_name);
        let orbit_names_distance_path_for_second_object =
            self.orbit_names_distance_path_for(second_object_name);

        orbit_names_distance_path_for_first_object
            .retain(|k, _| orbit_names_distance_path_for_second_object.contains_key(k));

        orbit_names_distance_path_for_first_object
            .iter()
            .map(|(object_name, distance)| {
                distance
                    + orbit_names_distance_path_for_second_object
                        .get(object_name)
                        .unwrap()
            })
            .min()
    }

    fn add_orbit_description_line(&mut self, orbit_description_line: &str) {
        let object_names: Vec<String> = orbit_description_line
            .split(')')
            .map(|s| String::from(s))
            .collect();

        assert!(object_names.len() == 2);

        let first_object_name = object_names[0].clone();
        let second_object_name = object_names[1].clone();

        let first_object = self
            .object_orbiting_map
            .entry(first_object_name.clone())
            .or_insert(OrbitObject::new(&first_object_name, None, &[]));

        (*first_object).add_new_object_name_that_directly_orbits(&second_object_name);

        let second_object = self
            .object_orbiting_map
            .entry(second_object_name.clone())
            .or_insert(OrbitObject::new(
                &second_object_name,
                Some(&first_object_name),
                &[],
            ));

        (*second_object).set_object_name_this_is_orbiting(&first_object_name);
    }

    fn number_of_direct_orbits_for(&self, object_name: &str) -> u32 {
        if let Some(object) = self.object_orbiting_map.get(object_name) {
            object.object_name_this_is_orbiting.is_some() as u32
        } else {
            0
        }
    }

    fn number_of_indirect_orbits_for(&self, object_name: &str) -> u32 {
        let mut number_of_indirect_orbits = 0;
        let mut optional_next_object_name: Option<String> = None;

        // Do not count directly orbiting objects
        if let Some(object) = self.object_orbiting_map.get(object_name) {
            optional_next_object_name = object.object_name_this_is_orbiting.clone();
        }

        while let Some(next_object_name) = optional_next_object_name.clone() {
            if let Some(object) = self.object_orbiting_map.get(&next_object_name) {
                optional_next_object_name = object.object_name_this_is_orbiting.clone();
            }

            if optional_next_object_name.is_some() {
                number_of_indirect_orbits += 1;
            }
        }

        number_of_indirect_orbits as u32
    }

    fn number_of_orbits_for(&self, object_name: &str) -> u32 {
        self.number_of_direct_orbits_for(object_name)
            + self.number_of_indirect_orbits_for(object_name)
    }

    fn orbit_names_distance_path_for(&self, object_name: &str) -> HashMap<String, u32> {
        let mut distance_to = 0;
        let mut optional_next_object_name: Option<String> = None;
        let mut result = HashMap::new();

        // Get directly orbiting object
        if let Some(object) = self.object_orbiting_map.get(object_name) {
            optional_next_object_name = object.object_name_this_is_orbiting.clone();
        }

        while let Some(next_object_name) = optional_next_object_name.clone() {
            result.insert(String::from(next_object_name.clone()), distance_to);

            if let Some(object) = self.object_orbiting_map.get(&next_object_name) {
                optional_next_object_name = object.object_name_this_is_orbiting.clone();
            }

            if optional_next_object_name.is_some() {
                distance_to += 1;
            }
        }

        result
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
        let mut orbit_map = OrbitMap::new();

        let expected: HashMap<String, OrbitObject> = [
            (String::from("COM"), OrbitObject::new("COM", None, &["B"])),
            (String::from("B"), OrbitObject::new("B", Some("COM"), &[])),
        ]
        .iter()
        .cloned()
        .collect();

        orbit_map.add_orbit_description_line(TEST_ORBIT_DESCRIPTION[0]);

        assert_eq!(orbit_map.object_orbiting_map, expected);
    }

    #[test]
    fn test_add_two_orbit_description_line() {
        let mut orbit_map = OrbitMap::new();

        let expected: HashMap<String, OrbitObject> = [
            (
                String::from("COM"),
                OrbitObject::new("COM", None, &["B", "C"]),
            ),
            (String::from("B"), OrbitObject::new("B", Some("COM"), &[])),
            (String::from("C"), OrbitObject::new("C", Some("COM"), &[])),
        ]
        .iter()
        .cloned()
        .collect();

        orbit_map.add_orbit_description_line(TEST_ORBIT_DESCRIPTION[0]);
        orbit_map.add_orbit_description_line("COM)C");

        assert_eq!(orbit_map.object_orbiting_map, expected);
    }

    #[test]
    fn test_add_orbit_description() {
        run_test(|orbit_map: OrbitMap| {
            let expected: HashMap<String, OrbitObject> = [
                (String::from("COM"), OrbitObject::new("COM", None, &["B"])),
                (
                    String::from("B"),
                    OrbitObject::new("B", Some("COM"), &["C", "G"]),
                ),
                (String::from("C"), OrbitObject::new("C", Some("B"), &["D"])),
                (
                    String::from("D"),
                    OrbitObject::new("D", Some("C"), &["E", "I"]),
                ),
                (
                    String::from("E"),
                    OrbitObject::new("E", Some("D"), &["F", "J"]),
                ),
                (String::from("F"), OrbitObject::new("F", Some("E"), &[])),
                (String::from("G"), OrbitObject::new("G", Some("B"), &["H"])),
                (String::from("H"), OrbitObject::new("H", Some("G"), &[])),
                (String::from("I"), OrbitObject::new("I", Some("D"), &[])),
                (String::from("J"), OrbitObject::new("J", Some("E"), &["K"])),
                (String::from("K"), OrbitObject::new("K", Some("J"), &["L"])),
                (String::from("L"), OrbitObject::new("L", Some("K"), &[])),
            ]
            .iter()
            .cloned()
            .collect();

            assert_eq!(orbit_map.object_orbiting_map, expected);
        });
    }

    #[test]
    fn test_number_of_direct_orbits_for_b() {
        run_test(|orbit_map: OrbitMap| {
            let expected = 1;

            let result = orbit_map.number_of_direct_orbits_for("B");

            assert_eq!(result, expected);
        })
    }

    #[test]
    fn test_number_of_direct_orbits_for_q() {
        run_test(|orbit_map: OrbitMap| {
            let expected = 0;

            let result = orbit_map.number_of_direct_orbits_for("Q");

            assert_eq!(result, expected);
        })
    }

    #[test]
    fn test_number_of_indirect_orbits_for_d() {
        run_test(|orbit_map: OrbitMap| {
            let expected = 2;

            let result = orbit_map.number_of_indirect_orbits_for("D");

            assert_eq!(result, expected);
        })
    }

    #[test]
    fn test_number_of_indirect_orbits_for_l() {
        run_test(|orbit_map: OrbitMap| {
            let expected = 6;

            let result = orbit_map.number_of_indirect_orbits_for("L");

            assert_eq!(result, expected);
        })
    }

    #[test]
    fn test_number_of_indirect_orbits_for_com() {
        run_test(|orbit_map: OrbitMap| {
            let expected = 0;

            let result = orbit_map.number_of_indirect_orbits_for("COM");

            assert_eq!(result, expected);
        })
    }

    #[test]
    fn test_number_of_indirect_orbits_for_q() {
        run_test(|orbit_map: OrbitMap| {
            let expected = 0;

            let result = orbit_map.number_of_indirect_orbits_for("Q");

            assert_eq!(result, expected);
        })
    }

    #[test]
    fn test_number_of_orbits_for_d() {
        run_test(|orbit_map: OrbitMap| {
            let expected = 3;

            let result = orbit_map.number_of_orbits_for("D");

            assert_eq!(result, expected);
        })
    }

    #[test]
    fn test_number_of_orbits_for_l() {
        run_test(|orbit_map: OrbitMap| {
            let expected = 7;

            let result = orbit_map.number_of_orbits_for("L");

            assert_eq!(result, expected);
        })
    }

    #[test]
    fn test_number_of_orbits_for_com() {
        run_test(|orbit_map: OrbitMap| {
            let expected = 0;

            let result = orbit_map.number_of_orbits_for("COM");

            assert_eq!(result, expected);
        })
    }

    #[test]
    fn test_number_of_orbits_for_q() {
        run_test(|orbit_map: OrbitMap| {
            let expected = 0;

            let result = orbit_map.number_of_orbits_for("Q");

            assert_eq!(result, expected);
        })
    }

    #[test]
    fn test_total_number_of_orbits() {
        run_test(|orbit_map: OrbitMap| {
            let expected = 42;

            let result = orbit_map.total_number_of_orbits();

            assert_eq!(result, expected);
        })
    }

    #[test]
    fn test_orbit_names_distance_path_for() {
        run_test(|orbit_map: OrbitMap| {
            let expected: HashMap<String, u32> = [
                (String::from("K"), 0),
                (String::from("J"), 1),
                (String::from("E"), 2),
                (String::from("D"), 3),
                (String::from("C"), 4),
                (String::from("B"), 5),
                (String::from("COM"), 6),
            ]
            .iter()
            .cloned()
            .collect();

            let result = orbit_map.orbit_names_distance_path_for("L");

            assert_eq!(result, expected);
        });
    }

    #[test]
    fn test_number_of_objects_between() {
        run_test(|orbit_map: OrbitMap| {
            let expected = Some(6);

            let result = orbit_map.number_of_objects_between("L", "H");

            assert_eq!(result, expected);
        });
    }

    fn run_test<T>(test: T)
    where
        T: FnOnce(OrbitMap) -> () + panic::UnwindSafe,
    {
        let mut orbit_map = OrbitMap::new();
        let orbit_description: Vec<String> = TEST_ORBIT_DESCRIPTION
            .iter()
            .rev()
            .map(|&s| String::from(s))
            .collect();

        orbit_map.add_orbit_description(&orbit_description);

        let result = panic::catch_unwind(|| test(orbit_map));

        assert!(result.is_ok());
    }
}
