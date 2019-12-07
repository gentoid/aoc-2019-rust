use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Diff {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

enum Direction {
    Vertical,
    Horizontal,
}

struct Segment {
    p1: Point,
    p2: Point,
    direction: Direction,
}

fn read_and_parse() -> Vec<Vec<Diff>> {
    let file = File::open("input-03.txt").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| parse_line(line.unwrap()))
        .collect()
}

fn parse_line(line: String) -> Vec<Diff> {
    line.trim().split(",").map(parse_movement).collect()
}

fn parse_movement(movement: &str) -> Diff {
    let distance = i32::from_str_radix(&movement[1..], 10).unwrap();
    match movement.chars().next().unwrap() {
        'U' => Diff { x: 0, y: distance },
        'R' => Diff { x: distance, y: 0 },
        'D' => Diff { x: 0, y: -distance },
        'L' => Diff { x: -distance, y: 0 },
        _ => unreachable!(),
    }
}

fn calcNextPoint(from: &Point, diff: &Diff) -> Point {
    Point {
        x: from.x + diff.x,
        y: from.y + diff.y,
    }
}

fn newSegment(from: Point, diff: &Diff) -> Segment {
    Segment {
        p1: from,
        p2: calcNextPoint(&from, diff),
        direction: if diff.x == 0 {
            Direction::Vertical
        } else {
            Direction::Horizontal
        },
    }
}

fn toSegments(diffs: &Vec<Diff>) -> Vec<Segment> {
    let point = Point { x: 0, y: 0 };
    let segments = vec![];

    for diff in diffs.iter() {
        let segment = newSegment(point.clone(), diff);
        segments.push(segment);
        point = segment.p2;
    }

    segments
}
