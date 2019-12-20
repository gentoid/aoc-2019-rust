use {
    num_integer::gcd,
    std::{
        collections::{HashMap, HashSet},
        ops::{Div, Mul},
    },
};

pub fn aoc_10_01() -> Coord {
    let input = vec![".#..#", ".....", "#####", "....#", "...##"];
    find_best_asteroid(&input)
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

    pub fn square_distance(&self, other: &Self) -> isize {
        let delta = self.delta(&other);
        delta.x.pow(2) + delta.y.pow(2)
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

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Delta {
    x: isize,
    y: isize,
}

impl Delta {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Mul<isize> for Delta {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<isize> for Delta {
    type Output = Self;

    fn div(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

struct DeltaIter {
    delta: Delta,
    current: Delta,
    factor: isize,
}

impl DeltaIter {
    pub fn new(delta: &Delta) -> Self {
        let (factor, delta) = if delta.x == 0 {
            (delta.y, Delta::new(0, 1))
        } else if delta.y == 0 {
            (delta.x, Delta::new(1, 0))
        } else {
            let factor = gcd(delta.x, delta.y);
            (factor, *delta / factor)
        };

        Self {
            factor,
            delta,
            current: delta.clone(),
        }
    }
}

impl Iterator for DeltaIter {
    type Item = Delta;

    fn next(&mut self) -> Option<Self::Item> {
        self.current = self.delta * self.factor;
        if self.factor > 0 {
            self.factor += 1;
        } else {
            self.factor -= 1;
        }

        Some(self.current)
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
    let mut covered: HashSet<Coord> = HashSet::new();

    for test_coord in map.asteroids.keys() {
        if coord == test_coord {
            continue;
        }

        covered.extend(&find_covered(&map, &coord, &test_coord));
    }

    map.asteroids.len() - covered.len() - 1
}

fn find_covered(map: &Map, coord: &Coord, test_coord: &Coord) -> HashSet<Coord> {
    let mut covered = HashSet::new();

    for delta in DeltaIter::new(&coord.delta(&test_coord)) {
        let check = coord.with_delta(&delta);

        if check == *coord {
            break;
        }

        if check == *test_coord {
            continue;
        }

        if !map.is_inside(&check) {
            break;
        }

        if coord.square_distance(&test_coord) > coord.square_distance(&check) {
            covered.insert(*test_coord);
            continue;
        }

        map.asteroids.get(&check).map(|_| {
            covered.insert(check)
        });
    }

    covered
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_best_asteroid_on_tiny_map() {
        let input = vec![".#..#", ".....", "#####", "....#", "...##"];
        assert_eq!(Coord::new(3, 4), find_best_asteroid(&input));
    }

    #[test]
    fn correctly_generates_diagonal_deltas() {
        let base_delta = Delta ::new(6, 12);
        let deltas: Vec<Delta> = DeltaIter::new(&base_delta).into_iter().take(3).collect();
        let expect_deltas = vec![Delta::new(6, 12), Delta::new(7, 14), Delta::new(8, 16)];
        assert_eq!(expect_deltas, deltas);
    }

    #[test]
    fn correctly_generates_negative_diagonal_deltas() {
        let base_delta = Delta ::new(-4, 2);
        let deltas: Vec<Delta> = DeltaIter::new(&base_delta).into_iter().take(3).collect();
        let expect_deltas = vec![Delta::new(-4, 2), Delta::new(-6, 3), Delta::new(-8, 4)];
        assert_eq!(expect_deltas, deltas);
    }

    #[test]
    fn correctly_generates_negative_horizontal() {
        let base_delta = Delta ::new(-79, 0);
        let deltas: Vec<Delta> = DeltaIter::new(&base_delta).into_iter().take(3).collect();
        let expect_deltas = vec![Delta::new(-79, 0), Delta::new(-80, 0), Delta::new(-81, 0)];
        assert_eq!(expect_deltas, deltas);
    }

    #[test]
    fn correctly_generates_positive_horizontal() {
        let base_delta = Delta ::new(5, 0);
        let deltas: Vec<Delta> = DeltaIter::new(&base_delta).into_iter().take(3).collect();
        let expect_deltas = vec![Delta::new(5, 0), Delta::new(6, 0), Delta::new(7, 0)];
        assert_eq!(expect_deltas, deltas);
    }

    #[test]
    fn correctly_generates_negative_vertical() {
        let base_delta = Delta ::new(0, -79);
        let deltas: Vec<Delta> = DeltaIter::new(&base_delta).into_iter().take(3).collect();
        let expect_deltas = vec![Delta::new(0, -79), Delta::new(0, -80), Delta::new(0, -81)];
        assert_eq!(expect_deltas, deltas);
    }

    #[test]
    fn correctly_generates_positive_vertical() {
        let base_delta = Delta ::new(0, 5);
        let deltas: Vec<Delta> = DeltaIter::new(&base_delta).into_iter().take(3).collect();
        let expect_deltas = vec![Delta::new(0, 5), Delta::new(0, 6), Delta::new(0, 7)];
        assert_eq!(expect_deltas, deltas);
    }
}
