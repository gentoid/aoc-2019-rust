use {
    num_integer::gcd,
    std::{
        collections::{HashMap, HashSet},
        f64::consts::{FRAC_PI_2, PI},
        fs::File,
        io::{BufRead, BufReader},
        ops::{Div, Mul},
    },
};

fn read_lines() -> Vec<String> {
    let file = File::open("inputs/input-10.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.unwrap()).collect()
}

pub fn aoc_10_01() -> (Coord, usize) {
    let lines = read_lines();
    let input = lines.iter().map(String::as_ref).collect();
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

    pub fn angle_rad(&self, other: &Self) -> f64 {
        let delta_orig = Delta::new(0, 1);
        let delta_other = self.delta(&other);
        let radians = f64::atan2(delta_orig.y as f64, delta_orig.x as f64)
            - f64::atan2(delta_other.y as f64, delta_other.x as f64);

        if radians < 0.0 {
            radians + 2.0 * PI
        } else {
            radians
        }
    }
}

struct Map {
    width: usize,
    height: usize,
    asteroids: HashSet<Coord>,
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

fn find_best_asteroid(input: &Vec<&str>) -> (Coord, usize) {
    let map = parse_map(&input);

    let mut max_seen = HashSet::new();
    let mut best_asteroid = Coord::new(0, 0);

    for asteroid in map.asteroids.iter() {
        let seen = seen_asteroids(&map, &asteroid);

        if seen.len() > max_seen.len() {
            max_seen = seen;
            best_asteroid = asteroid.clone();
        }
    }

    (best_asteroid, max_seen.len())
}

fn parse_map(input: &Vec<&str>) -> Map {
    let mut asteroids = HashSet::new();

    for (y, line) in input.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                asteroids.insert(Coord::new(x as isize, y as isize));
            }
        }
    }

    Map {
        width: input[0].len(),
        height: input.len(),
        asteroids,
    }
}

fn seen_asteroids(map: &Map, coord: &Coord) -> HashSet<Coord> {
    let mut covered: HashSet<Coord> = HashSet::new();

    for test_coord in map.asteroids.iter() {
        if coord == test_coord {
            continue;
        }

        covered.extend(&find_covered(&map, &coord, &test_coord));
    }

    let mut asteroids = map.asteroids.clone();
    asteroids.remove(&coord);

    asteroids
        .difference(&covered)
        .into_iter()
        .map(|a| a.clone())
        .collect()
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

        map.asteroids.get(&check).map(|_| covered.insert(check));
    }

    covered
}

fn find_200th_vaporized_asteroid(input: &Vec<&str>, station: &Coord) -> Coord {
    let map = parse_map(&input);
    let nth_asteroid = 200;

    let mut vaporized= HashSet::new();
    let mut seen = HashSet::new();

    loop {
        seen = seen_asteroids(&map, &station);
        if vaporized.len() + seen.len() < 200 {
            for asteroid in seen {
                if !vaporized.contains(&asteroid) {
                    vaporized.insert(asteroid);
                }
            }
        } else {
            break;
        }
    }

    let sorted = sort_clockwise(&seen, &station);
    sorted[nth_asteroid - vaporized.len() - 1]
}

