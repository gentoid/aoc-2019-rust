use {
    crate::{opcode_computer::OpcodeComputer, read_input::read_intcode_program},
    pancurses::{
        cbreak, curs_set, endwin, initscr, noecho, resize_term, start_color, Input, Window,
    },
    std::cmp::max,
};

pub fn aoc_13_01() -> usize {
    let program = read_intcode_program(13);
    let mut computer = OpcodeComputer::new(&program);
    computer.run();

    let game = Game::from_output(&computer.get_all_output());
    game.pixels
        .iter()
        .filter(|v| v.tile_type == TileType::Block)
        .count()
}

pub fn aoc_13_02() -> isize {
    let mut program = read_intcode_program(13);
    program[0] = 2;
    let mut computer = OpcodeComputer::new(&program);

    let window = initscr();
    resize_term(35, 40);
    start_color();
    window.refresh();
    window.keypad(true);
    cbreak();
    noecho();
    curs_set(0);

    computer.run();
    let mut game = Game::from_output(&computer.get_all_output());

    for line in game.prepare_output() {
        window.printw(format!("{}\n", line));
    }

    loop {
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

        computer.run();

        game.update_state(&computer.get_all_output());
        game.draw_update(&window);

        if computer.halted() {
            break;
        }
    }

    window.getch();
    endwin();

    game.score
}

fn parse_output(seq: &Vec<isize>) -> (Vec<Pixel>, Option<isize>) {
    let mut pixels = vec![];
    let mut score = None;

    let mut index = 0;
    while index + 2 < seq.len() {
        let x = seq[index];
        let y = seq[index + 1];

        if x == -1 && y == 0 {
            score = Some(seq[index + 2]);
        } else {
            let tile_id = seq[index + 2];
            pixels.push(Pixel {
                coord: (x, y),
                tile_type: TileType::from_type_id(tile_id),
            });
        }

        index += 3;
    }
    (pixels, score)
}

fn coord_to_index(width: isize, (x, y): &(isize, isize)) -> usize {
    (x + y * width) as usize
}

struct Game {
    pixels: Vec<Pixel>,
    score: isize,
    width: isize,
    height: isize,
    changes_at: Vec<usize>,
}

impl Game {
    fn from_output(output: &Vec<isize>) -> Self {
        let (pixels, score) = parse_output(&output);

        let mut max_x = 0;
        let mut max_y = 0;

        for pixel in pixels.iter() {
            let (x, y) = pixel.coord;
            max_x = max(max_x, x);
            max_y = max(max_y, y);
        }

        let width = max_x + 1;

        Self {
            pixels,
            score: score.unwrap_or(0),
            width,
            height: max_y + 1,
            changes_at: vec![],
        }
    }

    fn prepare_output(&self) -> Vec<String> {
        let mut lines: Vec<String> = vec![];
        let mut line_index = 0;
        let mut line = String::new();

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
        lines.push(format!("Score: {}", self.score));

        lines
    }

    fn update_state(&mut self, output: &Vec<isize>) {
        let (pixels, score) = parse_output(&output);
        score.map(|score| self.score = score);

        for pixel in pixels.iter() {
            let index = coord_to_index(self.width, &pixel.coord);
            self.changes_at.push(index);
            self.pixels[index] = (*pixel).clone();
        }
    }

    fn draw_update(&mut self, window: &Window) {
        window.mvaddstr(self.height as i32, 7, format!("{}          ", self.score));

        for index in self.changes_at.iter() {
            let pixel = self.pixels[*index].clone();
            let (x, y) = pixel.coord;
            window.mvaddch(y as i32, x as i32, pixel.tile_type.to_char());
        }

        self.changes_at = vec![];
    }
}

#[derive(Clone)]
struct Pixel {
    coord: Coord,
    tile_type: TileType,
}

type Coord = (isize, isize);

#[derive(Clone, PartialEq)]
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
            Paddle => '\u{25A0}',
            Ball => '\u{25CB}',
        }
    }
}
