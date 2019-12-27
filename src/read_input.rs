use {
    std::{
        fs::File,
        io::{BufRead, BufReader},
    },
};

pub fn read_lines(day_number: usize) -> Vec<String> {
    let file = File::open(format!("inputs/input-{:02}.txt", day_number)).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.unwrap()).collect()
}
