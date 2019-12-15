use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

type Cache = HashMap<String, (u32, Option<String>)>;

fn read_lines() -> Vec<String> {
    let file = File::open("input-06.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.unwrap()).collect()
}

pub fn aoc_06_01() -> u32 {
    let cache = prepare_tree_cache();

    let mut total: u32 = 0;
    for (_, value) in cache {
        total += value.0 as u32;
    }

    total
}

pub fn aoc_06_02() -> u32 {
    let cache = prepare_tree_cache();
    let mut you = get_path("YOU", &cache).unwrap();
    let mut san = get_path("SAN", &cache).unwrap();

    loop {
        if you[0] == san[0] {
            you.remove(0);
            san.remove(0);
        } else {
            break;
        }
    }
    (you.len() + san.len()) as u32 - 2
}

fn prepare_tree_cache() -> HashMap<String, (u32, Option<String>)> {
    let mut relations: Vec<(String, String)> = vec![];
    let mut cache: HashMap<String, (u32, Option<String>)> = HashMap::new();
    for relation in read_lines() {
        let (name1, name2) = parse(&relation);
        relations.push((name1.clone(), name2.clone()));
    }

    for relation in relations.iter() {
        let path = solve_path(&relation.1, &relations, &cache);
        update_cache(&path, &mut cache);
    }

    cache
}

fn parse(relation: &str) -> (String, String) {
    let names: Vec<&str> = relation.split(")").collect();
    (names[0].to_owned(), names[1].to_owned())
}

fn solve_path(
    find_for: &str,
    relations: &Vec<(String, String)>,
    cache: &HashMap<String, (u32, Option<String>)>,
) -> (Vec<String>, (u32, Option<String>)) {
    match cache.get::<String>(&find_for.into()) {
        Some(cached) => (vec![find_for.into()], cached.clone()),
        None => match relations.iter().find(|(_, planet)| planet == find_for) {
            None => (vec![find_for.into()], (0, None)),
            Some((sun, planet)) => {
                let (mut path, cached) = solve_path(sun, relations, cache);
                path.push(planet.into());
                (path, cached)
            }
        },
    }
}

fn update_cache(
    (path, cached): &(Vec<String>, (u32, Option<String>)),
    cache: &mut HashMap<String, (u32, Option<String>)>,
) {
    if path.is_empty() {
        return;
    }
    let mut path = path.clone();
    let mut cached = cached.clone();
    let mut prev = path.remove(0);

    if cached.1.is_none() {
        cache.insert(prev.clone(), (0, None));
    }

    for next in path {
        cached = (cached.0 + 1, Some(prev));
        prev = next.clone();
        cache.insert(next, cached.clone());
    }
}

fn get_path(planet: &str, cache: &Cache) -> Option<Vec<String>> {
    match cache.get(planet) {
        None => None,
        Some((_, next)) => match next {
            None => Some(vec![planet.into()]),
            Some(next) => get_path(next, cache).map(|path| {
                let mut path = path.clone();
                path.push(planet.into());
                path
            }),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_sun_of_a_single_relation() {
        let relations = vec![("sun".into(), "planet".into())];
        let res = solve_path("planet", &relations, &HashMap::new());
        assert_eq!(vec!["sun", "planet"], res.0);
    }

    #[test]
    fn finds_sun_for_two_relations() {
        let relations = vec![
            ("sun".into(), "planet1".into()),
            ("planet1".into(), "planet2".into()),
        ];
        let res = solve_path("planet2", &relations, &HashMap::new());
        assert_eq!(vec!["sun", "planet1", "planet2"], res.0);
    }

    #[test]
    fn finds_path_to_sun_for_randomly_placed_relations() {
        let relations = vec![
            ("planet2".into(), "planet3".into()),
            ("sun".into(), "planet1".into()),
            ("planet1".into(), "planet2".into()),
        ];
        let res = solve_path("planet3", &relations, &HashMap::new());
        assert_eq!(vec!["sun", "planet1", "planet2", "planet3"], res.0);
    }

    #[test]
    fn with_empty_cache_returns_none_as_cached() {
        let cache: HashMap<String, (u32, Option<String>)> = HashMap::new();
        let relations = vec![
            ("sun".into(), "planet1".into()),
            ("planet2".into(), "planet3".into()),
            ("planet1".into(), "planet2".into()),
        ];
        let path = solve_path("planet3", &relations, &cache);
        assert_eq!(
            (
                vec![
                    "sun".into(),
                    "planet1".into(),
                    "planet2".into(),
                    "planet3".into()
                ],
                (0, None)
            ),
            path
        );
    }

    #[test]
    fn path_solver_gets_value_from_cache() {
        let mut cache: HashMap<String, (u32, Option<String>)> = HashMap::new();
        cache.insert("sun".into(), (0, None));
        cache.insert("other_planet1".into(), (1, Some("sun".into())));
        cache.insert("planet2".into(), (2, Some("other_planet1".into())));
        let relations = vec![
            ("sun".into(), "planet1".into()),
            ("planet2".into(), "planet3".into()),
            ("planet1".into(), "planet2".into()),
        ];
        let path = solve_path("planet3", &relations, &cache);
        assert_eq!(
            (
                vec!["planet2".into(), "planet3".into()],
                (2, Some("other_planet1".into()))
            ),
            path
        );
    }

    #[test]
    fn updates_cache_with_new_path() {
        let mut cache = HashMap::new();
        cache.insert("sun".into(), (0, None));
        cache.insert("planet1".into(), (1, Some("sun".into())));
        cache.insert("planet2".into(), (2, Some("planet1".into())));
        let path = (
            vec!["planet2".into(), "planet3".into()],
            (2, Some("planet1".into())),
        );

        update_cache(&path, &mut cache);

        let mut correct_cache: HashMap<String, (u32, Option<String>)> = HashMap::new();
        correct_cache.insert("sun".into(), (0, None));
        correct_cache.insert("planet1".into(), (1, Some("sun".into())));
        correct_cache.insert("planet2".into(), (2, Some("planet1".into())));
        correct_cache.insert("planet3".into(), (3, Some("planet2".into())));
        assert_eq!(correct_cache, cache);
    }

    #[test]
    fn adds_data_to_empty_cache() {
        let mut cache: HashMap<String, (u32, Option<String>)> = HashMap::new();
        let path = (
            vec![
                "sun".into(),
                "planet1".into(),
                "planet2".into(),
                "planet3".into(),
            ],
            (0, None),
        );

        update_cache(&path, &mut cache);

        let mut correct_cache: HashMap<String, (u32, Option<String>)> = HashMap::new();
        correct_cache.insert("sun".into(), (0, None));
        correct_cache.insert("planet1".into(), (1, Some("sun".into())));
        correct_cache.insert("planet2".into(), (2, Some("planet1".into())));
        correct_cache.insert("planet3".into(), (3, Some("planet2".into())));
        assert_eq!(correct_cache, cache);
    }

    #[test]
    fn gets_path_from_cache() {
        let mut cache = HashMap::new();
        cache.insert("sun".into(), (0, None));
        cache.insert("planet1".into(), (1, Some("sun".into())));
        cache.insert("planet2".into(), (2, Some("planet1".into())));

        let path = get_path("planet2", &cache);
        assert_eq!(
            Some(vec!["sun".into(), "planet1".into(), "planet2".into()]),
            path
        );
    }
}
