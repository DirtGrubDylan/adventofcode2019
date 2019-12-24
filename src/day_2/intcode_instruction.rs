#[derive(Debug, PartialEq)]
pub enum Parameter {
    Position(i32),
    Immediate(i32),
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    SaveInput(Parameter, Parameter),
    Output(Parameter),
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
                let first_parameter = match first_parameter_mode {
                    0 => Parameter::Position(program_memory[current_index + 1]),
                    1 => Parameter::Immediate(program_memory[current_index + 1]),
                    _ => panic!("Unexpected first_parameter_mode!"),
                };

                let second_parameter = match second_parameter_mode {
                    0 => Parameter::Position(program_memory[current_index + 2]),
                    1 => Parameter::Immediate(program_memory[current_index + 2]),
                    _ => panic!("Unexpected second_parameter_mode!"),
                };

                let third_parameter = match third_parameter_mode {
                    0 => Parameter::Position(program_memory[current_index + 3]),
                    1 => Parameter::Immediate(program_memory[current_index + 3]),
                    _ => panic!("Unexpected third_parameter_mode!"),
                };

                Opcode::Add(first_parameter, second_parameter, third_parameter)
            }
            2 => {
                let first_parameter = match first_parameter_mode {
                    0 => Parameter::Position(program_memory[current_index + 1]),
                    1 => Parameter::Immediate(program_memory[current_index + 1]),
                    _ => panic!("Unexpected first_parameter_mode!"),
                };

                let second_parameter = match second_parameter_mode {
                    0 => Parameter::Position(program_memory[current_index + 2]),
                    1 => Parameter::Immediate(program_memory[current_index + 2]),
                    _ => panic!("Unexpected second_parameter_mode!"),
                };

                let third_parameter = match third_parameter_mode {
                    0 => Parameter::Position(program_memory[current_index + 3]),
                    1 => Parameter::Immediate(program_memory[current_index + 3]),
                    _ => panic!("Unexpected third_parameter_mode!"),
                };

                Opcode::Multiply(first_parameter, second_parameter, third_parameter)
            }
            3 => {
                let input_parameter = Parameter::Immediate(user_input);

                let first_parameter = match first_parameter_mode {
                    0 => Parameter::Position(program_memory[current_index + 1]),
                    1 => Parameter::Immediate(program_memory[current_index + 1]),
                    _ => panic!("Unexpected first_parameter_mode!"),
                };

                Opcode::SaveInput(input_parameter, first_parameter)
            }
            4 => {
                let first_parameter = match first_parameter_mode {
                    0 => Parameter::Position(program_memory[current_index + 1]),
                    1 => Parameter::Immediate(program_memory[current_index + 1]),
                    _ => panic!("Unexpected first_parameter_mode!"),
                };

                Opcode::Output(first_parameter)
            }
            99 => Opcode::Terminate,
            _ => panic!("Unexpected opcode given!"),
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
            Terminate => None,
            _ => unimplemented!(),
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

        let expected_output = Some((1, 4));
        let expected_program_memory = vec![3, 0, 4, 0, 99];

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
}
