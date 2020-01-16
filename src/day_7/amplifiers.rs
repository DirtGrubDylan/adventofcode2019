use crate::intcode_computer::IntcodeComputer;

#[derive(Debug, PartialEq)]
pub struct Amplifier {
    name: String,
    phase_setting: i32,
    input_signal: i32,
    intcode_computer: IntcodeComputer,
}

impl Amplifier {
    pub fn new(
        name: &str,
        phase_setting: i32,
        input_signal: i32,
        intcode_computer: &IntcodeComputer,
    ) -> Amplifier {
        Amplifier {
            name: String::from(name),
            phase_setting: phase_setting,
            input_signal: input_signal,
            intcode_computer: intcode_computer.clone(),
        }
    }

    pub fn run_program(&mut self) -> Result<i32, String> {
        self.intcode_computer.run_program_until_first_input_opcode();

        self.intcode_computer
            .continue_program_until_next_input_opcode(self.phase_setting);

        self.intcode_computer
            .continue_program_until_next_input_opcode(self.input_signal)
            .ok_or(format!("Something went wrong for amplifier: {}", self.name))
    }

    pub fn reset_computer(&mut self) {
        self.intcode_computer.reset();
    }
}

#[derive(Debug)]
struct AmplifierCircuit {
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

    pub fn get_largest_output_signal(&mut self) -> Result<(Vec<i32>, i32), String> {
        unimplemented!()
    }

    fn get_all_phase_signal_variations(number_of_amplifiers: usize) -> Vec<Vec<i32>> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use super::*;

    use crate::intcode_computer::IntcodeComputer;

    const NAMES: [&'static str; 5] = ["A", "B", "C", "D", "E"];
    const PHASE_SETTINGS: [i32; 5] = [4, 3, 2, 1, 0];
    const STARTING_INPUT_SIGNAL: i32 = 0;
    const PROGRAM: [i32; 17] = [
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, -1, -1,
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
            let expected = Ok(4);

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

            let result = amplifier_circuit.get_largest_output_signal();

            assert_eq!(result, expected);
        });
    }

    #[test]
    fn test_amplifier_circuit_get_all_phase_signal_variations() {
        let expected = vec![
            vec![0, 1, 2],
            vec![0, 2, 1],
            vec![1, 0, 2],
            vec![1, 2, 0],
            vec![2, 0, 1],
            vec![2, 1, 0],
        ];

        let result = AmplifierCircuit::get_all_phase_signal_variations(3);

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
