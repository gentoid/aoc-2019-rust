use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

pub fn read_lines(day_number: usize) -> Vec<String> {
    let file = File::open(format!("inputs/input-{:02}.txt", day_number)).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.unwrap()).collect()
}

pub fn read_intcode_program(day_number: usize) -> Vec<isize> {
    fs::read_to_string(format!("inputs/input-{:02}.txt", day_number))
        .unwrap()
        .trim()
        .split(",")
        .map(|string| isize::from_str_radix(string.as_ref(), 10).unwrap())
        .collect()
}
