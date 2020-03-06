use crate::lcm;
use crate::location::point_3d::Point3d;
use crate::location::Location;

#[derive(Debug, PartialEq, Clone)]
struct Moon {
    position: Point3d<i32>,
    velocity: Point3d<i32>,
}

impl Moon {
    pub fn new_at(position: Point3d<i32>) -> Moon {
        let velocity = Point3d::new(0, 0, 0);

        Moon { position, velocity }
    }

    pub fn get_total_energy(&self) -> i32 {
        let potential_energy =
            self.position.x.abs() + self.position.y.abs() + self.position.z.abs();
        let kinetic_energy = self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs();

        potential_energy * kinetic_energy
    }

    pub fn move_one_step(&mut self) {
        self.position = self.position.add(&self.velocity);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct System {
    original_moons: Vec<Moon>,
    moons: Vec<Moon>,
}

impl<'a> System {
    pub fn new() -> System {
        System {
            original_moons: Vec::new(),
            moons: Vec::new(),
        }
    }

    pub fn iter_mut(&'a mut self) -> SystemIteratorMut<'a> {
        SystemIteratorMut {
            system: self,
            index: 0,
        }
    }

    pub fn add_moon_at(&mut self, position: Point3d<i32>) {
        self.original_moons.push(Moon::new_at(position));
        self.moons.push(Moon::new_at(position));
    }

    pub fn get_total_energy(&self) -> i32 {
        self.moons
            .iter()
            .fold(0, |acc, moon| acc + moon.get_total_energy())
    }

    pub fn steps_to_get_moons_at_original_positions_and_velocities(&mut self) -> u128 {
        let mut self_clone = self.clone();

        self_clone.reset();

        let steps_to_same_x =
            self_clone.steps_to_get_moons_at_original_x_positions_and_velocities();

        self_clone.reset();

        let steps_to_same_y =
            self_clone.steps_to_get_moons_at_original_y_positions_and_velocities();

        self_clone.reset();

        let steps_to_same_z =
            self_clone.steps_to_get_moons_at_original_z_positions_and_velocities();

        let x_y_lcm = lcm(steps_to_same_x, steps_to_same_y);

        lcm(x_y_lcm, steps_to_same_z)
    }

    pub fn reset(&mut self) {
        self.moons = self.original_moons.clone();
    }

    fn simulate_moon_movement(&mut self) {
        let number_of_moons = self.moons.len();

        for current_index in 0..number_of_moons {
            for next_index in 0..number_of_moons {
                if current_index == next_index {
                    continue;
                }

                let next_moon = self.moons.get(next_index).map(|moon| moon.clone()).unwrap();

                let mut current_moon = self.moons.get_mut(current_index).unwrap();

                Self::appy_gravity_between(&mut current_moon, &next_moon);
            }
        }

        for moon in self.moons.iter_mut() {
            moon.move_one_step();
        }
    }

    fn appy_gravity_between(first: &mut Moon, second: &Moon) {
        let mut new_x_velocity = first.velocity.x;
        let mut new_y_velocity = first.velocity.y;
        let mut new_z_velocity = first.velocity.z;

        if first.position.x < second.position.x {
            new_x_velocity += 1;
        } else if first.position.x > second.position.x {
            new_x_velocity -= 1;
        }

        if first.position.y < second.position.y {
            new_y_velocity += 1;
        } else if first.position.y > second.position.y {
            new_y_velocity -= 1;
        }

        if first.position.z < second.position.z {
            new_z_velocity += 1;
        } else if first.position.z > second.position.z {
            new_z_velocity -= 1;
        }

        first.velocity = Point3d::new(new_x_velocity, new_y_velocity, new_z_velocity);
    }

    fn steps_to_get_moons_at_original_axis_positions_and_velocities<F>(&mut self, f: F) -> u128
    where
        F: Fn(&Moon, &Moon) -> bool,
    {
        let mut system_output = self.iter_mut().next();
        let mut steps = 1;

        while let Some(_) = system_output {
            let has_same_x_values = self
                .original_moons
                .iter()
                .zip(self.moons.iter())
                .all(|(original_moon, moon)| f(original_moon, moon));

            if has_same_x_values {
                break;
            }

            system_output = self.iter_mut().next();
            steps += 1;
        }

        steps
    }

    fn steps_to_get_moons_at_original_x_positions_and_velocities(&mut self) -> u128 {
        self.steps_to_get_moons_at_original_axis_positions_and_velocities(|original_moon, moon| {
            (original_moon.position.x == moon.position.x)
                && (original_moon.velocity.x == moon.velocity.x)
        })
    }

    fn steps_to_get_moons_at_original_y_positions_and_velocities(&mut self) -> u128 {
        self.steps_to_get_moons_at_original_axis_positions_and_velocities(|original_moon, moon| {
            (original_moon.position.y == moon.position.y)
                && (original_moon.velocity.y == moon.velocity.y)
        })
    }

    fn steps_to_get_moons_at_original_z_positions_and_velocities(&mut self) -> u128 {
        self.steps_to_get_moons_at_original_axis_positions_and_velocities(|original_moon, moon| {
            (original_moon.position.z == moon.position.z)
                && (original_moon.velocity.z == moon.velocity.z)
        })
    }
}

pub struct SystemIteratorMut<'a> {
    system: &'a mut System,
    index: u128,
}

impl<'a> Iterator for SystemIteratorMut<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.system.simulate_moon_movement();

