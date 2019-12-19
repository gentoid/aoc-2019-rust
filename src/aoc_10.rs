use std::{collections::HashMap, ops::Mul};

pub fn aoc_10_01() -> Coord {
    find_best_asteroid(&vec![""])
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn delta(&self, other: &Self) -> Delta {
        Delta {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }

    pub fn with_delta(&self, delta: &Delta) -> Self {
        Self {
            x: self.x + delta.x,
            y: self.y + delta.y,
        }
    }
}

struct Map {
    width: usize,
    height: usize,
    asteroids: HashMap<Coord, ()>,
}

impl Map {
    pub fn is_inside(&self, coord: &Coord) -> bool {
        coord.x >= 0
            && coord.x < self.width as isize
            && coord.y >= 0
            && coord.y < self.height as isize
    }
}

#[derive(Clone, Copy)]
pub struct Delta {
    x: isize,
    y: isize,
}

impl Mul<usize> for Delta {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            x: self.x * rhs as isize,
            y: self.x * rhs as isize,
        }
    }
}

fn find_best_asteroid(input: &Vec<&str>) -> Coord {
    let map = parse_map(&input);

    let mut max_seen = 0;
    let mut best_asteroid = Coord::new(0, 0);

    for asteroid in map.asteroids.keys() {
        let seen = how_much_asteroids_seen(&map, &asteroid);

        if seen > max_seen {
            max_seen = seen;
            best_asteroid = asteroid.clone();
        }
    }

    best_asteroid
}

fn parse_map(input: &Vec<&str>) -> Map {
    let mut asteroids = HashMap::new();

    for (y, line) in input.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                asteroids.insert(Coord::new(x as isize, y as isize), ());
            }
        }
    }

    Map {
        width: input[0].len(),
        height: input.len(),
        asteroids,
    }
}

fn how_much_asteroids_seen(map: &Map, coord: &Coord) -> usize {
    let mut covered = 0;

    for test_coord in map.asteroids.keys() {
        if coord == test_coord {
            continue;
        }

        covered += find_covered(&map, &coord, &test_coord);
    }

    map.asteroids.len() - covered - 1
}

fn find_covered(map: &Map, coord: &Coord, test_coord: &Coord) -> usize {
    let mut covered = 0;
    let delta = coord.delta(&test_coord);
    let mut factor = 1;
    loop {
        let check = coord.with_delta(&(delta.clone() * factor));
        factor += 1;
        if *coord == check {
            continue;
        }

        if !map.is_inside(&check) {
            break;
        }

        map.asteroids.get(&check).map(|_| covered += 1);
    }

    covered
}

#[cfg(tests)]
mod tests {
    #[test]
    fn finds_the_best_asteroid_on_tiny_map() {
        let input = vec![".#..#", ".....", "#####", "....#", "...##"];
        assert_eq!(find_best_asteroid(&input), (3, 4));
    }
}
