use std::collections::HashMap;

pub fn aoc_14_01() -> usize {
    0
}

#[derive(Debug, PartialEq)]
struct ReceiptLine {
    ingredients: Vec<Component>,
    result: Component,
}

type Receipt = HashMap<String, ReceiptLine>;

type Resources = HashMap<String, usize>;

#[derive(Debug, PartialEq)]
struct Component {
    quantity: usize,
    name: String,
}

impl Component {
    fn new(quantity: usize, name: &str) -> Self {
        Self {
            quantity,
            name: name.into(),
        }
    }

    fn from_vec(items: Vec<(usize, &str)>) -> Vec<Self> {
        items
            .into_iter()
            .map(|(quantity, name)| Self {
                quantity,
                name: name.into(),
            })
            .collect()
    }
}

fn parse_input(lines: &Vec<String>) -> Receipt {
    let mut result = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split("=>").collect();
        let left_part = parts[0];
        let right_part = parts[1];
        let receipt_line = ReceiptLine{
            ingredients: parse_multiple_component(left_part),
            result: parse_single_component(right_part),
        };
        result.insert(receipt_line.result.name.clone(), receipt_line);
    }

    result
}

fn solve_receipt(receipt: &Receipt, component_name: &str, resources: &mut Resources) {
    // TODO
}

fn parse_multiple_component(line: &str) -> Vec<Component> {
    line.split(",").map(parse_single_component).collect()
}

fn parse_single_component(line: &str) -> Component {
    let parts: Vec<&str> = line.trim().split(" ").collect();
    let quantity = usize::from_str_radix(parts[0], 10).unwrap();
    Component::new(quantity, parts[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_single_component() {
        let expected = Component::new(3, "Fuel");
        assert_eq!(expected, parse_single_component(" 3 Fuel "));
    }

    #[test]
    fn parses_multiple_components() {
        let expected = Component::from_vec(vec![(3, "Fuel"), (6, "Ore")]);
        assert_eq!(expected, parse_multiple_component(" 3 Fuel, 6 Ore "));
    }

    #[test]
    fn parses_input_to_receipt() {
        let input = [
            "10 ORE => 10 A",
            "1 ORE => 1 B",
            "7 A, 1 B => 1 C",
            "7 A, 1 C => 1 D",
            "7 A, 1 D => 1 E",
            "7 A, 1 E => 1 FUEL",
        ]
        .iter()
        .map(|l| String::from(*l))
        .collect();

        let mut expected: Receipt = HashMap::new();
        expected.insert("A".into(), ReceiptLine{
            result: Component::new(10, "A"),
            ingredients: Component::from_vec(vec![(10, "ORE")]),
        });
        expected.insert("B".into(), ReceiptLine{
            result: Component::new(1, "B"),
            ingredients: Component::from_vec(vec![(1, "ORE")]),
        });
        expected.insert("C".into(), ReceiptLine{
            result: Component::new(1, "C"),
            ingredients: Component::from_vec(vec![(7, "A"), (1, "B")]),
        });
        expected.insert("D".into(), ReceiptLine{
            result: Component::new(1, "D"),
            ingredients: Component::from_vec(vec![(7, "A"), (1, "C")]),
        });
        expected.insert("E".into(), ReceiptLine{
            result: Component::new(1, "E"),
            ingredients: Component::from_vec(vec![(7, "A"), (1, "D")]),
        });
        expected.insert("FUEL".into(), ReceiptLine{
            result: Component::new(1, "FUEL"),
            ingredients: Component::from_vec(vec![(7, "A"), (1, "E")]),
        });

        let parsed = parse_input(&input);

        assert_eq!(expected, parsed);
    }

    #[test]
    fn solves_small_example() {
        let input = [
            "10 ORE => 10 A",
            "1 ORE => 1 B",
            "7 A, 1 B => 1 C",
            "7 A, 1 C => 1 D",
            "7 A, 1 D => 1 E",
            "7 A, 1 E => 1 FUEL",
        ]
        .iter()
        .map(|l| String::from(*l))
        .collect();

        let expected = 31;

        let mut resources = HashMap::new();

        solve_receipt(&parse_input(&input), "FUEL", &mut resources);

        assert_eq!(expected, *resources.get("ORE").unwrap_or(&0));

    }
}
