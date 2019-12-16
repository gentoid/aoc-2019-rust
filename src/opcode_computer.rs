#[derive(Debug)]
pub struct OpcodeComputer {
    memory: Vec<isize>,
    instruction_pointer: usize,
    halted: bool,
    input: Option<Vec<isize>>,
    input_pointer: usize,
    pub output: Vec<isize>,
}

impl OpcodeComputer {
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
            OC05(params) => {
                if self.value_for(&params[0]) != 0 {
                    self.instruction_pointer = self.value_for(&params[1]) as usize;
                }
            }
            OC06(params) => {
                if self.value_for(&params[0]) == 0 {
                    self.instruction_pointer = self.value_for(&params[1]) as usize;
                }
            }
            OC07(params) => {
                if self.value_for(&params[0]) < self.value_for(&params[1]) {
                    self.set_value(&params[2], 1)
                } else {
                    self.set_value(&params[2], 0)
                }
            }
            OC08(params) => {
                if self.value_for(&params[0]) == self.value_for(&params[1]) {
                    self.set_value(&params[2], 1)
                } else {
                    self.set_value(&params[2], 0)
                }
            }
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

    fn opcode_with_3_args(&mut self, params: &[Param; 3], f: fn(isize, isize) -> isize) {
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

#[derive(Debug, PartialEq)]
enum ParamMode {
    Positional,
    Immidiate,
}

impl ParamMode {
    fn new(value: isize) -> Self {
        match value {
            0 => ParamMode::Positional,
            1 => ParamMode::Immidiate,
            _ => unreachable!("Incorrect value to init ParamMode: {}", value),
        }
    }
}

struct Param {
    value: isize,
    mode: ParamMode,
}

impl Param {
    pub fn new(value: isize, mode: ParamMode) -> Self {
        Self { value, mode }
    }
}

enum Instruction {
    OC01([Param; 3]),
    OC02([Param; 3]),
    OC03(Param),
    OC04(Param),
    OC05([Param; 2]),
    OC06([Param; 2]),
    OC07([Param; 3]),
    OC08([Param; 3]),
    OC99,
}

impl Instruction {
    pub fn next(program: &mut OpcodeComputer) -> Self {
        use Instruction::*;
        let instruction = program.get();

        match Instruction::parse(instruction) {
            (1, [m1, m2, m3]) => OC01([
                Param::new(program.get(), m1),
                Param::new(program.get(), m2),
                Param::new(program.get(), m3),
            ]),
            (2, [m1, m2, m3]) => OC02([
                Param::new(program.get(), m1),
                Param::new(program.get(), m2),
                Param::new(program.get(), m3),
            ]),
            (3, [m, _, _]) => OC03(Param::new(program.get(), m)),
            (4, [m, _, _]) => OC04(Param::new(program.get(), m)),
            (5, [m1, m2, _]) => {
                OC05([Param::new(program.get(), m1), Param::new(program.get(), m2)])
            }
            (6, [m1, m2, _]) => {
                OC06([Param::new(program.get(), m1), Param::new(program.get(), m2)])
            }
            (7, [m1, m2, m3]) => OC07([
                Param::new(program.get(), m1),
                Param::new(program.get(), m2),
                Param::new(program.get(), m3),
            ]),
            (8, [m1, m2, m3]) => OC08([
                Param::new(program.get(), m1),
                Param::new(program.get(), m2),
                Param::new(program.get(), m3),
            ]),
            (99, _) => OC99,
            _ => unreachable!("Wrong instruction {}!", instruction),
        }
    }

    fn parse(code: isize) -> (isize, [ParamMode; 3]) {
        (
            code % 100,
            [
                ParamMode::new((code / 100) % 10),
                ParamMode::new((code / 1000) % 10),
                ParamMode::new((code / 10000) % 10),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puts_input_to_output() {
        let mut program = OpcodeComputer::new(vec![3, 0, 4, 0, 99], Some(vec![7]));
        program.run();
        assert_eq!(program.output, vec![7]);
    }

    #[test]
    fn multiplies_and_puts_to_the_latest() {
        let mut program = OpcodeComputer::new(vec![2, 4, 4, 5, 99, 0], None);
        program.run();
        assert_eq!(program.memory, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn sums_and_puts_to_the_first() {
        let mut program = OpcodeComputer::new(vec![1, 0, 0, 0, 99], None);
        program.run();
        assert_eq!(program.memory, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn overrides_99_in_the_middle() {
        let mut program = OpcodeComputer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], None);
        program.run();
        assert_eq!(program.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn parse_code_01() {
        use ParamMode::*;
        let parsed = Instruction::parse(102);
        assert_eq!(parsed, (2, [Immidiate, Positional, Positional]));
    }

    #[test]
    fn parse_code_too_long() {
        use ParamMode::*;
        let parsed = Instruction::parse(10001103);
        assert_eq!(parsed, (3, [Immidiate, Immidiate, Positional]));
    }

    #[test]
    fn sum_opcode_with_modes() {
        let mut program = OpcodeComputer::new(vec![1001, 5, 3, 0, 99, 8], None);
        program.run();
        assert_eq!(program.memory, vec![11, 5, 3, 0, 99, 8]);
    }

    #[test]
    fn sum_negativ_with_modes() {
        let mut program = OpcodeComputer::new(vec![1101, 100, -1, 4, 0], None);
        program.run();
        assert_eq!(program.memory, vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn position_equal_to() {
        let mut program =
            OpcodeComputer::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], Some(vec![8]));
        program.run();
        assert_eq!(*program.output.last().unwrap(), 1);
    }

    #[test]
    fn position_not_equal_to() {
        let mut program =
            OpcodeComputer::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], Some(vec![7]));
        program.run();
        assert_eq!(*program.output.last().unwrap(), 0);
    }

    #[test]
    fn immediate_less_than() {
        let mut program = OpcodeComputer::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], Some(vec![7]));
        program.run();
        assert_eq!(*program.output.last().unwrap(), 1);
    }

    #[test]
    fn immediate_not_less_than() {
        let mut program = OpcodeComputer::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], Some(vec![10]));
        program.run();
        assert_eq!(*program.output.last().unwrap(), 0);
    }
}
