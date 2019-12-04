use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() {
    let nums = File::open("aoc-01/input-01.txt").and_then(read_and_parse);
    match nums.as_ref().map(aoc_01_01) {
        Ok(res) => println!("01 / 01: {}", res),
        Err(err) => println!("{}", err),
    }
    match nums.as_ref().map(aoc_01_02) {
        Ok(res) => println!("01 / 02: {}", res),
        Err(err) => println!("{}", err),
    }
}

fn aoc_01_01(nums: &Vec<i32>) -> i32 {
    nums.iter().map(calculate_fuel).sum()
}

fn aoc_01_02(nums: &Vec<i32>) -> i32 {
    nums.iter().map(calculate_total_fuel).sum()
}

fn read_and_parse(file: File) -> io::Result<Vec<i32>> {
    let reader = BufReader::new(file);
    let mut result = vec![];

    for line in reader.lines() {
        result.push(i32::from_str_radix(line?.as_ref(), 10).expect("Cannot parse LINE"));
    }

    Ok(result)
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
            break
        }

        total_fuel += fuel;
        mass = fuel;
    }

    total_fuel
}
