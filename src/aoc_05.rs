use crate::{opcode_computer::OpcodeComputer, read_input::read_intcode_program};

pub fn aoc_05_01() -> isize {
    let memory = read_intcode_program(5);
    let mut program = OpcodeComputer::new(&memory);
    program.add_input(&1).run();

    program.get_output().unwrap()
}

pub fn aoc_05_02() -> isize {
    let memory = read_intcode_program(5);
    let mut program = OpcodeComputer::new(&memory);
    program.add_input(&5).run();

    program.get_output().unwrap()
}
