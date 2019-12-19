use {crate::opcode_computer::OpcodeComputer, std::fs};

fn read_and_parse() -> Vec<isize> {
    let content = fs::read_to_string("inputs/input-09.txt").unwrap();
    content
        .trim()
        .split(",")
        .map(|string| isize::from_str_radix(string.as_ref(), 10).unwrap())
        .collect()
}

pub fn aoc_09_01() -> isize {
    let mut computer = OpcodeComputer::new(&read_and_parse());
    computer.add_input(&1);
    computer.run();
    computer.get_output().unwrap()
}

pub fn aoc_09_02() -> isize {
    let mut computer = OpcodeComputer::new(&read_and_parse());
    computer.add_input(&2);
    computer.run();
    computer.get_output().unwrap()
}
