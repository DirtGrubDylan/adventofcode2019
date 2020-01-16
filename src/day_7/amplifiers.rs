use crate::incode_computer::IntcodeComputer;

#[derive(Debug, PartialEq)]
pub struct Amplifier {
    name: String,
    phase_setting: u32,
    input_signal: i32,
    incode_computer: IntcodeComputer,
}

