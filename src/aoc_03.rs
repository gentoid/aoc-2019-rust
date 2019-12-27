use crate::read_input::read_lines;

struct Diff {
    x: i32,
    y: i32,
}

impl Diff {
    pub fn from_vector(vector: &str) -> Self {
        let vector = vector.trim();
        let distance = i32::from_str_radix(&vector[1..], 10).unwrap();
        let (x, y) = match vector.chars().next().unwrap() {
            'U' => (0, distance),
            'R' => (distance, 0),
            'D' => (0, -distance),
            'L' => (-distance, 0),
            _ => unreachable!(),
        };

        Self { x, y }
    }

    pub fn direction(&self) -> Direction {
        if self.x == 0 {
            Direction::Vertical
        } else {
            Direction::Horizontal
        }
    }
}

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn start_point() -> Self {
        Self::new(0, 0)
    }

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn with_diff(&self, diff: &Diff) -> Point {
        Self::new(self.x + diff.x, self.y + diff.y)
    }

    pub fn manhattan_distance_to_start(&self) -> i32 {
        self.manhattan_distance(&Self::start_point())
    }

    pub fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
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

impl Segment {
    pub fn new(from: &Point, diff: &Diff) -> Self {
        Self {
            p1: from.clone(),
            p2: from.with_diff(diff),
            direction: diff.direction(),
        }
    }

    pub fn is_horizontal(&self) -> bool {
        use Direction::*;

        match self.direction {
            Horizontal => true,
            Vertical => false,
        }
    }

    pub fn find_intersection(&self, other: &Self) -> Option<Point> {
        self.find_intersection_one_way(other)
            .or(other.find_intersection_one_way(self))
    }

    pub fn find_intersection_one_way(&self, other: &Self) -> Option<Point> {
        if self.is_horizontal()
            && !other.is_horizontal()
            && self.in_range(&other, Direction::Vertical)
            && other.in_range(&self, Direction::Horizontal)
        {
            Some(Point::new(other.p1.x, self.p1.y))
        } else {
            None
        }
    }

    pub fn in_range(&self, other: &Self, direction: Direction) -> bool {
        use Direction::*;

        let (what, start, end) = match direction {
            Vertical => (&self.p1.y, &other.p1.y, &other.p2.y),
            Horizontal => (&self.p1.x, &other.p1.x, &other.p2.x),
        };

        if start < end {
            start < what && what < end
        } else {
            end < what && what < start
        }
    }

    pub fn manhattan_distance(&self, point: &Point) -> i32 {
        self.p1.manhattan_distance(point)
    }

    pub fn length(&self) -> i32 {
        self.p1.manhattan_distance(&self.p2)
    }
}

fn read_and_parse() -> Vec<Vec<Diff>> {
    read_lines(3).iter().map(parse_line).collect()
}

fn parse_line(line: &String) -> Vec<Diff> {
    line.split(",").map(Diff::from_vector).collect()
}

fn to_segments(diffs: &Vec<Diff>) -> Vec<Segment> {
    let mut point = Point::start_point();
    let mut segments = vec![];

    for diff in diffs.iter() {
        let segment = Segment::new(&point, diff);
        point = segment.p2.clone();
        segments.push(segment);
    }

    segments
}

pub fn aoc_03_01() -> i32 {
    let data = read_and_parse();
    let mut distances: Vec<i32> = vec![];

    for segm1 in to_segments(&data[0]) {
        for segm2 in to_segments(&data[1]) {
            segm1
                .find_intersection(&segm2)
                .map(|point| distances.push(point.manhattan_distance_to_start()));
        }
    }

    *distances.iter().min().unwrap()
}

pub fn aoc_03_02() -> i32 {
    let data = read_and_parse();
    let mut lengths: Vec<i32> = vec![];

    let mut segm1_length = 0;
    for segm1 in to_segments(&data[0]) {
        let mut segm2_length = 0;
        for segm2 in to_segments(&data[1]) {
            segm1.find_intersection(&segm2).map(|intersection| {
                let length = segm1_length
                    + segm2_length
                    + segm1.manhattan_distance(&intersection)
                    + segm2.manhattan_distance(&intersection);
                lengths.push(length);
            });
            segm2_length += segm2.length()
        }
        segm1_length += segm1.length()
    }

    *lengths.iter().min().unwrap()
}
