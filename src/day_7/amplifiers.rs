use std::collections::VecDeque;

use crate::intcode_computer::{IntcodeComputer, IntcodeComputerResult};

#[derive(Debug, PartialEq)]
pub struct Amplifier {
    name: String,
    phase_setting: i32,
    input_signal: i128,
    intcode_computer: IntcodeComputer,
}

impl Amplifier {
    pub fn new(
        name: &str,
        phase_setting: i32,
        input_signal: i128,
        intcode_computer: &IntcodeComputer,
    ) -> Amplifier {
        Amplifier {
            name: String::from(name),
            phase_setting: phase_setting,
            input_signal: input_signal,
            intcode_computer: intcode_computer.clone(),
        }
    }

    pub fn run_program(&mut self) -> Result<(IntcodeComputerResult, i128), String> {
        self.intcode_computer.set_input(self.phase_setting);

        self.intcode_computer.execute_program_new_hash();

        self.continue_program()
    }

    pub fn continue_program(&mut self) -> Result<(IntcodeComputerResult, i128), String> {
        self.intcode_computer.set_input(self.input_signal as i32);

        let (result, output) = self.intcode_computer.execute_program_new_hash();

        match output {
            Some(value) => Ok((result, value)),
            None => Err(format!("Something went wrong for amplifier: {}", self.name)),
        }
    }

    pub fn reset_computer(&mut self) {
        self.intcode_computer.reset();
    }
}

#[derive(Debug)]
pub struct AmplifierCircuit {
    amplifiers: Vec<Amplifier>,
}

impl AmplifierCircuit {
    pub fn new(amplifier_names: &[&str], program: &[i32]) -> AmplifierCircuit {
        let mut temp_amplifiers = Vec::new();

        let temp_phase_setting = 0;
        let temp_input_signal = 0;

        for amplifier_name in amplifier_names {
            let temp_intcode_computer = IntcodeComputer::new(program);
            let temp_amplifier = Amplifier::new(
                amplifier_name,
                temp_phase_setting,
                temp_input_signal,
                &temp_intcode_computer,
            );

            temp_amplifiers.push(temp_amplifier);
        }

        AmplifierCircuit {
            amplifiers: temp_amplifiers,
        }
    }

    pub fn get_largest_output_signal(
        &mut self,
        phase_settings: &[i32],
    ) -> Result<(Vec<i32>, i128), String> {
        let number_of_amplifiers = self.amplifiers.len();

        let mut best_phase_settings = Vec::new();
        let mut best_output_signal = i128::min_value();

        let variations = Self::get_all_phase_signal_variations(phase_settings);

        for phase_settings in variations {
            let mut is_first_run = true;
            let mut next_input_signal = 0;
            let mut amplifier_index = 0;

            loop {
                let phase_setting = phase_settings[amplifier_index];

                let mut amplifier = self.amplifiers.get_mut(amplifier_index).unwrap();

                amplifier.phase_setting = phase_setting;
                amplifier.input_signal = next_input_signal;

                let (result, output) = match is_first_run {
                    true => amplifier.run_program().unwrap(),
                    false => amplifier.continue_program().unwrap(),
                };

                if best_output_signal < output {
                    best_phase_settings = phase_settings.clone();
                    best_output_signal = output;
                }

                if (result == IntcodeComputerResult::FINISHED)
                    && (amplifier_index == (number_of_amplifiers - 1))
                {
                    break;
                }

                if amplifier_index == (number_of_amplifiers - 1) {
                    is_first_run = false;
                }

                next_input_signal = output;
                amplifier_index = (amplifier_index + 1) % self.amplifiers.len();
            }

            for amplifier in self.amplifiers.iter_mut() {
                amplifier.reset_computer();
            }
        }

        Ok((best_phase_settings, best_output_signal))
    }

    fn get_all_phase_signal_variations(phase_settings: &[i32]) -> Vec<Vec<i32>> {
        let mut variations = Vec::new();
        let mut used = Vec::new();
        let mut unused = phase_settings.iter().map(|x| *x).collect::<VecDeque<i32>>();

        permutate(&mut used, &mut unused, &mut variations);

        variations
    }
}

