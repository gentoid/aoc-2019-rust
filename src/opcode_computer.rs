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

    fn get(&mut self) -> isize {
        let result = self.memory[self.instruction_pointer];
        self.instruction_pointer += 1;
        result
    }

    fn tick(&mut self) {
        use Instruction::*;

        match Instruction::next(self) {
            OC01(params) => self.opcode_with_3_args(&params, |a, b| a + b),
            OC02(params) => self.opcode_with_3_args(&params, |a, b| a * b),
            OC03(param) => {
                let input = self.take_input();
                self.set_value(&param, input);
            }
            OC04(param) => self.put_output(self.value_for(&param)),
            OC99 => self.halted = true,
            _ => unimplemented!(),
        }
    }

    fn value_for(&self, param: &Param) -> isize {
        match param.mode {
            ParamMode::Positional => self.memory[param.value as usize],
            ParamMode::Immidiate => param.value,
        }
    }

    fn opcode_with_3_args(&mut self, params: &[Param;3],f: fn(isize, isize) -> isize) {
        let [p1, p2, p3] = params;
        let val1 = self.value_for(&p1);
        let val2 = self.value_for(&p2);
        self.set_value(&p3, f(val1, val2));
    }

    fn set_value(&mut self, param: &Param, value: isize) {
        match param.mode {
            ParamMode::Positional => self.memory[param.value as usize] = value,
            ParamMode::Immidiate => panic!("It's impossible to use immidiate mode to set value"),
        }
        
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

enum ParamMode {
    Positional,
    Immidiate,
}

struct Param {
    value: isize,
    mode: ParamMode,
}

impl Param {
    pub fn pos(value: isize) -> Self {
        Self {
            value,
            mode: ParamMode::Positional,
        }
    }

    pub fn imm(value: isize) -> Self {
        Self {
            value,
            mode: ParamMode::Immidiate,
        }
    }
}

enum Instruction {
    OC01([Param; 3]),
    OC02([Param; 3]),
    OC03(Param),
    OC04(Param),
    OC99,
}

impl Instruction {
    pub fn next(program: &mut Program) -> Self {
        use Instruction::*;
        let instruction = program.get();

        match instruction {
            1 => OC01([
                Param::pos(program.get()),
                Param::pos(program.get()),
                Param::pos(program.get()),
            ]),
            2 => OC02([
                Param::pos(program.get()),
                Param::pos(program.get()),
                Param::pos(program.get()),
            ]),
            3 => OC03(Param::pos(program.get())),
            4 => OC04(Param::pos(program.get())),
            99 => OC99,
            _ => unreachable!("Wrong instruction {}!", instruction),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puts_input_to_output() {
        let mut program = Program::new(vec![3, 0, 4, 0, 99], Some(vec![7]));
        program.run();
        assert_eq!(program.output, vec![7]);
    }

    #[test]
    fn multiplies_and_puts_to_the_latest() {
        let mut program = Program::new(vec![2, 4, 4, 5, 99, 0], None);
        program.run();
        assert_eq!(program.memory, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn sums_and_puts_to_the_first() {
        let mut program = Program::new(vec![1, 0, 0, 0, 99], None);
        program.run();
        assert_eq!(program.memory, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn overrides_99_in_the_middle() {
        let mut program = Program::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], None);
        program.run();
        assert_eq!(program.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
