use {itertools::Itertools, std::fs};

fn read() -> String {
    fs::read_to_string("inputs/input-08.txt")
        .unwrap()
        .trim()
        .into()
}

pub fn aoc_08_01() -> usize {
    let input = read();
    let layers = split_into_layers(&input, 25, 6);
    let layer = find_with_minimal_zeros(&layers);

    count_chars(&layer, '1') * count_chars(&layer, '2')
}

fn split_into_layers(input: &str, width: usize, heigth: usize) -> Vec<Vec<char>> {
    input
        .chars()
        .chunks(width * heigth)
        .into_iter()
        .map(|chunk| chunk.collect())
        .collect()
}

fn find_with_minimal_zeros(layers: &Vec<Vec<char>>) -> Vec<char> {
    let mut found_layer = layers[0].clone();
    let mut min_zeros = layers[0].len();

    for layer in layers {
        let zeros = count_chars(&layer, '0');
        if zeros < min_zeros {
            min_zeros = zeros;
            found_layer = layer.clone();
        }
    }

    found_layer
}

fn count_chars(layer: &Vec<char>, character: char) -> usize {
    layer
        .iter()
        .filter(|ch| **ch == character)
        .collect::<String>()
        .len()
}
