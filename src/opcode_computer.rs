pub struct Program {
    memory: Vec<isize>,
    instruction_pointer: usize,
    halted: bool,
    input: Option<Vec<isize>>,
    input_pointer: usize,
    output: Vec<isize>,
}

impl Program {
    pub fn new(memory: Vec<isize>, input: Option<Vec<isize>>) -> Self {
        Self {
            memory,
            instruction_pointer: 0,
            halted: false,
            input,
            input_pointer: 0,
            output: vec![],
        }
    }

    pub fn run(&mut self) -> isize {
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

    fn opcode_with_3_args(&mut self, f: fn(isize, isize) -> isize) {
        let arg1 = self.get_arg(1);
        let arg2 = self.get_arg(2);
        self.set_arg(3, f(arg1, arg2));
        self.instruction_pointer += 4;
    }

    fn get_arg(&self, offset: usize) -> isize {
        self.memory[self.memory[self.instruction_pointer + offset] as usize]
    }

    fn set_arg(&mut self, offset: usize, value: isize) {
        let put_to = self.memory[self.instruction_pointer + offset].clone() as usize;
        self.memory[put_to] = value;
    }

    fn take_input(&mut self) -> isize {
        let input = self.input.as_ref().unwrap()[self.input_pointer];
        self.input_pointer += 1;
        input
    }

    fn put_output(&mut self, value: isize) {
        self.output.push(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puts_input_to_output() {
        let mut program = Program::new(vec![3,0,4,0,99], Some(vec![7]));
        program.run();
        assert_eq!(program.output, vec![7]);
    }

    #[test]
    fn multiplies_and_puts_to_the_latest() {
        let mut program = Program::new(vec![2,4,4,5,99,0], None);
        program.run();
        assert_eq!(program.memory, vec![2,4,4,5,99,9801]);
    }

    #[test]
    fn sums_and_puts_to_the_first() {
        let mut program = Program::new(vec![1,0,0,0,99], None);
        program.run();
        assert_eq!(program.memory, vec![2,0,0,0,99]);
    }

    #[test]
    fn overrides_99_in_the_middle() {
        let mut program = Program::new(vec![1,1,1,4,99,5,6,0,99], None);
        program.run();
        assert_eq!(program.memory, vec![30,1,1,4,2,5,6,0,99]);
    }
}
