use std::fs;

fn read() -> String {
    fs::read_to_string("inputs/input-08.txt")
        .unwrap()
        .trim()
        .into()
}

fn to_chars(input: &str) -> Vec<char> {
    input.chars().into_iter().collect()
}

pub fn aoc_08_01() -> usize {
    let input: Vec<char> = to_chars(&read());
    let layers = split_into_layers(&input, 25, 6);
    let layer = find_with_minimal_zeros(&layers);

    count_chars(&layer, '1') * count_chars(&layer, '2')
}

pub fn aoc_08_02() -> Vec<String> {
    let width = 25;
    let heigth = 6;
    let input: Vec<char> = to_chars(&read());
    let composed = compose_layers(&split_into_layers(&input, width, heigth));
    to_image(&composed, width)
}

fn to_image(layer: &Vec<char>, width: usize) -> Vec<String> {
    layer
        .chunks(width)
        .into_iter()
        .map(|chunk| chunk.iter().collect())
        .collect()
}

fn compose_layers(layers: &Vec<Vec<char>>) -> Vec<char> {
    let mut layers = layers.clone();
    let mut top = layers.remove(0);

    for layer in layers {
        top = compose_two_layers(&top, &layer);
    }

    top
}

fn compose_two_layers(top: &Vec<char>, bottom: &Vec<char>) -> Vec<char> {
    top.iter()
        .zip(bottom.iter())
        .map(|(&t, &b)| if t == '0' || t == '1' { t } else { b })
        .collect()
}

fn split_into_layers(input: &Vec<char>, width: usize, heigth: usize) -> Vec<Vec<char>> {
    input
        .chunks(width * heigth)
        .into_iter()
        .map(|chunk| chunk.into())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_small_test_image() {
        let input = to_chars("0222112222120000");
        let composed = compose_layers(&split_into_layers(&input, 2, 2))
            .iter()
            .collect::<String>();

        assert_eq!(composed, "0110");
    }
}
