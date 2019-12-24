use {
    crate::opcode_computer::OpcodeComputer,
    std::{collections::HashMap, fs},
};

fn read_and_parse() -> Vec<isize> {
    let content = fs::read_to_string("inputs/input-13.txt").unwrap();
    content
        .trim()
        .split(",")
        .map(|string| isize::from_str_radix(string.as_ref(), 10).unwrap())
        .collect()
}

pub fn aoc_13_01() -> usize {
    let program = read_and_parse();
    let mut computer = OpcodeComputer::new(&program);
    computer.run();

    let screen: HashMap<Coord, TileType> = parse_output(&computer.output);
    screen.values().filter(|v| **v == TileType::Block).count()
}

fn parse_output(seq: &Vec<isize>) -> HashMap<Coord, TileType> {
    let mut res = HashMap::new();

    let mut index = 0;
    while index + 2 < seq.len() {
        let x = seq[index];
        let y = seq[index + 1];
        let tile_id = seq[index + 2];
        res.insert((x, y), TileType::from_type_id(tile_id));

        index += 3;
    }
    res
}

type Coord = (isize, isize);

#[derive(PartialEq)]
enum TileType {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl TileType {
    fn from_type_id(type_id: isize) -> Self {
        use TileType::*;

        match type_id {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => Paddle,
            4 => Ball,
            _ => unreachable!(),
        }
    }
}
