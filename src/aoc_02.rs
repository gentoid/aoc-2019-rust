use std::fs;

fn read_and_parse() -> Vec<usize> {
    let content = fs::read_to_string("input-02.txt").unwrap();
    content
        .trim()
        .split(",")
        .map(|string| usize::from_str_radix(string.as_ref(), 10).unwrap())
        .collect()
}

struct Program {
    memory: Vec<usize>,
    address: usize,
    halted: bool,
}

impl Program {
    pub fn new(memory: Vec<usize>) -> Self {
        Self {
            memory,
            address: 0,
            halted: false,
        }
    }

    pub fn run(&mut self, noun: usize, verb: usize) -> usize {
        self.memory[1] = noun;
        self.memory[2] = verb;

        while !self.halted {
            self.tick();
        }

        self.memory[0]
    }

    fn tick(&mut self) {
        let opcode = self.memory[self.address];

        match opcode {
            1 => {
                let arg1 = self.memory[self.memory[self.address + 1]];
                let arg2 = self.memory[self.memory[self.address + 2]];
                let put_to = self.memory[self.address + 3].clone();
                self.memory[put_to] = arg1 + arg2;
            }
            2 => {
                let arg1 = self.memory[self.memory[self.address + 1]];
                let arg2 = self.memory[self.memory[self.address + 2]];
                let put_to = self.memory[self.address + 3].clone();
                self.memory[put_to] = arg1 * arg2;
            }
            99 => self.halted = true,
            _ => unreachable!("Wrong OpCode {}!", opcode),
        }

        self.address += 4;
    }
}

pub fn aoc_02_01() -> usize {
    Program::new(read_and_parse()).run(12, 2)
}

pub fn aoc_02_02() -> usize {
    let looking_for = 19690720;
    let initial_memory = read_and_parse();

    for noun in 0..100 {
        for verb in 0..100 {
            if Program::new(initial_memory.clone()).run(noun, verb) == looking_for {
                return noun * 100 + verb;
            }
        }
    }

    return 0;
}
