use {crate::opcode_computer::Program, std::fs};

fn read_and_parse() -> Vec<usize> {
    let content = fs::read_to_string("input-02.txt").unwrap();
    content
        .trim()
        .split(",")
        .map(|string| usize::from_str_radix(string.as_ref(), 10).unwrap())
        .collect()
}

pub fn aoc_02_01() -> usize {
    Program::new(read_and_parse()).run(12, 2)
}

pub fn aoc_02_02() -> usize {
    let looking_for = 19690720;
    let initial_memory = read_and_parse();

    for noun in 0..100 {
        for verb in 0..100 {
            if Program::new(initial_memory.clone()).run(noun, verb) == looking_for {
                return noun * 100 + verb;
            }
        }
    }

    return 0;
}
