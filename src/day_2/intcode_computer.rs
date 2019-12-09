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

    pub fn run_program(&self) -> Vec<i32> {
        let mut result: Vec<i32> = self.program.clone();

        let mut index: usize = 0;

        while index < result.len() {
            match result.get(index) {
                Some(1) => Self::run_opcode_1(&mut result, index),
                Some(2) => Self::run_opcode_2(&mut result, index),
                Some(99) => break,
                _ => panic!(
                    "Something went horribly wrong for `run_program` at index {:?}!",
                    index
                ),
            }

            index += 4;
        }

        result
    }

    pub fn replace_code_in_program(&mut self, code_index: usize, new_value: i32) {
        if let Some(code) = self.program.get_mut(code_index) {
            *code = new_value;
        }
    }

    fn run_opcode_1(program: &mut [i32], opcode_index: usize) {
        let first_fetch_index = program[opcode_index + 1] as usize;
        let second_fetch_index = program[opcode_index + 2] as usize;
        let set_index = program[opcode_index + 3] as usize;

        let sum_result = program[first_fetch_index] + program[second_fetch_index];

        program[set_index] = sum_result;
    }

    fn run_opcode_2(program: &mut [i32], opcode_index: usize) {
        let first_fetch_index = program[opcode_index + 1] as usize;
        let second_fetch_index = program[opcode_index + 2] as usize;
        let set_index = program[opcode_index + 3] as usize;

        let product_result = program[first_fetch_index] * program[second_fetch_index];

        program[set_index] = product_result;
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
    fn test_run_opcode_1() {
        let mut values = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];

        let expected = vec![1, 1, 1, 4, 2, 5, 6, 0, 99];

        IntcodeComputer::run_opcode_1(&mut values, 0);

        assert_eq!(values, expected);
    }

    #[test]
    fn test_run_opcode_2() {
        let mut values = vec![2, 4, 4, 5, 99, 0];

        let expected = vec![2, 4, 4, 5, 99, 9801];

        IntcodeComputer::run_opcode_2(&mut values, 0);

        assert_eq!(values, expected);
    }

    #[test]
    fn test_run_program_opcode_1() {
        let intcode_computer = IntcodeComputer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99].as_slice());

        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];

        let result = intcode_computer.run_program();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_run_program_opcode_2() {
        let intcode_computer = IntcodeComputer::new(vec![2, 4, 4, 5, 99, 0].as_slice());

        let expected = vec![2, 4, 4, 5, 99, 9801];

        let result = intcode_computer.run_program();

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

        let intcode_computer = IntcodeComputer::new(values.as_slice());

        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        let result = intcode_computer.run_program();

        assert_eq!(result, expected);
    }
}
