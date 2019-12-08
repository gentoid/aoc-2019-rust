pub struct Program {
    memory: Vec<usize>,
    instruction_pointer: usize,
    halted: bool,
    input: Option<Vec<usize>>,
    input_pointer: usize,
    output: Vec<usize>,
}

impl Program {
    pub fn new(memory: Vec<usize>, input: Option<Vec<usize>>) -> Self {
        Self {
            memory,
            instruction_pointer: 0,
            halted: false,
            input,
            input_pointer: 0,
            output: vec![],
        }
    }

    pub fn run(&mut self) -> usize {
        while !self.halted {
            self.tick();
        }

        self.memory[0]
    }

    fn tick(&mut self) {
        let opcode = self.memory[self.instruction_pointer];

        match opcode {
            1 => self.opcode_with_3_args(|a, b| a + b),
            2 => self.opcode_with_3_args(|a, b| a * b),
            3 => {
                let input = self.take_input();
                self.set_arg(1, input);
                self.instruction_pointer += 2;
            }
            4 => {
                self.put_output(self.get_arg(1));
                self.instruction_pointer += 2;
            }
            99 => {
                self.halted = true;
                self.instruction_pointer += 1;
            }
            _ => unreachable!("Wrong OpCode {}!", opcode),
        }
    }

    fn opcode_with_3_args(&mut self, f: fn(usize, usize) -> usize) {
        let arg1 = self.get_arg(1);
        let arg2 = self.get_arg(2);
        self.set_arg(3, f(arg1, arg2));
        self.instruction_pointer += 4;
    }

    fn get_arg(&self, offset: usize) -> usize {
        self.memory[self.memory[self.instruction_pointer + offset]]
    }

    fn set_arg(&mut self, offset: usize, value: usize) {
        let put_to = self.memory[self.instruction_pointer + offset].clone();
        self.memory[put_to] = value;
    }

    fn take_input(&mut self) -> usize {
        let input = self.input.as_ref().unwrap()[self.input_pointer];
        self.input_pointer += 1;
        input
    }

    fn put_output(&mut self, value: usize) {
        self.output.push(value);
    }
}
