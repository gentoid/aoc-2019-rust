use crate::{opcode_computer::OpcodeComputer, read_input::read_intcode_program};

pub fn aoc_02_01() -> isize {
    init_with_noun_verb(12, 2).run()
}

pub fn aoc_02_02() -> isize {
    let looking_for = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {
            if init_with_noun_verb(noun, verb).run() == looking_for {
                return noun * 100 + verb;
            }
        }
    }

    return 0;
}

fn init_with_noun_verb(noun: isize, verb: isize) -> OpcodeComputer {
    let mut memory = read_intcode_program(2);
    memory[1] = noun;
    memory[2] = verb;

    OpcodeComputer::new(&memory)
}
