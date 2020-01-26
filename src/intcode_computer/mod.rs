pub mod intcode_instruction;

use intcode_instruction::Opcode;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum IntcodeComputerStatus {
    NotStarted,
    WaitingForInput,
    Finished,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IntcodeComputer {
    current_program: HashMap<u128, i128>,
    current_index: u128,
    current_input: Option<i128>,
    current_status: IntcodeComputerStatus,
    original_program: HashMap<u128, i128>,
}

impl IntcodeComputer {
    pub fn new<A>(args: A) -> IntcodeComputer
    where
        A: Into<IntcodeComputer>,
    {
        args.into()
    }

    pub fn execute_program(&mut self) -> Option<i128> {
        let mut output = None;
        self.current_status = IntcodeComputerStatus::WaitingForInput;

        let mut opcode = Opcode::new(
            self.current_input.unwrap_or(0),
            &self.current_program,
            self.current_index,
        );

        if let Opcode::SaveInput(_, _) = opcode {
            if self.current_input.is_none() {
                return output;
            }
        }

        while let Some((opcode_execution_result, next_index)) =
            opcode.execute(&mut self.current_program, self.current_index)
        {
            let next_opcode = Opcode::new(
                self.current_input.unwrap_or(0),
                &self.current_program,
                next_index,
            );
            self.current_index = next_index;

            if let Opcode::Output(_) = opcode {
                output = Some(opcode_execution_result as i128);
            } else if let Opcode::SaveInput(_, _) = opcode {
                self.current_input = None;
            }

            if let Opcode::SaveInput(_, _) = next_opcode {
                if self.current_input.is_none() {
                    return output;
                }
            }

            opcode = next_opcode;
        }

        self.current_status = IntcodeComputerStatus::Finished;

        output
    }

    pub fn set_input(&mut self, input: i128) {
        self.current_input = Some(input);
    }

    pub fn replace_code_in_program(&mut self, code_index: usize, new_value: i32) {
        if let Some(code) = self.original_program.get_mut(&(code_index as u128)) {
            *code = new_value as i128;
        }

        if let Some(code) = self.current_program.get_mut(&(code_index as u128)) {
            *code = new_value as i128;
        }
    }

    pub fn reset(&mut self) {
        self.current_program = self.original_program.clone();
        self.current_index = 0;
        self.current_input = None;
        self.current_status = IntcodeComputerStatus::NotStarted;
    }

    pub fn get_current_memory(&self) -> HashMap<u128, i128> {
        self.current_program.clone()
    }

    pub fn get_status(&self) -> IntcodeComputerStatus {
        self.current_status.clone()
    }
}

impl From<&[i128]> for IntcodeComputer {
    fn from(a: &[i128]) -> IntcodeComputer {
        IntcodeComputer {
            current_program: slice_to_hashmap(a),
            current_index: 0,
            current_input: None,
            current_status: IntcodeComputerStatus::NotStarted,
            original_program: slice_to_hashmap(a),
        }
    }
}

impl From<&[i32]> for IntcodeComputer {
    fn from(a: &[i32]) -> IntcodeComputer {
        let temp: Vec<i128> = a.iter().map(|x| *x as i128).collect();

        IntcodeComputer::new(temp.as_slice())
    }
}

impl From<&[String]> for IntcodeComputer {
    fn from(a: &[String]) -> IntcodeComputer {
        let temp: Vec<i128> = a.iter().map(|s| s.parse::<i128>().unwrap()).collect();

        IntcodeComputer::new(temp.as_slice())
    }
}

pub fn slice_to_hashmap<N>(slice: &[N]) -> HashMap<u128, i128>
where
    N: Into<i128> + Copy,
{
    slice
        .iter()
        .enumerate()
        .map(|(index, &value)| (index as u128, value.into()))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_reader::to_string_vector;

    // new and replace: [1, 2, 3];
    // execute op1: [1, 1, 1, 4, 0, 5, 6, 0, 2, 4, 4, 13, 99, 0] idx 0
    // execute op2: " idx 8
    // execute till waiting: [3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]
    //fi: 566, si: 10
    // expected till : [3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 666, 10]

    #[test]
    fn test_new() {
        let values = vec![1, 2, 3, 4, 5];
        let hash_map: HashMap<u128, i128> = values
            .iter()
            .enumerate()
            .map(|(index, &value)| (index as u128, value as i128))
            .collect();

        let expected = IntcodeComputer {
            current_program: hash_map.clone(),
            current_index: 0,
            current_input: None,
            current_status: IntcodeComputerStatus::NotStarted,
            original_program: hash_map.clone(),
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
    fn test_replace_code_in_program() {
        let mut intcode_computer = IntcodeComputer::new(vec![1, 2, 3].as_slice());

        let expected = IntcodeComputer::new(vec![1, 4, 3].as_slice());

        intcode_computer.replace_code_in_program(1, 4);

        assert_eq!(intcode_computer, expected);
    }

    #[test]
    fn test_execute_program_opcode_1() {
        let mut intcode_computer =
            IntcodeComputer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99].as_slice());
        let user_input = 0;

        intcode_computer.set_input(user_input);
        intcode_computer.execute_program();

        let expected: HashMap<u128, i128> = [30, 1, 1, 4, 2, 5, 6, 0, 99]
            .iter()
            .enumerate()
            .map(|(index, &value)| (index as u128, value as i128))
            .collect();

        let result = intcode_computer.current_program;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_run_program_opcode_2() {
        let mut intcode_computer = IntcodeComputer::new(vec![2, 4, 4, 5, 99, 0].as_slice());
        let user_input = 0;

        intcode_computer.set_input(user_input);
        intcode_computer.execute_program();

        let expected: HashMap<u128, i128> = [2, 4, 4, 5, 99, 9801]
            .iter()
            .enumerate()
            .map(|(index, &value)| (index as u128, value as i128))
            .collect();

        let result = intcode_computer.current_program;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_execute_program_until_waiting() {
        let program = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let mut intcode_computer = IntcodeComputer::new(program.as_slice());

        let mut expected_computer = IntcodeComputer::new(program.as_slice());

        expected_computer.current_status = IntcodeComputerStatus::WaitingForInput;

        let output = intcode_computer.execute_program();

        assert_eq!(intcode_computer, expected_computer);
        assert_eq!(
            intcode_computer.get_status(),
            IntcodeComputerStatus::WaitingForInput
        );
        assert!(output.is_none());
    }

    #[test]
    fn test_execute_program_until_next_waiting() {
        let program = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let program_hash_map: HashMap<u128, i128> = program
            .iter()
            .enumerate()
            .map(|(index, &value)| (index as u128, value as i128))
            .collect();
        let mut intcode_computer = IntcodeComputer::new(program.as_slice());
        let first_input = 666;

        let expected_current_program = vec![
            3,           //0
            15,          //1
            3,           //2
            16,          //3
            1002,        //4
            16,          //5
            10,          //6
            16,          //7
            1,           //8
            16,          //9
            15,          //10
            15,          //11
            4,           //12
            15,          //13
            99,          //14
            first_input, //15
            0,           //16
        ];
        let expected_hash_map: HashMap<u128, i128> = expected_current_program
            .iter()
            .enumerate()
            .map(|(index, &value)| (index as u128, value as i128))
            .collect();
        let expected_computer = IntcodeComputer {
            current_program: expected_hash_map.clone(),
            current_index: 2,
            current_input: None,
            current_status: IntcodeComputerStatus::WaitingForInput,
            original_program: program_hash_map.clone(),
        };

        // waits at first input
        intcode_computer.execute_program();
        // sets first input
        intcode_computer.set_input(first_input);

        let output = intcode_computer.execute_program();

        assert_eq!(intcode_computer, expected_computer);
        assert_eq!(
            intcode_computer.get_status(),
            IntcodeComputerStatus::WaitingForInput
        );
        assert!(output.is_none());
    }

    #[test]
    fn test_execute_program_until_finished() {
        let program = vec![3, 11, 3, 12, 1, 12, 11, 11, 4, 11, 99, 0, 0];
        let program_hash_map: HashMap<u128, i128> = program
            .iter()
            .enumerate()
            .map(|(index, &value)| (index as u128, value as i128))
            .collect();
        let mut intcode_computer = IntcodeComputer::new(program.as_slice());
        let first_input = 656;
        let second_input = 10;

        let expected_current_program =
            vec![3, 11, 3, 12, 1, 12, 11, 11, 4, 11, 99, 666, second_input];
        let expected_hash_map: HashMap<u128, i128> = expected_current_program
            .iter()
            .enumerate()
            .map(|(index, &value)| (index as u128, value as i128))
            .collect();
        let expected_computer = IntcodeComputer {
            current_program: expected_hash_map.clone(),
            current_index: 10,
            current_input: None,
            current_status: IntcodeComputerStatus::Finished,
            original_program: program_hash_map.clone(),
        };
        let expected_result = Some(666);

        // waits at first input
        intcode_computer.execute_program();
        // sets first input
        intcode_computer.set_input(first_input);
        // waits at second input
        intcode_computer.execute_program();
        // sets second input
        intcode_computer.set_input(second_input);

        let result = intcode_computer.execute_program();

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

        intcode_computer.set_input(user_input);
        intcode_computer.execute_program();
        intcode_computer.reset();

        let expected_program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        let expected = IntcodeComputer::new(expected_program.as_slice());

        assert_eq!(intcode_computer, expected);
    }
}
