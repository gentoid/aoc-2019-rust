use crate::read_input::read_lines;

pub fn aoc_01_01() -> i32 {
    read_and_parse().iter().map(calculate_fuel).sum()
}

pub fn aoc_01_02() -> i32 {
    read_and_parse().iter().map(calculate_total_fuel).sum()
}

fn read_and_parse() -> Vec<i32> {
    let lines = read_lines(1);
    let mut result = vec![];

    for line in lines {
        result.push(i32::from_str_radix(line.as_ref(), 10).expect("Cannot parse LINE"));
    }

    result
}

fn calculate_fuel(mass: &i32) -> i32 {
    let fuel = (*mass as f64 / 3.0).floor() as i32 - 2;

    if fuel < 0 {
        0
    } else {
        fuel
    }
}

fn calculate_total_fuel(mass: &i32) -> i32 {
    let mut total_fuel = 0;
    let mut mass = *mass;

    loop {
        let fuel = calculate_fuel(&mass);
        if fuel == 0 {
            break;
        }

        total_fuel += fuel;
        mass = fuel;
    }

    total_fuel
}
