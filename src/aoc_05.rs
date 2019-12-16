use {crate::opcode_computer::OpcodeComputer, std::fs};

fn read_and_parse() -> Vec<isize> {
    let content = fs::read_to_string("input-05.txt").unwrap();
    content
        .trim()
        .split(",")
        .map(|string| isize::from_str_radix(string.as_ref(), 10).unwrap())
        .collect()
}

pub fn aoc_05_01() -> isize {
    let memory = read_and_parse();
    let mut program = OpcodeComputer::new(memory);
    program.add_input(&1).run();

    *program.output.last().unwrap()
}

pub fn aoc_05_02() -> isize {
    let memory = read_and_parse();
    let mut program = OpcodeComputer::new(memory);
    program.add_input(&5).run();

    *program.output.last().unwrap()
}
