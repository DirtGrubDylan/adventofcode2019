use std::panic;

use crate::intcode_computer::IntcodeComputer;

#[derive(Debug, PartialEq)]
pub struct Amplifier {
    name: String,
    phase_setting: u32,
    input_signal: i32,
    intcode_computer: IntcodeComputer,
}

impl Amplifier {
    pub fn new(
        name: &str,
        phase_setting: u32,
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

    pub fn run_program(&mut self) -> i32 {
        unimplemented!()
    }

    pub fn reset(&mut self) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::intcode_computer::IntcodeComputer;

    const NAMES: [&'static str; 5] = ["A", "B", "C", "D", "E"];
    const PHASE_SETTINGS: [u32; 5] = [4, 3, 2, 1, 0];
    const STARTING_INPUT_SIGNAL: i32 = 0;
    const PROGRAM: [i32; 17] = [
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];

    #[test]
    fn test_new_amplifier() {
        run_amplifier_test(|amplifier| {
            let expected = Amplifier {
                name: String::from(NAMES[0]),
                phase_setting: 4,
                input_signal: 0,
                intcode_computer: IntcodeComputer::new(PROGRAM.to_vec().as_slice()),
            };

            assert_eq!(amplifier, expected);
        });
    }

    #[test]
    fn test_amplifier_run_progrom() {
        run_amplifier_test(|mut amplifier| {
            let expected = 259;

            let result = amplifier.run_program();

            assert_eq!(result, expected);
        });
    }

    #[test]
    fn test_amplifier_reset() {
        run_amplifier_test(|mut amplifier| {
            let expected = Amplifier {
                name: String::from(NAMES[0]),
                phase_setting: 4,
                input_signal: 0,
                intcode_computer: IntcodeComputer::new(PROGRAM.to_vec().as_slice()),
            };

            amplifier.run_program();
            amplifier.reset();

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
}