fn permutate<T>(used: &mut Vec<T>, unused: &mut VecDeque<T>, permutations: &mut Vec<Vec<T>>)
where
    T: Clone,
{
    let number_of_unused = unused.len();

    if number_of_unused == 0 {
        permutations.push(used.to_vec());
    } else {
        for _ in 0..number_of_unused {
            used.push(unused.pop_front().unwrap());
            permutate(used, unused, permutations);
            unused.push_back(used.pop().unwrap());
        }
    }
}
#[cfg(test)]
mod tests {
    use std::panic;

    use super::*;

    use crate::intcode_computer::IntcodeComputer;

    const NAMES: [&'static str; 5] = ["A", "B", "C", "D", "E"];
    const PHASE_SETTINGS: [i32; 5] = [4, 3, 2, 1, 0];
    const OTHER_PHASE_SETTINGS: [i32; 5] = [9, 8, 7, 6, 5];
    const STARTING_INPUT_SIGNAL: i128 = 0;
    const PROGRAM: [i32; 17] = [
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, -1, -1,
    ];
    const OTHER_PROGRAM: [i32; 29] = [
        3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
        1005, 28, 6, 99, 0, 0, 5,
    ];

    #[test]
    fn test_new_amplifier() {
        run_amplifier_test(|amplifier| {
            let expected = Amplifier {
                name: String::from(NAMES[0]),
                phase_setting: PHASE_SETTINGS[0],
                input_signal: STARTING_INPUT_SIGNAL,
                intcode_computer: IntcodeComputer::new(PROGRAM.to_vec().as_slice()),
            };

            assert_eq!(amplifier, expected);
        });
    }

    #[test]
    fn test_amplifier_run_progrom() {
        run_amplifier_test(|mut amplifier| {
            let expected = Ok((IntcodeComputerResult::FINISHED, 4));

            let result = amplifier.run_program();

            assert_eq!(result, expected);
        });
    }

    #[test]
    fn test_amplifier_reset_computer() {
        run_amplifier_test(|mut amplifier| {
            let expected = Amplifier {
                name: String::from(NAMES[0]),
                phase_setting: PHASE_SETTINGS[0],
                input_signal: STARTING_INPUT_SIGNAL,
                intcode_computer: IntcodeComputer::new(PROGRAM.to_vec().as_slice()),
            };

            let result = amplifier.run_program();
            amplifier.reset_computer();

            assert!(result.is_ok());
            assert_eq!(amplifier, expected);
        });
    }

    fn run_amplifier_test<T>(test: T)
    where
        T: FnOnce(Amplifier) -> () + panic::UnwindSafe,
    {
        let intcode_computer = IntcodeComputer::new(PROGRAM.to_vec().as_slice());
        let amplifier = Amplifier::new(
            NAMES[0],
            PHASE_SETTINGS[0],
            STARTING_INPUT_SIGNAL,
            &intcode_computer,
        );

        let result = panic::catch_unwind(|| test(amplifier));

        assert!(result.is_ok());
    }

    #[test]
    fn test_amplifier_circuit_get_larget_output_signal() {
        run_amplifier_circuit_test(|mut amplifier_circuit| {
            let expected = Ok((PHASE_SETTINGS.to_vec(), 43210));

            let result = amplifier_circuit.get_largest_output_signal(&PHASE_SETTINGS);

            assert_eq!(result, expected);
        });
    }

    #[test]
    fn test_amplifier_circuit_get_larget_output_signal_feedback_loop() {
        let mut amplifier_circuit = AmplifierCircuit::new(&NAMES, &OTHER_PROGRAM);

        let expected = Ok((OTHER_PHASE_SETTINGS.to_vec(), 139629729));

        let result = amplifier_circuit.get_largest_output_signal(&OTHER_PHASE_SETTINGS);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_amplifier_circuit_get_all_phase_signal_variations() {
        let expected = vec![
            vec![0, 1, 2],
            vec![0, 2, 1],
            vec![1, 2, 0],
            vec![1, 0, 2],
            vec![2, 0, 1],
            vec![2, 1, 0],
        ];

        let result = AmplifierCircuit::get_all_phase_signal_variations(&[0, 1, 2]);

        assert_eq!(result, expected);
    }

    fn run_amplifier_circuit_test<T>(test: T)
    where
        T: FnOnce(AmplifierCircuit) -> () + panic::UnwindSafe,
    {
        let amplifier_circuit = AmplifierCircuit::new(&NAMES, &PROGRAM);

        let result = panic::catch_unwind(|| test(amplifier_circuit));

        assert!(result.is_ok());
    }
}
