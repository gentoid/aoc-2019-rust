use {crate::opcode_computer::Program, std::fs};

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
    let mut program = Program::new(memory, Some(vec![1]));
    program.run();

    *program.output.last().unwrap()
}

pub fn aoc_05_02() -> isize {
    let memory = read_and_parse();
    let mut program = Program::new(memory, Some(vec![5]));
    program.run();

    *program.output.last().unwrap()
}
