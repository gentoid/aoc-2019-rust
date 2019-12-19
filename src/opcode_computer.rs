use std::{collections::HashMap, thread::sleep, time::Duration};

#[derive(Debug)]
pub struct OpcodeComputer {
    instructions: Vec<isize>,
    instruction_pointer: usize,
    extended_memory: HashMap<usize, isize>,
    pub state: ComputerState,
    input: Vec<isize>,
    pub output: Vec<isize>,
    relative_base: isize,
    debug: bool,
    name: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum ComputerState {
    Initialized,
    Running,
    WaitingForInput(Param),
    Halted,
}

impl OpcodeComputer {
    pub fn new(instructions: &Vec<isize>) -> Self {
        Self {
            instructions: instructions.clone(),
            instruction_pointer: 0,
            extended_memory: HashMap::new(),
            state: ComputerState::Initialized,
            input: vec![],
            output: vec![],
            relative_base: 0,
            debug: false,
            name: None,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.into());
    }

    pub fn add_input(&mut self, input: &isize) -> &mut Self {
        self.input.push(input.clone());
        self
    }

    pub fn get_output(&mut self) -> Option<isize> {
        match self.output.is_empty() {
            true => None,
            false => Some(self.output.remove(0)),
        }
    }

    pub fn halted(&self) -> bool {
        self.state == ComputerState::Halted
    }

    pub fn run(&mut self) -> isize {
        use ComputerState::*;

        if let WaitingForInput(param) = self.state {
            self.state = Running;
            self.take_input(&param);
        }

        while self.perform_more() {
            self.tick();
        }

        self.instructions[0]
    }

    pub fn debug(&mut self) {
        self.debug = true;
    }

    fn perform_more(&self) -> bool {
        use ComputerState::*;

        match self.state {
            WaitingForInput(_) | Halted => false,
            _ => true,
        }
    }

    fn get(&mut self) -> isize {
        let result = self.get_value(self.instruction_pointer);
        self.instruction_pointer += 1;
        result
    }

    fn tick(&mut self) {
        use Instruction::*;

        let instruction = Instruction::next(self);
        if self.debug {
            println!("!!== {:?} == !!", self.name);
            println!("Got instruction: {:?}", instruction);
        }

        match instruction {
            Sum(params) => self.opcode_with_3_args(&params, |a, b| a + b),
            Mul(params) => self.opcode_with_3_args(&params, |a, b| a * b),
            Input(param) => self.take_input(&param),
            Output(param) => self.put_output(self.value_for_param(&param)),
            JmpIfTrue(params) => {
                if self.value_for_param(&params[0]) != 0 {
                    self.set_pointer(self.value_for_param(&params[1]) as usize);
                }
            }
            JmpIfFalse(params) => {
                if self.value_for_param(&params[0]) == 0 {
                    self.set_pointer(self.value_for_param(&params[1]) as usize);
                }
            }
            LessThan(params) => {
                if self.value_for_param(&params[0]) < self.value_for_param(&params[1]) {
                    self.set_value_from_param(&params[2], 1)
                } else {
                    self.set_value_from_param(&params[2], 0)
                }
            }
            Equal(params) => {
                if self.value_for_param(&params[0]) == self.value_for_param(&params[1]) {
                    self.set_value_from_param(&params[2], 1)
                } else {
                    self.set_value_from_param(&params[2], 0)
                }
            }
            SetRelBase(param) => self.relative_base += self.value_for_param(&param),
            Halt => self.state = ComputerState::Halted,
        }
        if self.debug {
            println!("instructions: {:?}", self.instructions);
            println!("instruction_pointer: {:?}", self.instruction_pointer);
            println!("extended_memory: {:?}", self.extended_memory);
            println!("state: {:?}", self.state);
            println!("input: {:?}", self.input);
            println!("output: {:?}", self.output);
            println!("relative_base: {:?}", self.relative_base);
            println!("=====================================================================");
            println!("");
            sleep(Duration::from_millis(2000 as u64));
        }
    }

    fn set_pointer(&mut self, address: usize) {
        if self.debug {
            println!("Setting pointer to {}", address);
        }
        self.instruction_pointer = address;
    }

    fn value_for_param(&self, param: &Param) -> isize {
        match param.mode {
            ParamMode::Positional => self.get_value(param.value as usize),
            ParamMode::Immidiate => param.value,
            ParamMode::Relative => self.get_value((param.value + self.relative_base) as usize),
        }
    }

    fn get_value(&self, address: usize) -> isize {
        let result = if self.extended_memory_address(address) {
            self.extended_memory.get(&address).unwrap_or(&0).clone()
        } else {
            self.instructions[address]
        };
        if self.debug {
            println!("Got value {} at {}", result, address);
        }
        result
    }

    fn set_value(&mut self, address: usize, value: isize) {
        if self.debug {
            println!("Going to set {} to {}", address, value);
        }
        if self.extended_memory_address(address) {
            self.extended_memory.insert(address, value);
        } else {
            self.instructions[address] = value;
        }
    }

    fn extended_memory_address(&self, address: usize) -> bool {
        address >= self.instructions.len()
    }

    fn opcode_with_3_args(&mut self, params: &[Param; 3], f: fn(isize, isize) -> isize) {
        let [p1, p2, p3] = params;
        let val1 = self.value_for_param(&p1);
        let val2 = self.value_for_param(&p2);
        self.set_value_from_param(&p3, f(val1, val2));
    }

