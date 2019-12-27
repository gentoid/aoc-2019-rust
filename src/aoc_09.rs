use {crate::{opcode_computer::OpcodeComputer, read_input::read_intcode_program}};

pub fn aoc_09_01() -> isize {
    let mut computer = OpcodeComputer::new(&read_intcode_program(9));
    computer.add_input(&1);
    computer.run();
    computer.get_output().unwrap()
}

pub fn aoc_09_02() -> isize {
    let mut computer = OpcodeComputer::new(&read_intcode_program(9));
    computer.add_input(&2);
    computer.run();
    computer.get_output().unwrap()
}
