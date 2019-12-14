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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_sun_of_a_single_relation() {
        let relations = vec![("sun".into(), "planet".into())];
        let res = down_to_the_root("planet", &relations, &HashMap::new());
        assert_eq!(vec!["sun", "planet"], res);
    }

    #[test]
    fn finds_sun_for_two_relations() {
        let relations = vec![
            ("sun".into(), "planet1".into()), 
            ("planet1".into(), "planet2".into()),
            ];
        let res = down_to_the_root("planet2", &relations, &HashMap::new());
        assert_eq!(vec!["sun", "planet1", "planet2"], res);
    }

    #[test]
    fn finds_path_to_sun_for_randomly_placed_relations() {
        let relations = vec![
            ("planet2".into(), "planet3".into()),
            ("sun".into(), "planet1".into()), 
            ("planet1".into(), "planet2".into()),
            ];
        let res = down_to_the_root("planet3", &relations, &HashMap::new());
        assert_eq!(vec!["sun", "planet1", "planet2", "planet3"], res);
    }
}
