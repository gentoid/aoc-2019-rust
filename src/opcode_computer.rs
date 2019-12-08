pub struct Program {
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
