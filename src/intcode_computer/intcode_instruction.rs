#[derive(Debug, PartialEq)]
pub enum Parameter {
    Position(i32),
    Immediate(i32),
}

impl Parameter {
    pub fn new(mode: i32, value: i32) -> Parameter {
        match mode {
            0 => Parameter::Position(value),
            1 => Parameter::Immediate(value),
            _ => panic!("Unexpected parameter_mode!"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    SaveInput(Parameter, Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    StoreIfLessThan(Parameter, Parameter, Parameter),
    StoreIfEquals(Parameter, Parameter, Parameter),
    Terminate,
}

impl Opcode {
    pub fn new(user_input: i32, program_memory: &[i32], current_index: usize) -> Opcode {
        let instruction_definitions = match program_memory.get(current_index) {
            Some(value) => value,
            None => panic!("current_index is outside of program_memory bounds!"),
        };

        let opcode_value = instruction_definitions % 100;

        let first_parameter_mode = (instruction_definitions / 100) % 10;
        let second_parameter_mode = (instruction_definitions / 1000) % 10;
        let third_parameter_mode = instruction_definitions / 10000;

        match opcode_value {
            1 => {
                let first_parameter =
                    Parameter::new(first_parameter_mode, program_memory[current_index + 1]);

                let second_parameter =
                    Parameter::new(second_parameter_mode, program_memory[current_index + 2]);

                let third_parameter =
                    Parameter::new(third_parameter_mode, program_memory[current_index + 3]);

                Opcode::Add(first_parameter, second_parameter, third_parameter)
            }
            2 => {
                let first_parameter =
                    Parameter::new(first_parameter_mode, program_memory[current_index + 1]);

                let second_parameter =
                    Parameter::new(second_parameter_mode, program_memory[current_index + 2]);

                let third_parameter =
                    Parameter::new(third_parameter_mode, program_memory[current_index + 3]);

                Opcode::Multiply(first_parameter, second_parameter, third_parameter)
            }
            3 => {
                let input_parameter = Parameter::new(1, user_input);

                let first_parameter =
                    Parameter::new(first_parameter_mode, program_memory[current_index + 1]);

                Opcode::SaveInput(input_parameter, first_parameter)
            }
            4 => {
                let first_parameter =
                    Parameter::new(first_parameter_mode, program_memory[current_index + 1]);

                Opcode::Output(first_parameter)
            }
            5 => {
                let first_parameter =
                    Parameter::new(first_parameter_mode, program_memory[current_index + 1]);

                let second_parameter =
                    Parameter::new(second_parameter_mode, program_memory[current_index + 2]);

                Opcode::JumpIfTrue(first_parameter, second_parameter)
            }
            6 => {
                let first_parameter =
                    Parameter::new(first_parameter_mode, program_memory[current_index + 1]);

                let second_parameter =
                    Parameter::new(second_parameter_mode, program_memory[current_index + 2]);

                Opcode::JumpIfFalse(first_parameter, second_parameter)
            }
            7 => {
                let first_parameter =
                    Parameter::new(first_parameter_mode, program_memory[current_index + 1]);

                let second_parameter =
                    Parameter::new(second_parameter_mode, program_memory[current_index + 2]);

                let third_parameter =
                    Parameter::new(third_parameter_mode, program_memory[current_index + 3]);

                Opcode::StoreIfLessThan(first_parameter, second_parameter, third_parameter)
            }
            8 => {
                let first_parameter =
                    Parameter::new(first_parameter_mode, program_memory[current_index + 1]);

                let second_parameter =
                    Parameter::new(second_parameter_mode, program_memory[current_index + 2]);

                let third_parameter =
                    Parameter::new(third_parameter_mode, program_memory[current_index + 3]);

                Opcode::StoreIfEquals(first_parameter, second_parameter, third_parameter)
            }
            99 => Opcode::Terminate,
            _ => panic!(
                "Unexpected opcode given (instruction, index): {:?}",
                (instruction_definitions, current_index)
            ),
        }
    }

    // Returns values of the Add, Multiply, SaveInput, and Output and the next index.
    // None if Terminate
    pub fn execute(
        &self,
        program_memory: &mut [i32],
        current_index: usize,
    ) -> Option<(i32, usize)> {
        match self {
            Opcode::Add(first_parameter, second_parameter, third_parameter) => {
                let first_value =
                    Self::get_parameter_value_from_memory(first_parameter, program_memory);

                let second_value =
                    Self::get_parameter_value_from_memory(second_parameter, program_memory);

                let save_index = match third_parameter {
                    Parameter::Position(index) => Self::transform_index(*index, program_memory),
                    Parameter::Immediate(_) => {
                        panic!("Cannot save a value with an immediate parameter!")
                    }
                };

                let sum = first_value + second_value;

                program_memory[save_index] = sum;

                Some((sum, current_index + 4))
            }
            Opcode::Multiply(first_parameter, second_parameter, third_parameter) => {
                let first_value =
                    Self::get_parameter_value_from_memory(first_parameter, program_memory);

                let second_value =
                    Self::get_parameter_value_from_memory(second_parameter, program_memory);

                let save_index = match third_parameter {
                    Parameter::Position(index) => Self::transform_index(*index, program_memory),
                    Parameter::Immediate(_) => {
                        panic!("Cannot save a value with an immediate parameter!")
                    }
                };

                let product = first_value * second_value;

                program_memory[save_index] = product;

                Some((product, current_index + 4))
            }
            Opcode::SaveInput(input_parameter, first_parameter) => {
                let input_value =
                    Self::get_parameter_value_from_memory(input_parameter, program_memory);

                let save_index = match first_parameter {
                    Parameter::Position(index) => Self::transform_index(*index, program_memory),
                    Parameter::Immediate(_) => {
                        panic!("Cannot save a value with an immediate parameter!")
                    }
                };

                program_memory[save_index] = input_value;

                Some((input_value, current_index + 2))
            }
            Opcode::Output(first_parameter) => {
                let output_value =
                    Self::get_parameter_value_from_memory(first_parameter, program_memory);

                Some((output_value, current_index + 2))
            }
            Opcode::JumpIfTrue(first_parameter, second_parameter) => {
                let first_value =
                    Self::get_parameter_value_from_memory(first_parameter, program_memory);

                let second_value =
                    Self::get_parameter_value_from_memory(second_parameter, program_memory);

                let mut success_value = 0;
                let mut next_index = current_index + 3;

                if first_value != 0 {
                    success_value = 1;
                    next_index = second_value as usize;
                }

                Some((success_value, next_index))
            }
            Opcode::JumpIfFalse(first_parameter, second_parameter) => {
                let first_value =
                    Self::get_parameter_value_from_memory(first_parameter, program_memory);

                let second_value =
                    Self::get_parameter_value_from_memory(second_parameter, program_memory);

                let mut success_value = 0;
                let mut next_index = current_index + 3;

                if first_value == 0 {
                    success_value = 1;
                    next_index = second_value as usize;
                }

                Some((success_value, next_index))
            }
            Opcode::StoreIfLessThan(first_parameter, second_parameter, third_parameter) => {
                let first_value =
                    Self::get_parameter_value_from_memory(first_parameter, program_memory);

                let second_value =
                    Self::get_parameter_value_from_memory(second_parameter, program_memory);

                let save_index = match third_parameter {
                    Parameter::Position(index) => Self::transform_index(*index, program_memory),
                    Parameter::Immediate(_) => {
                        panic!("Cannot save a value with an immediate parameter!")
                    }
                };

                let mut success_value = 0;

                if first_value < second_value {
                    success_value = 1;
                }

                program_memory[save_index] = success_value;

                Some((success_value, current_index + 4))
            }
            Opcode::StoreIfEquals(first_parameter, second_parameter, third_parameter) => {
                let first_value =
                    Self::get_parameter_value_from_memory(first_parameter, program_memory);

                let second_value =
                    Self::get_parameter_value_from_memory(second_parameter, program_memory);

                let save_index = match third_parameter {
                    Parameter::Position(index) => Self::transform_index(*index, program_memory),
                    Parameter::Immediate(_) => {
                        panic!("Cannot save a value with an immediate parameter!")
                    }
                };

                let mut success_value = 0;

                if first_value == second_value {
                    success_value = 1;
                }

                program_memory[save_index] = success_value;

                Some((success_value, current_index + 4))
            }
            Opcode::Terminate => None,
        }
    }

    fn get_parameter_value_from_memory(parameter: &Parameter, program_memory: &[i32]) -> i32 {
        match parameter {
            Parameter::Position(index) => {
                program_memory[Self::transform_index(*index, program_memory)]
            }
            Parameter::Immediate(value) => *value,
        }
    }

    // Transforms a negative index to wrap
    fn transform_index(index: i32, program_memory: &[i32]) -> usize {
        if index < 0 {
            ((program_memory.len() as i32) + index) as usize
        } else {
            index as usize
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_add() {
        let program_memory = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let user_input = 1;
        let current_index = 0;

        let expected = Opcode::Add(
            Parameter::Position(9),
            Parameter::Position(10),
            Parameter::Position(3),
        );

        let result = Opcode::new(user_input, &program_memory, current_index);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_multiply() {
        let program_memory = vec![1002, 4, 3, 4, 33];
        let user_input = 1;
        let current_index = 0;

        let expected = Opcode::Multiply(
            Parameter::Position(4),
            Parameter::Immediate(3),
            Parameter::Position(4),
        );

        let result = Opcode::new(user_input, &program_memory, current_index);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_save_input() {
        let program_memory = vec![3, 0, 4, 0, 99];
        let user_input = 1;
        let current_index = 0;

        let expected = Opcode::SaveInput(Parameter::Immediate(1), Parameter::Position(0));

        let result = Opcode::new(user_input, &program_memory, current_index);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_output() {
        let program_memory = vec![3, 0, 4, 0, 99];
        let user_input = 1;
        let current_index = 2;

        let expected = Opcode::Output(Parameter::Position(0));

        let result = Opcode::new(user_input, &program_memory, current_index);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_jump_if_true() {
        let program_memory = vec![5, 4, 5, 4, -1, 99];
        let user_input = 1;
        let current_index = 0;

        let expected = Opcode::JumpIfTrue(Parameter::Position(4), Parameter::Position(5));

        let result = Opcode::new(user_input, &program_memory, current_index);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_jump_if_false() {
        let program_memory = vec![106, 4, 5, 4, -1, 99];
        let user_input = 1;
        let current_index = 0;

        let expected = Opcode::JumpIfFalse(Parameter::Immediate(4), Parameter::Position(5));

        let result = Opcode::new(user_input, &program_memory, current_index);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_store_if_less_than() {
        let program_memory = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let user_input = 1;
        let current_index = 2;

        let expected = Opcode::StoreIfLessThan(
            Parameter::Position(9),
            Parameter::Position(10),
            Parameter::Position(9),
        );

        let result = Opcode::new(user_input, &program_memory, current_index);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_store_if_equals() {
        let program_memory = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let user_input = 1;
        let current_index = 2;

        let expected = Opcode::StoreIfEquals(
            Parameter::Immediate(-1),
            Parameter::Immediate(8),
            Parameter::Position(3),
        );

        let result = Opcode::new(user_input, &program_memory, current_index);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_terminate() {
        let program_memory = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let user_input = 1;
        let current_index = 8;

        let expected = Opcode::Terminate;

        let result = Opcode::new(user_input, &program_memory, current_index);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_execute_add() {
        let mut program_memory = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let user_input = 1;
        let current_index = 0;
        let opcode = Opcode::new(user_input, &program_memory, current_index);

        let expected_output = Some((70, 4));
        let expected_program_memory = vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        let result = opcode.execute(&mut program_memory, current_index);

        assert_eq!(result, expected_output);
        assert_eq!(program_memory, expected_program_memory);
    }

    #[test]
    fn test_execute_multiply() {
        let mut program_memory = vec![1002, 4, 3, 4, 33];
        let user_input = 1;
        let current_index = 0;
        let opcode = Opcode::new(user_input, &program_memory, current_index);

        let expected_output = Some((99, 4));
        let expected_program_memory = vec![1002, 4, 3, 4, 99];

        let result = opcode.execute(&mut program_memory, current_index);

        assert_eq!(result, expected_output);
        assert_eq!(program_memory, expected_program_memory);
    }

    #[test]
    fn test_execute_save_input() {
        let mut program_memory = vec![3, 0, 4, 0, 99];
        let user_input = 1;
        let current_index = 0;
        let opcode = Opcode::new(user_input, &program_memory, current_index);

        let expected_output = Some((1, 2));
        let expected_program_memory = vec![1, 0, 4, 0, 99];

        let result = opcode.execute(&mut program_memory, current_index);

        assert_eq!(result, expected_output);
        assert_eq!(program_memory, expected_program_memory);
    }

    #[test]
    fn test_execute_output() {
        let mut program_memory = vec![3, 0, 4, 0, 99];
        let user_input = 1;
        let current_index = 2;
        let opcode = Opcode::new(user_input, &program_memory, current_index);

        let expected_output = Some((3, 4));
        let expected_program_memory = vec![3, 0, 4, 0, 99];

        let result = opcode.execute(&mut program_memory, current_index);

        assert_eq!(result, expected_output);
        assert_eq!(program_memory, expected_program_memory);
    }

    #[test]
    fn test_execute_jump_if_true() {
        let mut program_memory = vec![3, 3, 1105, 1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let user_input = 1;
        let current_index = 2;
        let opcode = Opcode::new(user_input, &program_memory, current_index);

        let expected_output = Some((1, 9));
        let expected_program_memory = program_memory.clone();

        let result = opcode.execute(&mut program_memory, current_index);

        assert_eq!(result, expected_output);
        assert_eq!(program_memory, expected_program_memory);
    }

    #[test]
    fn test_execute_no_jump_if_true() {
        let mut program_memory = vec![3, 3, 5, 6, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let user_input = 1;
        let current_index = 2;
        let opcode = Opcode::new(user_input, &program_memory, current_index);

        let expected_output = Some((0, 5));
        let expected_program_memory = program_memory.clone();

        let result = opcode.execute(&mut program_memory, current_index);

        assert_eq!(result, expected_output);
        assert_eq!(program_memory, expected_program_memory);
    }

    #[test]
    fn test_execute_jump_if_false() {
        let mut program_memory = vec![3, 3, 1106, 0, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let user_input = 1;
        let current_index = 2;
        let opcode = Opcode::new(user_input, &program_memory, current_index);

        let expected_output = Some((1, 9));
        let expected_program_memory = program_memory.clone();

        let result = opcode.execute(&mut program_memory, current_index);

        assert_eq!(result, expected_output);
        assert_eq!(program_memory, expected_program_memory);
    }

    #[test]
    fn test_execute_no_jump_if_false() {
        let mut program_memory = vec![3, 3, 6, 6, 9, 1101, 1, 0, 12, 4, 12, 99, 1];
        let user_input = 1;
        let current_index = 2;
        let opcode = Opcode::new(user_input, &program_memory, current_index);

        let expected_output = Some((0, 5));
        let expected_program_memory = program_memory.clone();

        let result = opcode.execute(&mut program_memory, current_index);

        assert_eq!(result, expected_output);
        assert_eq!(program_memory, expected_program_memory);
    }

    #[test]
    fn test_execute_store_if_less_than() {
        let mut program_memory = vec![3, 3, 1107, 7, 8, 3, 4, 3, 99];
        let user_input = 1;
        let current_index = 2;
        let opcode = Opcode::new(user_input, &program_memory, current_index);

        let expected_output = Some((1, 6));
        let expected_program_memory = vec![3, 3, 1107, 1, 8, 3, 4, 3, 99];

        let result = opcode.execute(&mut program_memory, current_index);

        assert_eq!(result, expected_output);
        assert_eq!(program_memory, expected_program_memory);
    }

    #[test]
    fn test_execute_no_store_if_less_than() {
        let mut program_memory = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 8, 8];
        let user_input = 1;
        let current_index = 2;
        let opcode = Opcode::new(user_input, &program_memory, current_index);

        let expected_output = Some((0, 6));
        let expected_program_memory = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8];

        let result = opcode.execute(&mut program_memory, current_index);

        assert_eq!(result, expected_output);
        assert_eq!(program_memory, expected_program_memory);
    }

    #[test]
    fn test_execute_store_if_equals() {
        let mut program_memory = vec![3, 3, 1108, 8, 8, 3, 4, 3, 99];
        let user_input = 1;
        let current_index = 2;
        let opcode = Opcode::new(user_input, &program_memory, current_index);

        let expected_output = Some((1, 6));
        let expected_program_memory = vec![3, 3, 1108, 1, 8, 3, 4, 3, 99];

        let result = opcode.execute(&mut program_memory, current_index);

        assert_eq!(result, expected_output);
        assert_eq!(program_memory, expected_program_memory);
    }

    #[test]
    fn test_execute_no_store_if_equals() {
        let mut program_memory = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let user_input = 1;
        let current_index = 2;
        let opcode = Opcode::new(user_input, &program_memory, current_index);

        let expected_output = Some((0, 6));
        let expected_program_memory = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 0, 8];

        let result = opcode.execute(&mut program_memory, current_index);

        assert_eq!(result, expected_output);
        assert_eq!(program_memory, expected_program_memory);
    }

    #[test]
    fn test_execute_terminate() {
        let mut program_memory = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let user_input = 1;
        let current_index = 8;
        let opcode = Opcode::new(user_input, &program_memory, current_index);

        let expected_output = None;
        let expected_program_memory = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        let result = opcode.execute(&mut program_memory, current_index);

        assert_eq!(result, expected_output);
        assert_eq!(program_memory, expected_program_memory);
    }

    #[test]
    fn test_get_parameter_value_from_memory_position_positive() {
        let program_memory = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let parameter = Parameter::new(0, 2);

        let expected = 10;

        let result = Opcode::get_parameter_value_from_memory(&parameter, &program_memory);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_parameter_value_from_memory_position_negative() {
        let program_memory = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let parameter = Parameter::new(0, -2);

        let expected = 40;

        let result = Opcode::get_parameter_value_from_memory(&parameter, &program_memory);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_parameter_value_from_memory_immediate() {
        let program_memory = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let parameter = Parameter::new(1, 2);

        let expected = 2;

        let result = Opcode::get_parameter_value_from_memory(&parameter, &program_memory);

        assert_eq!(result, expected);
    }
}
