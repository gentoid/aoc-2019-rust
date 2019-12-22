use {
    crate::opcode_computer::OpcodeComputer,
    std::{collections::HashMap, fs},
};

fn read_and_parse() -> Vec<isize> {
    let content = fs::read_to_string("inputs/input-11.txt").unwrap();
    content
        .trim()
        .split(",")
        .map(|string| isize::from_str_radix(string.as_ref(), 10).unwrap())
        .collect()
}

pub fn aoc_11_01() -> usize {
    let program = read_and_parse();
    let mut robot = PaintingRobot::new(&program, &(0, 0), Color::Black);
    while !robot.done() {
        robot.next();
    }
    robot.map.len()
}

pub fn aoc_11_02() -> Vec<String> {
    let program = read_and_parse();
    let mut robot = PaintingRobot::new(&program, &(0, 0), Color::White);
    while !robot.done() {
        robot.next();
    }

    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;

    for ((x, y), _) in robot.map.clone() {
        x_min = isize::min(x, x_min);
        x_max = isize::max(x, x_max);
        y_min = isize::min(y, y_min);
        y_max = isize::max(y, y_max);
    }

    let mut lines = vec![];

    for y in y_min..=y_max {
        let mut line = "".to_owned();
        for x in x_min..=x_max {
            line.push(robot.map.get(&(x, y)).unwrap_or(&Color::Black).to_char());
        }
        lines.push(line);
    }

    lines
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Self {
        use Direction::*;

        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn turl_left(&self) -> Self {
        use Direction::*;

        match self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }

    fn turn(&self, output: &isize) -> Self {
        if *output == 0 {
            self.turl_left()
        } else {
            self.turn_right()
        }
    }

    fn next_coordinate(&self, (x, y): &(isize, isize)) -> (isize, isize) {
        use Direction::*;

        match self {
            Up => (*x, y - 1),
            Right => (x + 1, *y),
            Down => (*x, y + 1),
            Left => (x - 1, *y),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Black,
    White,
}

impl Color {
    fn to_int(&self) -> isize {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }

    fn from_int(int: &isize) -> Self {
        match int {
            0 => Color::Black,
            1 => Color::White,
            _ => unreachable!(),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Color::Black => ' ',
            Color::White => '#',
        }
        .into()
    }
}

struct PaintingRobot {
    direction: Direction,
    computer: OpcodeComputer,
    map: HashMap<(isize, isize), Color>,
    coordinate: (isize, isize),
}

impl PaintingRobot {
    fn new(program: &Vec<isize>, coordinate: &(isize, isize), color: Color) -> Self {
        let mut map = HashMap::new();
        map.insert(coordinate.clone(), color);
        let computer = OpcodeComputer::new(&program);

        Self {
            direction: Direction::Up,
            computer,
            map,
            coordinate: coordinate.clone(),
        }
    }

    fn next(&mut self) {
        let input_color = self.map.get(&self.coordinate).unwrap_or(&Color::Black);
        self.computer.add_input(&input_color.to_int());
        self.computer.run();

        let color = Color::from_int(&self.computer.get_output().unwrap());
        let direction = self.direction.turn(&self.computer.get_output().unwrap());
        let coordinate = direction.next_coordinate(&self.coordinate);

        self.direction = direction;
        self.map.insert(self.coordinate, color);
        self.coordinate = coordinate;
    }

    fn done(&self) -> bool {
        self.computer.halted()
    }
}
