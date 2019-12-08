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
    init_with_noun_verb(12, 2).run()
}

pub fn aoc_02_02() -> usize {
    let looking_for = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {
            if init_with_noun_verb(noun, verb).run() == looking_for {
                return noun * 100 + verb;
            }
        }
    }

    return 0;
}

fn init_with_noun_verb(noun: usize, verb: usize) -> Program {
    let mut memory = read_and_parse();
    memory[1] = noun;
    memory[2] = verb;

    Program::new(memory, None)
}