fn sort_clockwise(asteroids: &HashSet<Coord>, station: &Coord) -> Vec<Coord> {
    let mut with_angles = vec![];

    for a in asteroids {
        with_angles.push((*a, station.angle_rad(a)));
    }

    with_angles.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    with_angles.into_iter().map(|(a, _)| a).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_best_asteroid_on_tiny_map() {
        let input = vec![".#..#", ".....", "#####", "....#", "...##"];
        assert_eq!((Coord::new(3, 4), 8), find_best_asteroid(&input));
    }

    #[test]
    fn finds_the_best_asteroid_on_1st_mid_map() {
        let input = vec![
            "......#.#.",
            "#..#.#....",
            "..#######.",
            ".#.#.###..",
            ".#..#.....",
            "..#....#.#",
            "#..#....#.",
            ".##.#..###",
            "##...#..#.",
            ".#....####",
        ];
        assert_eq!((Coord::new(5, 8), 33), find_best_asteroid(&input));
    }

    #[test]
    fn finds_the_best_asteroid_on_2nd_mid_map() {
        let input = vec![
            "#.#...#.#.",
            ".###....#.",
            ".#....#...",
            "##.#.#.#.#",
            "....#.#.#.",
            ".##..###.#",
            "..#...##..",
            "..##....##",
            "......#...",
            ".####.###.",
        ];
        assert_eq!((Coord::new(1, 2), 35), find_best_asteroid(&input));
    }

    #[test]
    fn finds_the_best_asteroid_on_3rt_mid_map() {
        let input = vec![
            ".#..#..###",
            "####.###.#",
            "....###.#.",
            "..###.##.#",
            "##.##.#.#.",
            "....###..#",
            "..#.#..#.#",
            "#..#.#.###",
            ".##...##.#",
            ".....#.#..",
        ];
        assert_eq!((Coord::new(6, 3), 41), find_best_asteroid(&input));
    }

    #[test]
    fn finds_the_best_asteroid_on_big_map() {
        let input = vec![
            ".#..##.###...#######",
            "##.############..##.",
            ".#.######.########.#",
            ".###.#######.####.#.",
            "#####.##.#.##.###.##",
            "..#####..#.#########",
            "####################",
            "#.####....###.#.#.##",
            "##.#################",
            "#####.##.###..####..",
            "..######..##.#######",
            "####.##.####...##..#",
            ".#####..#.######.###",
            "##...#.##########...",
            "#.##########.#######",
            ".####.#.###.###.#.##",
            "....##.##.###..#####",
            ".#.#.###########.###",
            "#.#.#.#####.####.###",
            "###.##.####.##.#..##",
        ];
        assert_eq!((Coord::new(11, 13), 210), find_best_asteroid(&input));
    }

    #[test]
    fn correctly_generates_diagonal_deltas() {
        let base_delta = Delta::new(6, 12);
        let deltas: Vec<Delta> = DeltaIter::new(&base_delta).into_iter().take(3).collect();
        let expect_deltas = vec![Delta::new(6, 12), Delta::new(7, 14), Delta::new(8, 16)];
        assert_eq!(expect_deltas, deltas);
    }

    #[test]
    fn correctly_generates_negative_diagonal_deltas() {
        let base_delta = Delta::new(-4, 2);
        let deltas: Vec<Delta> = DeltaIter::new(&base_delta).into_iter().take(3).collect();
        let expect_deltas = vec![Delta::new(-4, 2), Delta::new(-6, 3), Delta::new(-8, 4)];
        assert_eq!(expect_deltas, deltas);
    }

    #[test]
    fn correctly_generates_negative_horizontal() {
        let base_delta = Delta::new(-79, 0);
        let deltas: Vec<Delta> = DeltaIter::new(&base_delta).into_iter().take(3).collect();
        let expect_deltas = vec![Delta::new(-79, 0), Delta::new(-80, 0), Delta::new(-81, 0)];
        assert_eq!(expect_deltas, deltas);
    }

    #[test]
    fn correctly_generates_positive_horizontal() {
        let base_delta = Delta::new(5, 0);
        let deltas: Vec<Delta> = DeltaIter::new(&base_delta).into_iter().take(3).collect();
        let expect_deltas = vec![Delta::new(5, 0), Delta::new(6, 0), Delta::new(7, 0)];
        assert_eq!(expect_deltas, deltas);
    }

    #[test]
    fn correctly_generates_negative_vertical() {
        let base_delta = Delta::new(0, -79);
        let deltas: Vec<Delta> = DeltaIter::new(&base_delta).into_iter().take(3).collect();
        let expect_deltas = vec![Delta::new(0, -79), Delta::new(0, -80), Delta::new(0, -81)];
        assert_eq!(expect_deltas, deltas);
    }

    #[test]
    fn correctly_generates_positive_vertical() {
        let base_delta = Delta::new(0, 5);
        let deltas: Vec<Delta> = DeltaIter::new(&base_delta).into_iter().take(3).collect();
        let expect_deltas = vec![Delta::new(0, 5), Delta::new(0, 6), Delta::new(0, 7)];
        assert_eq!(expect_deltas, deltas);
    }

    #[test]
    fn calculates_zero_angle() {
        let station = Coord::new(5, 8);
        let asteroid = Coord::new(5, 10);

        let diff = station.angle_rad(&asteroid);

        assert!(diff < 1e-10);
    }

    #[test]
    fn calculates_pi_2_angle() {
        let station = Coord::new(5, 8);
        let asteroid = Coord::new(10, 8);

        let diff = station.angle_rad(&asteroid) - FRAC_PI_2;

        assert!(diff < 1e-10);
    }

    #[test]
    fn calculates_pi_angle() {
        let station = Coord::new(5, 8);
        let asteroid = Coord::new(5, 1);

        let diff = station.angle_rad(&asteroid) - PI;

        assert!(diff < 1e-10);
    }

    #[test]
    fn calculates_3_pi_2_angle() {
        let station = Coord::new(5, 8);
        let asteroid = Coord::new(0, 8);

        let diff = station.angle_rad(&asteroid) - 3.0 * FRAC_PI_2;

        assert!(diff < 1e-10);
    }

    #[test]
    fn correctly_sorts_asteroids() {
        let station = Coord::new(4, 5);
        let asteroids = vec![
            Coord::new(0, 0),
            Coord::new(7, 3),
            Coord::new(10, 5),
            Coord::new(2, 9),
        ];
        let mut asteroids_hash = HashSet::new();
        for asteroid in asteroids {
            asteroids_hash.insert(asteroid);
        }
        let expected_order = vec![
            Coord::new(10, 5),
            Coord::new(7, 3),
            Coord::new(0, 0),
            Coord::new(2, 9),
        ];

        assert_eq!(sort_clockwise(&asteroids_hash, &station), expected_order);
    }

    #[test]
    fn finds_200th_asteroid_on_big_map() {
        let input = vec![
            ".#..##.###...#######",
            "##.############..##.",
            ".#.######.########.#",
            ".###.#######.####.#.",
            "#####.##.#.##.###.##",
            "..#####..#.#########",
            "####################",
            "#.####....###.#.#.##",
            "##.#################",
            "#####.##.###..####..",
            "..######..##.#######",
            "####.##.####...##..#",
            ".#####..#.######.###",
            "##...#.##########...",
            "#.##########.#######",
            ".####.#.###.###.#.##",
            "....##.##.###..#####",
            ".#.#.###########.###",
            "#.#.#.#####.####.###",
            "###.##.####.##.#..##",
        ];
        assert_eq!(Coord::new(8, 2), find_200th_vaporized_asteroid(&input, &Coord::new(11, 13)));
    }
}