    fn set_value_from_param(&mut self, param: &Param, value: isize) {
        match param.mode {
            ParamMode::Positional => self.set_value(param.value as usize, value),
            ParamMode::Immidiate => panic!("It's impossible to use immidiate mode to set value"),
            ParamMode::Relative => {
                self.set_value((param.value + self.relative_base) as usize, value);
            }
        }
    }

    fn take_input(&mut self, param: &Param) {
        if self.input.is_empty() {
            self.state = ComputerState::WaitingForInput(*param);
            return;
        }
        let input = self.input.remove(0);
        self.set_value_from_param(&param, input);
    }

    fn put_output(&mut self, value: isize) {
        self.output.push(value);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParamMode {
    Positional,
    Immidiate,
    Relative,
}

impl ParamMode {
    fn new(value: isize) -> Self {
        match value {
            0 => ParamMode::Positional,
            1 => ParamMode::Immidiate,
            2 => ParamMode::Relative,
            _ => unreachable!("Incorrect value to init ParamMode: {}", value),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Param {
    value: isize,
    mode: ParamMode,
}

impl Param {
    pub fn new(value: isize, mode: ParamMode) -> Self {
        Self { value, mode }
    }
}

#[derive(Debug)]
enum Instruction {
    Sum([Param; 3]),
    Mul([Param; 3]),
    Input(Param),
    Output(Param),
    JmpIfTrue([Param; 2]),
    JmpIfFalse([Param; 2]),
    LessThan([Param; 3]),
    Equal([Param; 3]),
    SetRelBase(Param),
    Halt,
}

impl Instruction {
    pub fn next(program: &mut OpcodeComputer) -> Self {
        use Instruction::*;
        let instruction = program.get();

        match Instruction::parse(instruction) {
            (1, [m1, m2, m3]) => Sum([
                Param::new(program.get(), m1),
                Param::new(program.get(), m2),
                Param::new(program.get(), m3),
            ]),
            (2, [m1, m2, m3]) => Mul([
                Param::new(program.get(), m1),
                Param::new(program.get(), m2),
                Param::new(program.get(), m3),
            ]),
            (3, [m, _, _]) => Input(Param::new(program.get(), m)),
            (4, [m, _, _]) => Output(Param::new(program.get(), m)),
            (5, [m1, m2, _]) => {
                JmpIfTrue([Param::new(program.get(), m1), Param::new(program.get(), m2)])
            }
            (6, [m1, m2, _]) => {
                JmpIfFalse([Param::new(program.get(), m1), Param::new(program.get(), m2)])
            }
            (7, [m1, m2, m3]) => LessThan([
                Param::new(program.get(), m1),
                Param::new(program.get(), m2),
                Param::new(program.get(), m3),
            ]),
            (8, [m1, m2, m3]) => Equal([
                Param::new(program.get(), m1),
                Param::new(program.get(), m2),
                Param::new(program.get(), m3),
            ]),
            (9, [m1, _, _]) => SetRelBase(Param::new(program.get(), m1)),
            (99, _) => Halt,
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
        let mut program = OpcodeComputer::new(&vec![3, 0, 4, 0, 99]);
        program.add_input(&7).run();
        assert_eq!(program.get_output(), Some(7));
    }

    #[test]
    fn multiplies_and_puts_to_the_latest() {
        let mut program = OpcodeComputer::new(&vec![2, 4, 4, 5, 99, 0]);
        program.run();
        assert_eq!(program.instructions, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn sums_and_puts_to_the_first() {
        let mut program = OpcodeComputer::new(&vec![1, 0, 0, 0, 99]);
        program.run();
        assert_eq!(program.instructions, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn overrides_99_in_the_middle() {
        let mut program = OpcodeComputer::new(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        program.run();
        assert_eq!(program.instructions, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
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
        let mut program = OpcodeComputer::new(&vec![1001, 5, 3, 0, 99, 8]);
        program.run();
        assert_eq!(program.instructions, vec![11, 5, 3, 0, 99, 8]);
    }

    #[test]
    fn sum_negativ_with_modes() {
        let mut program = OpcodeComputer::new(&vec![1101, 100, -1, 4, 0]);
        program.run();
        assert_eq!(program.instructions, vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn position_equal_to() {
        let mut program = OpcodeComputer::new(&vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        program.add_input(&8).run();
        assert_eq!(program.get_output(), Some(1));
    }

    #[test]
    fn position_not_equal_to() {
        let mut program = OpcodeComputer::new(&vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        program.add_input(&7).run();
        assert_eq!(program.get_output(), Some(0));
    }

    #[test]
    fn immediate_less_than() {
        let mut program = OpcodeComputer::new(&vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        program.add_input(&7).run();
        assert_eq!(program.get_output(), Some(1));
    }

    #[test]
    fn immediate_not_less_than() {
        let mut program = OpcodeComputer::new(&vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        program.add_input(&10).run();
        assert_eq!(program.get_output(), Some(0));
    }

    #[test]
    fn produce_copy_of_the_input() {
        let input = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut computer = OpcodeComputer::new(&input);
        computer.run();

        assert_eq!(computer.output, input);
    }

    #[test]
    fn outputs_16_digit_number() {
        let input = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut computer = OpcodeComputer::new(&input);
        computer.run();

        assert_eq!(computer.get_output(), Some(1219070632396864));
    }

    #[test]
    fn outputs_number_from_the_program() {
        let input = vec![104, 1125899906842624, 99];
        let mut computer = OpcodeComputer::new(&input);
        computer.run();

        assert_eq!(computer.get_output(), Some(1125899906842624));
    }
}
