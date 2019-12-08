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

    pub fn run(&mut self) -> usize {
        while !self.halted {
            self.tick();
        }

        self.memory[0]
    }

    fn tick(&mut self) {
        let opcode = self.memory[self.address];

        match opcode {
            1 => self.opcode_with_3_args(|a, b| a + b),
            2 => self.opcode_with_3_args(|a, b| a * b),
            99 => {
                self.halted = true;
                self.address += 1;
            }
            _ => unreachable!("Wrong OpCode {}!", opcode),
        }
    }

    fn opcode_with_3_args(&mut self, f: fn(usize, usize) -> usize) {
        let arg1 = self.memory[self.memory[self.address + 1]];
        let arg2 = self.memory[self.memory[self.address + 2]];
        let put_to = self.memory[self.address + 3].clone();
        self.memory[put_to] = f(arg1, arg2);
        self.address += 4;
    }
}
