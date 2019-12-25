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

    let game = parse_output(&computer.output);
    game.pixels
        .iter()
        .filter(|v| v.tile_type == TileType::Block)
        .count()
}

fn parse_output(seq: &Vec<isize>) -> Game {
    let mut pixels = vec![];
    let mut score = 0;

    let mut index = 0;
    while index + 2 < seq.len() {
        let x = seq[index];
        let y = seq[index + 1];

        if x == -1 && y == 0 {
            score = seq[index + 2];
        } else {
            let tile_id = seq[index + 2];
            pixels.push(Pixel {
                coord: (x, y),
                tile_type: TileType::from_type_id(tile_id),
            });
        }

        index += 3;
    }
    Game { pixels, score }
}

struct Game {
    pixels: Vec<Pixel>,
    score: isize,
}

struct Pixel {
    coord: Coord,
    tile_type: TileType,
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
