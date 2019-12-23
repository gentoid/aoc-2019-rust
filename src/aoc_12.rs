use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_lines() -> Vec<String> {
    let file = File::open("inputs/input-12.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.unwrap()).collect()
}

fn parse_line(line: &str) -> Moon {
    let coord: Vec<isize> = line
        .to_owned()
        .replace("<", "")
        .replace(">", "")
        .replace("x=", "")
        .replace("y=", "")
        .replace("z=", "")
        .split(",")
        .into_iter()
        .map(|l| isize::from_str_radix(l.trim().as_ref(), 10).unwrap())
        .collect();

    Moon::new(coord[0], coord[1], coord[2])
}

fn prepare_input() -> Vec<Moon> {
    read_lines()
        .iter()
        .map(|l| parse_line(l.as_ref()))
        .collect()
}

pub fn aoc_12_01() -> isize {
    let mut input = prepare_input();
    let mut moons: Vec<&mut Moon> = vec![];
    for moon in input.iter_mut() {
        moons.push(moon);
    }

    for _ in 0..1000 {
        update_velocities(&mut moons);
        update_positions(&mut moons);
    }

    moons.iter().map(|moon| moon.energy()).sum()
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Moon {
    coord: [isize; 3],
    velocity: [isize; 3],
}

impl Moon {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self {
            coord: [x, y, z],
            velocity: [0, 0, 0],
        }
    }
    fn energy(&self) -> isize {
        let mut result = 0;

        for axis in 0..3 {
            result += self.coord[axis].abs();
            result += self.velocity[axis].abs();
        }

        result
    }

    fn update_velocity(&mut self, other: &Moon) {
        for axis in 0..3 {
            if self.coord[axis] > other.coord[axis] {
                self.velocity[axis] -= 1;
            }
            if self.coord[axis] < other.coord[axis] {
                self.velocity[axis] += 1;
            }
        }
    }

    fn update_position(&mut self) {
        for axis in 0..3 {
            self.coord[axis] += self.velocity[axis];
        }
    }
}

fn update_velocities(moons: &mut Vec<&mut Moon>) {
    let mut moons_clone: Vec<Moon> = vec![];

    for moon in moons.iter() {
        moons_clone.push(**moon.clone());
    }

    for moon_update in moons.iter_mut() {
        for moon in moons_clone.iter() {
            moon_update.update_velocity(&moon);
        }
    }
}

fn update_positions(moons: &mut Vec<&mut Moon>) {
    for moon in moons.iter_mut() {
        moon.update_position();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_velocity_for_pair() {
        let mut moon1 = Moon {
            coord: [7, 3, -5],
            velocity: [-4, 2, 7],
        };
        let mut moon2 = Moon {
            coord: [4, 3, 7],
            velocity: [-1, -5, 6],
        };

        update_velocities(&mut vec![&mut moon1, &mut moon2]);

        let expected_result = (
            Moon {
                coord: [7, 3, -5],
                velocity: [-5, 2, 8],
            },
            Moon {
                coord: [4, 3, 7],
                velocity: [0, -5, 5],
            },
        );
        assert_eq!(expected_result, (moon1, moon2));
    }

    #[test]
    fn updates_position() {
        let mut moon = Moon {
            coord: [7, 3, -5],
            velocity: [-4, 2, 7],
        };

        update_positions(&mut vec![&mut moon]);

        let expected = Moon {
            coord: [3, 5, 2],
            velocity: [-4, 2, 7],
        };
        assert_eq!(expected, moon);
    }

    #[test]
    fn calculates_velocity() {
        let mut moon1 = Moon {
            coord: [1, 2, 3],
            velocity: [5, -2, 0],
        };
        let mut moon2 = Moon {
            coord: [-8, 5, 0],
            velocity: [6, -8, 4],
        };
        let mut moon3 = Moon {
            coord: [0, 6, -4],
            velocity: [-5, -6, -2],
        };

        update_velocities(&mut vec![&mut moon1, &mut moon2, &mut moon3]);

        let expect1 = Moon {
            coord: [1, 2, 3],
            velocity: [3, 0, -2],
        };
        let expect2 = Moon {
            coord: [-8, 5, 0],
            velocity: [8, -8, 4],
        };
        let expect3 = Moon {
            coord: [0, 6, -4],
            velocity: [-5, -8, 0],
        };
        assert_eq!((expect1, expect2, expect3), (moon1, moon2, moon3));
    }

    #[test]
    fn first_example() {
        let mut moon1 = Moon::new(-1, 0, 2);
        let mut moon2 = Moon::new(2, -10, -7);
        let mut moon3 = Moon::new(4, -8, 8);
        let mut moon4 = Moon::new(3, 5, -1);

        let mut moons = vec![&mut moon1, &mut moon2, &mut moon3, &mut moon4];

        update_velocities(&mut moons);
        update_positions(&mut moons);

        let mut expect1 = Moon {
            coord: [2, -1, 1],
            velocity: [3, -1, -1],
        };
        let mut expect2 = Moon {
            coord: [3, -7, -4],
            velocity: [1, 3, 3],
        };
        let mut expect3 = Moon {
            coord: [1, -7, 5],
            velocity: [-3, 1, -3],
        };
        let mut expect4 = Moon {
            coord: [2, 2, 0],
            velocity: [-1, -3, 1],
        };

        let expect_moons = vec![&mut expect1, &mut expect2, &mut expect3, &mut expect4];

        assert_eq!(expect_moons, moons);

        // 2nd step

        let mut moon1 = expect1;
        let mut moon2 = expect2;
        let mut moon3 = expect3;
        let mut moon4 = expect4;

        let mut moons = vec![&mut moon1, &mut moon2, &mut moon3, &mut moon4];

        update_velocities(&mut moons);
        update_positions(&mut moons);

        let mut expect1 = Moon {
            coord: [5, -3, -1],
            velocity: [0, -3, 0],
        };
        let mut expect2 = Moon {
            coord: [1, -2, 2],
            velocity: [-2, 5, 6],
        };
        let mut expect3 = Moon {
            coord: [1, -4, -1],
            velocity: [0, 3, -6],
        };
        let mut expect4 = Moon {
            coord: [1, -4, 2],
            velocity: [-1, -6, 2],
        };

        let expect_moons = vec![&mut expect1, &mut expect2, &mut expect3, &mut expect4];

        assert_eq!(expect_moons, moons);
    }
}
