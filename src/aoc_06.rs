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
    find_for: &str,
    relations: &Vec<(String, String)>,
    cache: &HashMap<String, Vec<String>>,
) -> Vec<String> {
    match cache.get::<String>(&find_for.into()) {
        Some(path) => {
            let mut path = path.clone();
            path.push(find_for.into());
            path
        }
        None => match relations.iter().find(|(_, planet)| planet == find_for) {
            None => vec![find_for.into()],
            Some((sun, planet)) => {
                let mut path = down_to_the_root(sun, relations, cache);
                path.push(planet.into());
                path
            }
        },
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

    #[test]
    fn path_solver_gets_value_from_cache() {
        let mut cache: HashMap<String, Vec<String>> = HashMap::new();
        cache.insert("sun".into(), vec![]);
        cache.insert("other_planet1".into(), vec!["sun".into()]);
        cache.insert(
            "planet2".into(),
            vec!["other_sun".into(), "other_planet1".into()],
        );
        let relations = vec![
            ("sun".into(), "planet1".into()),
            ("planet2".into(), "planet3".into()),
            ("planet1".into(), "planet2".into()),
        ];
        let path = down_to_the_root("planet3", &relations, &cache);
        assert_eq!(
            path,
            vec!["other_sun", "other_planet1", "planet2", "planet3"]
        );
    }
}
