pub mod intcode_instruction;

use intcode_instruction::Opcode;

#[derive(Debug, PartialEq)]
pub struct IntcodeComputer {
    current_program: Vec<i32>,
    current_index: usize,
    original_program: Vec<i32>,
}

impl IntcodeComputer {
    pub fn new<A>(args: A) -> IntcodeComputer
    where
        A: Into<IntcodeComputer>,
    {
        args.into()
    }

    pub fn run_program(&mut self, user_input: i32) -> Vec<i32> {
        let mut output_values = Vec::new();

        let mut opcode = Opcode::new(user_input, &self.current_program, self.current_index);

        while let Some((opcode_execution_result, next_index)) =
            opcode.execute(&mut self.current_program, self.current_index)
        {
            if let Opcode::Output(_) = opcode {
                output_values.push(opcode_execution_result);
            }

            self.current_index = next_index;
            opcode = Opcode::new(user_input, &self.current_program, self.current_index);
        }

        output_values
    }

    pub fn run_program_until_first_input_opcode(&mut self) -> Option<i32> {
        let fake_input = 0;

        if let Opcode::SaveInput(_, _) = Opcode::new(fake_input, &self.current_program, self.current_index) {
            None
        } else {
            self.continue_program_until_next_input_opcode(fake_input)
        }
    }

    pub fn continue_program_until_next_input_opcode(&mut self, input: i32) -> Option<i32> {
        let mut result = None;

        let mut opcode = Opcode::new(input, &self.current_program, self.current_index);

        while let Some((opcode_execution_result, next_index)) =
            opcode.execute(&mut self.current_program, self.current_index)
        {
            self.current_index = next_index;
            opcode = Opcode::new(input, &self.current_program, self.current_index);

            if let Opcode::SaveInput(_, _) = opcode {
                result = None;
                break;
            } else if let Opcode::Output(_) = opcode {
                result = Some(opcode_execution_result);
            }
        }

        result
    }

    pub fn replace_code_in_program(&mut self, code_index: usize, new_value: i32) {
        if let Some(code) = self.original_program.get_mut(code_index) {
            *code = new_value;
        }

        if let Some(code) = self.current_program.get_mut(code_index) {
            *code = new_value;
        }
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
        self.current_program = self.original_program.clone();
    }

    pub fn get_current_memory(&self) -> Vec<i32> {
        self.current_program.clone()
    }
}

impl From<&[i32]> for IntcodeComputer {
    fn from(a: &[i32]) -> IntcodeComputer {
        IntcodeComputer {
            current_program: a.clone().to_vec(),
            current_index: 0,
            original_program: a.to_vec(),
        }
    }
}

impl From<&[String]> for IntcodeComputer {
    fn from(a: &[String]) -> IntcodeComputer {
        let temp: Vec<i32> = a.iter().map(|s| s.parse::<i32>().unwrap()).collect();

        IntcodeComputer::new(temp.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_reader::to_string_vector;

    #[test]
    fn test_new() {
        let values = vec![1, 2, 3, 4, 5];

        let expected = IntcodeComputer {
            current_program: values.clone(),
            current_index: 0,
            original_program: values.clone(),
        };

        let result = IntcodeComputer::new(values.as_slice());

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_str_vec() {
        let values = vec![String::from("1"), String::from("2"), String::from("3")];

        let expected = IntcodeComputer::new(vec![1, 2, 3].as_slice());

        let result = IntcodeComputer::new(values.as_slice());

        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_from_file() {
        let values: Vec<String> = to_string_vector("test_inputs/day_2_part_1.txt")
            .unwrap()
            .get(0)
            .unwrap()
            .split(",")
            .map(|s| String::from(s))
            .collect();

        let expected =
            IntcodeComputer::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50].as_slice());

        let result = IntcodeComputer::new(values.as_slice());

        assert_eq!(result, expected);
    }

    #[test]
    fn test_replace_code_in_program() {
        let mut intcode_computer = IntcodeComputer::new(vec![1, 2, 3].as_slice());

        let expected = IntcodeComputer::new(vec![1, 4, 3].as_slice());

        intcode_computer.replace_code_in_program(1, 4);

        assert_eq!(intcode_computer, expected);
    }

    #[test]
    fn test_run_program_opcode_1() {
        let mut intcode_computer =
            IntcodeComputer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99].as_slice());
        let user_input = 0;

        intcode_computer.run_program(user_input);

        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];

