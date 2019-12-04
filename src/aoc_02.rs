use std::fs;

fn read_and_parse() -> Vec<usize> {
    let content = fs::read_to_string("input-02.txt").unwrap();
    content
        .trim()
        .split(",")
        .map(|string| usize::from_str_radix(string.as_ref(), 10).unwrap())
        .collect()
}

pub fn aoc_02_01() -> usize {
    let mut state = read_and_parse();
    let mut opcode_counter = 0;
    let mut halted = false;

    state[1] = 12;
    state[2] = 2;

    while !halted {
        tick(&mut state, &mut opcode_counter, &mut halted);
    }

    state[0]
}

fn tick(state: &mut Vec<usize>, opcode_counter: &mut usize, halted: &mut bool) {
    let opcode_position = *opcode_counter * 4;
    let opcode = state[opcode_position];

    match opcode {
        1 => {
            let arg1 = state[state[opcode_position + 1]];
            let arg2 = state[state[opcode_position + 2]];
            let put_to = state[opcode_position + 3].clone();
            state[put_to] = arg1 + arg2;
        }
        2 => {
            let arg1 = state[state[opcode_position + 1]];
            let arg2 = state[state[opcode_position + 2]];
            let put_to = state[opcode_position + 3].clone();
            state[put_to] = arg1 * arg2;
        }
        99 => *halted = true,
        _ => unreachable!("Wrong OpCode {}!", opcode),
    }

    *opcode_counter += 1;
}
