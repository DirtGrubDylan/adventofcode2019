pub mod intcode_instruction;

use intcode_instruction::Opcode;

#[derive(Debug, PartialEq)]
pub struct IntcodeComputer {
    program: Vec<i32>,
}

impl IntcodeComputer {
    pub fn new<A>(args: A) -> IntcodeComputer
    where
        A: Into<IntcodeComputer>,
    {
        args.into()
    }

    pub fn run_program(&self, user_input: i32) -> (Vec<i32>, Vec<i32>) {
        let mut result: Vec<i32> = self.program.clone();
        let mut output_values = Vec::new();

        let mut index: usize = 0;

        let mut opcode = Opcode::new(user_input, &result, index);

        while let Some((opcode_execution_result, next_index)) = opcode.execute(&mut result, index) {
            if let Opcode::Output(_) = opcode {
                output_values.push(opcode_execution_result);
            }

            index = next_index;
            opcode = Opcode::new(user_input, &result, index);
        }

        (result, output_values)
    }

    pub fn replace_code_in_program(&mut self, code_index: usize, new_value: i32) {
        if let Some(code) = self.program.get_mut(code_index) {
            *code = new_value;
        }
    }
}

impl From<&[i32]> for IntcodeComputer {
    fn from(a: &[i32]) -> IntcodeComputer {
        IntcodeComputer {
            program: a.to_vec(),
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
            program: values.clone(),
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
        let intcode_computer = IntcodeComputer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99].as_slice());
        let user_input = 0;

        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];

        let (result, _) = intcode_computer.run_program(user_input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_run_program_opcode_2() {
        let intcode_computer = IntcodeComputer::new(vec![2, 4, 4, 5, 99, 0].as_slice());
        let user_input = 0;

        let expected = vec![2, 4, 4, 5, 99, 9801];

        let (result, _) = intcode_computer.run_program(user_input);

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

        let intcode_computer = IntcodeComputer::new(values.as_slice());

        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        let (result, _) = intcode_computer.run_program(user_input);

        assert_eq!(result, expected);
    }
}
