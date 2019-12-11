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

    let mut total: u32 = 0;
    // TODO: reimplement it by caching imntermediate results - use orbit_counter
    for relation in relations.iter() {
        total += down_to_the_root(&relation.1, &relations)
    }
    total
}
fn parse(relation: &str) -> (String, String) {
    let names: Vec<&str> = relation.split(")").collect();
    (names[0].to_owned(), names[1].to_owned())
}

fn down_to_the_root(start: &str, relations: &Vec<(String, String)>) -> u32 {
    match relations.iter().find(|(_, n)| n == start) {
        None => 0,
        Some(r) => 1 + down_to_the_root(&r.0, relations),
    }
}
