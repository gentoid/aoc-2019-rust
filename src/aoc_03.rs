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
    line.split(",").map(parse_movement).collect()
}

fn parse_movement(movement: &str) -> Diff {
    let movement = movement.trim();
    let distance = i32::from_str_radix(&movement[1..], 10).unwrap();
    match movement.chars().next().unwrap() {
        'U' => Diff { x: 0, y: distance },
        'R' => Diff { x: distance, y: 0 },
        'D' => Diff { x: 0, y: -distance },
        'L' => Diff { x: -distance, y: 0 },
        _ => unreachable!(),
    }
}

fn calc_next_point(from: &Point, diff: &Diff) -> Point {
    Point {
        x: from.x + diff.x,
        y: from.y + diff.y,
    }
}

fn new_segment(from: Point, diff: &Diff) -> Segment {
    Segment {
        p1: from.clone(),
        p2: calc_next_point(&from, diff),
        direction: if diff.x == 0 {
            Direction::Vertical
        } else {
            Direction::Horizontal
        },
    }
}

fn to_segments(diffs: &Vec<Diff>) -> Vec<Segment> {
    let mut point = Point { x: 0, y: 0 };
    let mut segments = vec![];

    for diff in diffs.iter() {
        let segment = new_segment(point, diff);
        point = segment.p2.clone();
        segments.push(segment);
    }

    segments
}

fn is_horizontal(segment: &Segment) -> bool {
    use Direction::*;

    match &segment.direction {
        Horizontal => true,
        Vertical => false,
    }
}

fn in_range(what: &i32, start: &i32, end: &i32) -> bool {
    if start < end {
        start < what && what < end
    } else {
        end < what && what < start
    }
}

fn find_intersection_one_way(segm1: &Segment, segm2: &Segment) -> Option<Point> {
    if is_horizontal(segm1)
        && !is_horizontal(segm2)
        && in_range(&segm1.p1.y, &segm2.p1.y, &segm2.p2.y)
        && in_range(&segm2.p1.x, &segm1.p1.x, &segm1.p2.x)
    {
        Some(Point {
            x: segm2.p1.x,
            y: segm1.p1.y,
        })
    } else {
        None
    }
}

fn find_intersection(segm1: &Segment, segm2: &Segment) -> Option<Point> {
    find_intersection_one_way(segm1, segm2).or(find_intersection_one_way(segm1, segm2))
}

fn manhattan_distance(point: &Point) -> i32 {
    point.x.abs() + point.y.abs()
}

pub fn aoc_03_01() -> i32 {
    let data = read_and_parse();
    let mut intersections: Vec<Point> = vec![];

    for segm1 in to_segments(&data[0]) {
        for segm2 in to_segments(&data[1]) {
            find_intersection(&segm1, &segm2).map(|point| intersections.push(point));
        }
    }

    intersections.iter().map(manhattan_distance).min().unwrap()
}
