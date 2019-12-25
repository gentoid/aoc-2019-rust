use {
    crate::opcode_computer::OpcodeComputer,
    pancurses::{cbreak, curs_set, endwin, initscr, noecho, resize_term, start_color, Input},
    std::{fs, thread::sleep, time::Duration},
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

    let game = parse_output(&computer.get_all_output());
    game.pixels
        .iter()
        .filter(|v| v.tile_type == TileType::Block)
        .count()
}

pub fn aoc_13_02() -> isize {
    let program = read_and_parse();
    let mut computer = OpcodeComputer::new(&program);

    let window = initscr();
    resize_term(40, 50);
    start_color();
    window.refresh();
    window.keypad(true);
    cbreak();
    noecho();
    curs_set(0);

    let mut score = 0;
    loop {
        computer.run();

        let game = parse_output(&computer.get_all_output());
        let lines = game.prepare_output();

        for line in lines {
            window.printw(format!("{}\n", line));
        }

        if computer.halted() {
            score = game.score;
            break;
        }

        sleep(Duration::from_millis(1000));

        match window.getch() {
            Some(Input::KeyLeft) => {
                computer.add_input(&-1);
            }
            Some(Input::KeyRight) => {
                computer.add_input(&1);
            }
            Some(Input::KeyExit) => {
                break;
            }
            _ => {
                computer.add_input(&0);
            }
        }
    }
    window.getch();
    endwin();

    score
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

impl Game {
    fn prepare_output(&self) -> Vec<String> {
        let mut lines: Vec<String> = vec![];
        let mut line_index = 0;
        let mut line = String::new();

        lines.push(format!("Score: {}", self.score));

        for pixel in self.pixels.iter() {
            let coord = pixel.coord;

            if coord.1 != line_index {
                line_index = coord.1;
                lines.push(line.into());
                line = String::new();
            }

            line.push(pixel.tile_type.to_char());
        }

        lines.push(line.into());

        lines
    }
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

    fn to_char(&self) -> char {
        use TileType::*;

        match self {
            Empty => ' ',
            Wall => '@',
            Block => '#',
            Paddle => '=',
            Ball => 'o',
        }
    }
}
