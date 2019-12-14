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
    let mut cache: HashMap<String, Vec<String>> = HashMap::new();
    for relation in read_lines() {
        let (name1, name2) = parse(&relation);
        relations.push((name1.clone(), name2.clone()));
    }

    for relation in relations.iter() {
        let mut path_to_sun = down_to_the_root(&relation.1, &relations, &cache);
        let mut prev: Vec<String> = match cache.get(&path_to_sun.pop().unwrap()) {
            None => vec![],
            Some(prev) => prev.clone(),
        };
        path_to_sun.reverse();
        for orbit in path_to_sun {
            cache.insert(orbit.clone(), prev.clone());
            prev.insert(0, orbit);
        }
    }

    let mut total: u32 = 0;

    for (_, value) in cache {
        total += value.len() as u32;
    }

    total
}
fn parse(relation: &str) -> (String, String) {
    let names: Vec<&str> = relation.split(")").collect();
    (names[0].to_owned(), names[1].to_owned())
}

fn down_to_the_root(
    start: &str,
    relations: &Vec<(String, String)>,
    cache: &HashMap<String, Vec<String>>,
) -> Vec<String> {
    match relations.iter().find(|(_, planet)| planet == start) {
        None => vec![],
        Some((sun, planet)) => {
            if cache.contains_key(planet) {
                vec![planet.into()]
            } else {
                let mut result = down_to_the_root(sun, relations, cache);
                if result.is_empty() {
                    result = vec![sun.into()];
                }
                result.push(start.into());
                result
    }
}
    }
}