        self.index += 1;

        Some(self.system.get_total_energy())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STARTING_LOCATIONS: [[i32; 3]; 4] = [
        [-8, -10, 0], // Io
        [5, 5, 10],   // Europa
        [2, -7, 3],   // Ganymede
        [9, -8, -3],  // Callisto
    ];

    #[test]
    fn test_apply_gravity_between() {
        let mut ganymede = Moon::new_at(Point3d::new(1, -1, 1));
        let callisto = Moon::new_at(Point3d::new(-1, 1, 1));
        System::appy_gravity_between(&mut ganymede, &callisto);

        let expected_ganymede_velocity = Point3d::new(-1, 1, 0);

        assert_eq!(ganymede.velocity, expected_ganymede_velocity);
    }

    #[test]
    fn test_10_steps() {
        let moons: Vec<Moon> = STARTING_LOCATIONS
            .iter()
            .map(|location| Point3d::new(location[0], location[1], location[2]))
            .map(|point| Moon::new_at(point))
            .collect();
        let mut system = System {
            original_moons: moons.clone(),
            moons: moons.clone(),
        };

        let mut skip = system.iter_mut().skip(9);

        let expected_io = Moon {
            position: Point3d::new(-9, -10, 1),
            velocity: Point3d::new(-2, -2, -1),
        };
        let expected_europa = Moon {
            position: Point3d::new(4, 10, 9),
            velocity: Point3d::new(-3, 7, -2),
        };
        let expected_ganymede = Moon {
            position: Point3d::new(8, -10, -3),
            velocity: Point3d::new(5, -1, -2),
        };
        let expected_callisto = Moon {
            position: Point3d::new(5, -10, 3),
            velocity: Point3d::new(0, -4, 5),
        };
        let expected_moons = vec![
            expected_io,
            expected_europa,
            expected_ganymede,
            expected_callisto,
        ];
        let expected_system = System {
            original_moons: moons.clone(),
            moons: expected_moons,
        };

        assert_eq!(skip.next(), Some(706));
        assert_eq!(system, expected_system);
    }

    #[test]
    fn test_100_steps() {
        let moons: Vec<Moon> = STARTING_LOCATIONS
            .iter()
            .map(|location| Point3d::new(location[0], location[1], location[2]))
            .map(|point| Moon::new_at(point))
            .collect();
        let mut system = System {
            original_moons: moons.clone(),
            moons: moons.clone(),
        };
        let mut skip = system.iter_mut().skip(99);

        let expected_io = Moon {
            position: Point3d::new(8, -12, -9),
            velocity: Point3d::new(-7, 3, 0),
        };
        let expected_europa = Moon {
            position: Point3d::new(13, 16, -3),
            velocity: Point3d::new(3, -11, -5),
        };
        let expected_ganymede = Moon {
            position: Point3d::new(-29, -11, -1),
            velocity: Point3d::new(-3, 7, 4),
        };
        let expected_callisto = Moon {
            position: Point3d::new(16, -13, 23),
            velocity: Point3d::new(7, 1, 1),
        };
        let expected_moons = vec![
            expected_io,
            expected_europa,
            expected_ganymede,
            expected_callisto,
        ];
        let expected_system = System {
            original_moons: moons.clone(),
            moons: expected_moons,
        };

        assert_eq!(skip.next(), Some(1940));
        assert_eq!(system, expected_system);
    }

    #[test]
    fn test_moon_total_energy() {
        let position = Point3d::new(13, 16, -3);
        let velocity = Point3d::new(3, -11, -5);
        let moon = Moon { position, velocity };

        let expected = 608;

        let result = moon.get_total_energy();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_steps_to_get_moons_at_original_positions_and_velocities() {
        let moons: Vec<Moon> = STARTING_LOCATIONS
            .iter()
            .map(|location| Point3d::new(location[0], location[1], location[2]))
            .map(|point| Moon::new_at(point))
            .collect();
        let mut system = System {
            original_moons: moons.clone(),
            moons: moons.clone(),
        };

        let expected = 4_686_774_924;

        let result = system.steps_to_get_moons_at_original_positions_and_velocities();

        assert_eq!(result, expected);
    }
}
