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

    Moon {
        coord: [coord[0], coord[1], coord[2]],
        velocity: [0, 0, 0],
    }
}

fn prepare_input() -> Vec<Moon> {
    read_lines()
        .iter()
        .map(|l| parse_line(l.as_ref()))
        .collect()
}

pub fn aoc_12_01() -> isize {
    let mut moons = prepare_input();

    for _ in 0..1000 {
        moons = update_positions(&calculate_velocity(moons));
    }

    moons.iter().map(|moon| moon.energy()).sum()
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Moon {
    coord: [isize; 3],
    velocity: [isize; 3],
}

impl Moon {
    fn energy(&self) -> isize {
        let mut result = 0;

        for axis in 0..3 {
            result += self.coord[axis].abs();
            result += self.velocity[axis].abs();
        }

        result
    }
}

fn update_velocity_for_pair(moon_1: &mut Moon, moon_2: &mut Moon) {
    for axis in 0..=2 {
        let val_1 = moon_1.coord[axis];
        let val_2 = moon_2.coord[axis];

        if val_1 > val_2 {
            moon_1.velocity[axis] -= 1;
            moon_2.velocity[axis] += 1;
        } else if val_2 > val_1 {
            moon_1.velocity[axis] += 1;
            moon_2.velocity[axis] -= 1;
        }
    }
}

fn update_positions(moons: &Vec<Moon>) -> Vec<Moon> {
    let mut new_moons = vec![];

    for moon in moons {
        let mut moon = moon.clone();
        update_position(&mut moon);
        new_moons.push(moon);
    }

    new_moons
}

fn update_position(moon: &mut Moon) {
    for axis in 0..=2 {
        moon.coord[axis] += moon.velocity[axis];
    }
}

fn calculate_velocity(moons: Vec<Moon>) -> Vec<Moon> {
    let mut moons = moons;

    for index_1 in 0..(moons.len() - 1) {
        for index_2 in (index_1 + 1)..moons.len() {
            let mut moon_1 = moons[index_1].clone();
            let mut moon_2 = moons[index_2].clone();
            update_velocity_for_pair(&mut moon_1, &mut moon_2);
            moons[index_1] = moon_1;
            moons[index_2] = moon_2;
        }
    }

    moons
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_velocity_for_pair() {
        let mut moon_1 = Moon {
            coord: [7, 3, -5],
            velocity: [-4, 2, 7],
        };
        let mut moon_2 = Moon {
            coord: [4, 3, 7],
            velocity: [-1, -5, 6],
        };
        update_velocity_for_pair(&mut moon_1, &mut moon_2);

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
        assert_eq!(expected_result, (moon_1, moon_2));
    }

    #[test]
    fn updates_position() {
        let mut moon = Moon {
            coord: [7, 3, -5],
            velocity: [-4, 2, 7],
        };
        update_position(&mut moon);

        let expected = Moon {
            coord: [3, 5, 2],
            velocity: [-4, 2, 7],
        };
        assert_eq!(expected, moon);
    }

    #[test]
    fn calculates_velocity() {
        let moon_1 = Moon {
            coord: [1, 2, 3],
            velocity: [5, -2, 0],
        };
        let moon_2 = Moon {
            coord: [-8, 5, 0],
            velocity: [6, -8, 4],
        };
        let moon_3 = Moon {
            coord: [0, 6, -4],
            velocity: [-5, -6, -2],
        };

        let moons = calculate_velocity(vec![moon_1, moon_2, moon_3]);

        let expect_1 = Moon {
            coord: [1, 2, 3],
            velocity: [3, 0, -2],
        };
        let expect_2 = Moon {
            coord: [-8, 5, 0],
            velocity: [8, -8, 4],
        };
        let expect_3 = Moon {
            coord: [0, 6, -4],
            velocity: [-5, -8, 0],
        };
        assert_eq!(
            (expect_1, expect_2, expect_3),
            (moons[0], moons[1], moons[2])
        );
    }
}