        let result = intcode_computer.current_program;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_run_program_opcode_2() {
        let mut intcode_computer = IntcodeComputer::new(vec![2, 4, 4, 5, 99, 0].as_slice());
        let user_input = 0;

        intcode_computer.run_program(user_input);

        let expected = vec![2, 4, 4, 5, 99, 9801];

        let result = intcode_computer.current_program;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_run_program_from_file() {
        let values: Vec<String> = to_string_vector("test_inputs/day_2_part_1.txt")
            .unwrap()
            .get(0)
            .unwrap()
            .split(",")
            .map(|s| String::from(s))
            .collect();
        let user_input = 0;

        let mut intcode_computer = IntcodeComputer::new(values.as_slice());

        intcode_computer.run_program(user_input);

        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        let result = intcode_computer.current_program;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_run_program_until_first_input_opcode() {
        let program = vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0];
        let mut intcode_computer = IntcodeComputer::new(program.as_slice());

        let expected_computer = IntcodeComputer::new(program.as_slice());

        let result = intcode_computer.run_program_until_first_input_opcode();

        assert_eq!(intcode_computer, expected_computer);
        assert!(result.is_none());
    }

    #[test]
    fn test_continue_program_until_next_first_input_opcode_single_run() {
        let program = vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0];
        let mut intcode_computer = IntcodeComputer::new(program.as_slice());
        let first_input = 666;

        let expected_current_program = vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, first_input, 0];
        let expected_computer = IntcodeComputer {
            current_program: expected_current_program.clone(),
            current_index: 2,
            original_program: program.clone()
        };

        intcode_computer.run_program_until_first_input_opcode();

        let result = intcode_computer.continue_program_until_next_input_opcode(first_input);

        assert_eq!(intcode_computer, expected_computer);
        assert!(result.is_none());
    }

    #[test]
    fn test_continue_program_until_next_first_input_opcode_run_to_some() {
        let program = vec![3, 11, 3, 12, 1, 12, 11, 11, 4, 11, 99, 0, 0];
        let mut intcode_computer = IntcodeComputer::new(program.as_slice());
        let first_input = 656;
        let second_input = 10;

        let expected_current_program = vec![3, 11, 3, 12, 1, 12, 11, 11, 4, 11, 99, 666, second_input];
        let expected_computer = IntcodeComputer {
            current_program: expected_current_program.clone(),
            current_index: 10,
            original_program: program.clone()
        };
        let expected_result = Some(666);

        intcode_computer.run_program_until_first_input_opcode();
        intcode_computer.continue_program_until_next_input_opcode(first_input);
        let result = intcode_computer.continue_program_until_next_input_opcode(second_input);

        assert_eq!(intcode_computer, expected_computer);
        assert_eq!(result, expected_result);
    }


    #[test]
    fn test_reset() {
        let values: Vec<String> = to_string_vector("test_inputs/day_2_part_1.txt")
            .unwrap()
            .get(0)
            .unwrap()
            .split(",")
            .map(|s| String::from(s))
            .collect();
        let user_input = 0;

        let mut intcode_computer = IntcodeComputer::new(values.as_slice());

        intcode_computer.run_program(user_input);
        intcode_computer.reset();

        let expected_program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        let expected = IntcodeComputer::new(expected_program.as_slice());

        assert_eq!(intcode_computer, expected);
    }
}
