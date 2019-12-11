use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn read_lines() -> Vec<String> {
    let file = File::open("input-06.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.unwrap()).collect()
}

pub fn aoc_06_01() -> u32 {
    let mut relations: Vec<(String, String)> = vec![];
    let mut orbit_counter: HashMap<String, u32> = HashMap::new();
    for relation in read_lines() {
        let (name1, name2) = parse(&relation);
        relations.push((name1.clone(), name2.clone()));
        orbit_counter.insert(name1, 0);
        orbit_counter.insert(name2, 0);
    }
    let sequence = down_to_the_root(&relations[0].1, &relations);
    println!(
        "Started with {}, ended up with {:?}",
        &relations[0].1, sequence
    );
    0 // just to make the compiler happy
}
fn parse(relation: &str) -> (String, String) {
    let names: Vec<&str> = relation.split(")").collect();
    (names[0].to_owned(), names[1].to_owned())
}

fn down_to_the_root(start: &str, relations: &Vec<(String, String)>) -> Vec<String> {
    let mut result = vec![start.to_owned()];
    relations.iter().find(|(_, n)| n == start).map(|r| {
        let mut next = down_to_the_root(&r.0, relations);
        result.append(&mut next);
    });

    result
}
