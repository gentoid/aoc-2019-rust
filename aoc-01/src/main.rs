use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() {
    match aoc_01_01() {
        Ok(res) => println!("{}", res),
        Err(err) => println!("{}", err),
    }
}

fn aoc_01_01() -> io::Result<i32> {
    let nums = read_and_parse(File::open("input-01.txt")?)?;

    let result = nums.iter().map(calculate_fuel).sum();
    Ok(result)
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
