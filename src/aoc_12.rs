use {crate::read_input::read_lines, num_integer::lcm};

fn parse_line(line: &str) -> Vec<isize> {
    line.to_owned()
        .replace("<", "")
        .replace(">", "")
        .replace("x=", "")
        .replace("y=", "")
        .replace("z=", "")
        .split(",")
        .into_iter()
        .map(|l| isize::from_str_radix(l.trim().as_ref(), 10).unwrap())
        .collect()
}

fn prepare_input() -> Vec<Moon> {
    read_lines(12)
        .iter()
        .map(|l| {
            let line = parse_line(l.as_ref());
            Moon::new((line[0], line[1], line[2]), (0, 0, 0))
        })
        .collect()
}

fn prepare_moon1d_input() -> Vec<Vec<Moon1D>> {
    read_lines(12)
        .iter()
        .map(|l| {
            let line = parse_line(l.as_ref());
            vec![
                Moon1D::new(line[0], 0),
                Moon1D::new(line[1], 0),
                Moon1D::new(line[2], 0),
            ]
        })
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

pub fn aoc_12_02() -> usize {
    let moons = prepare_moon1d_input();

    let x_cycle = find_cycle(&moons, 0);
    let y_cycle = find_cycle(&moons, 1);
    let z_cycle = find_cycle(&moons, 2);

    lcm(x_cycle, lcm(y_cycle, z_cycle))
}

fn find_cycle(moons: &Vec<Vec<Moon1D>>, axis_index: usize) -> usize {
    let mut cloned = moons.clone();
    let original: Vec<Moon1D> = cloned.iter().map(|m| m[axis_index]).collect();
    let mut moons: Vec<&mut Moon1D> = cloned.iter_mut().map(|m| &mut m[axis_index]).collect();

    let mut counter = 0;

    loop {
        update_velocities_1d(&mut moons);
        update_positions_1d(&mut moons);

        counter += 1;

        let mut done = true;
        for i in 0..original.len() {
            if moons[i] != &original[i] {
                done = false;
                break;
            }
        }

        if done {
            break;
        }
    }

    counter
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Moon {
    x: Moon1D,
    y: Moon1D,
    z: Moon1D,
}

impl Moon {
    fn new((x, y, z): (isize, isize, isize), (vx, vy, vz): (isize, isize, isize)) -> Self {
        Self {
            x: Moon1D::new(x, vx),
            y: Moon1D::new(y, vy),
            z: Moon1D::new(z, vz),
        }
    }

    fn energy(&self) -> isize {
        let (x_pot, x_kin) = self.x.energy();
        let (y_pot, y_kin) = self.y.energy();
        let (z_pot, z_kin) = self.z.energy();

        (x_pot + y_pot + z_pot) * (x_kin + y_kin + z_kin)
    }

    fn update_position(&mut self) {
        self.x.update_position();
        self.y.update_position();
        self.z.update_position();
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Moon1D {
    position: isize,
    velocity: isize,
}

impl Moon1D {
    fn new(position: isize, velocity: isize) -> Self {
        Self { position, velocity }
    }

    fn update_velocity(&mut self, other: Moon1D) {
        if self.position > other.position {
            self.velocity -= 1;
        } else if self.position < other.position {
            self.velocity += 1;
        }
    }

    fn update_position(&mut self) {
        self.position += self.velocity;
    }

    fn energy(&self) -> (isize, isize) {
        (self.position.abs(), self.velocity.abs())
    }
}

fn update_velocities(moons: &mut Vec<&mut Moon>) {
    let mut x_moons: Vec<&mut Moon1D> = moons.iter_mut().map(|m| &mut m.x).collect();
    update_velocities_1d(&mut x_moons);

    let mut y_moons: Vec<&mut Moon1D> = moons.iter_mut().map(|m| &mut m.y).collect();
    update_velocities_1d(&mut y_moons);

    let mut z_moons: Vec<&mut Moon1D> = moons.iter_mut().map(|m| &mut m.z).collect();
    update_velocities_1d(&mut z_moons);
}

fn update_velocities_1d(moons: &mut Vec<&mut Moon1D>) {
    let mut moons_clone: Vec<Moon1D> = vec![];

    for moon in moons.iter() {
        moons_clone.push(**moon.clone());
    }

    for moon_update in moons.iter_mut() {
        for moon in moons_clone.iter() {
            moon_update.update_velocity(*moon);
        }
    }
}

fn update_positions(moons: &mut Vec<&mut Moon>) {
    for moon in moons.iter_mut() {
        moon.update_position();
    }
}

fn update_positions_1d(moons: &mut Vec<&mut Moon1D>) {
    for moon in moons.iter_mut() {
        moon.update_position();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_velocity_for_pair() {
        let mut moon1 = Moon::new((7, 3, -5), (-4, 2, 7));
        let mut moon2 = Moon::new((4, 3, 7), (-1, -5, 6));

        update_velocities(&mut vec![&mut moon1, &mut moon2]);

        let expected_result = (
            Moon::new((7, 3, -5), (-5, 2, 8)),
            Moon::new((4, 3, 7), (0, -5, 5)),
        );
        assert_eq!(expected_result, (moon1, moon2));
    }

    #[test]
    fn updates_position() {
        let mut moon = Moon::new((7, 3, -5), (-4, 2, 7));

        update_positions(&mut vec![&mut moon]);

        let expected = Moon::new((3, 5, 2), (-4, 2, 7));
        assert_eq!(expected, moon);
    }

    #[test]
    fn calculates_velocity() {
        let mut moon1 = Moon::new((1, 2, 3), (5, -2, 0));
        let mut moon2 = Moon::new((-8, 5, 0), (6, -8, 4));
        let mut moon3 = Moon::new((0, 6, -4), (-5, -6, -2));

        update_velocities(&mut vec![&mut moon1, &mut moon2, &mut moon3]);

        let expect1 = Moon::new((1, 2, 3), (3, 0, -2));
        let expect2 = Moon::new((-8, 5, 0), (8, -8, 4));
        let expect3 = Moon::new((0, 6, -4), (-5, -8, 0));
        assert_eq!((expect1, expect2, expect3), (moon1, moon2, moon3));
    }

    #[test]
    fn first_example() {
        let mut moon1 = Moon::new((-1, 0, 2), (0, 0, 0));
        let mut moon2 = Moon::new((2, -10, -7), (0, 0, 0));
        let mut moon3 = Moon::new((4, -8, 8), (0, 0, 0));
        let mut moon4 = Moon::new((3, 5, -1), (0, 0, 0));

        let mut moons = vec![&mut moon1, &mut moon2, &mut moon3, &mut moon4];

        update_velocities(&mut moons);
        update_positions(&mut moons);

        let mut expect1 = Moon::new((2, -1, 1), (3, -1, -1));
        let mut expect2 = Moon::new((3, -7, -4), (1, 3, 3));
        let mut expect3 = Moon::new((1, -7, 5), (-3, 1, -3));
        let mut expect4 = Moon::new((2, 2, 0), (-1, -3, 1));

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

        let mut expect1 = Moon::new((5, -3, -1), (3, -2, -2));
        let mut expect2 = Moon::new((1, -2, 2), (-2, 5, 6));
        let mut expect3 = Moon::new((1, -4, -1), (0, 3, -6));
        let mut expect4 = Moon::new((1, -4, 2), (-1, -6, 2));

        let expect_moons = vec![&mut expect1, &mut expect2, &mut expect3, &mut expect4];

        assert_eq!(expect_moons, moons);
    }
}
