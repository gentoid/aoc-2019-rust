use std::collections::HashMap;

static ORE: &str = "ORE";
static FUEL: &str = "FUEL";

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

impl Default for Component {
    fn default() -> Self {
        Self {
            quantity: 1,
            name: FUEL.into(),
        }
    }
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

    fn with_available(&self, available: usize) -> Self {
        Self::new(self.quantity - available, self.name.as_ref())
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

fn calculate_receipt(receipt: &Receipt, resources: &mut Resources, produce: &Component) -> usize {
    if produce.name == ORE {
        return produce.quantity;
    }

    let mut ore_required = 0;

    if !resources.contains_key(&produce.name) {
        resources.insert(produce.name.clone(), 0);
    }

    let available = resources.get(&produce.name).unwrap();
    if *available >= produce.quantity {
        resources
            .get_mut(&produce.name)
            .map(|quantity| *quantity -= produce.quantity);

        return 0;
    } else {
        let receipt_line = receipt.get(&produce.name).unwrap();

        for component in receipt_line.ingredients.iter() {
            ore_required += calculate_receipt(receipt, resources, component);
            resources
                .get_mut(&produce.name)
                .map(|quantity| *quantity += produce.quantity);
        }
    }

    ore_required
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
    fn calculates_one_line() {
        let input = [
            "8 ORE => 1 FUEL",
        ]
        .iter()
        .map(|l| String::from(*l))
        .collect();

        let expected = 8;
        let mut resources = HashMap::new();

        let calculated =
            calculate_receipt(&parse_input(&input), &mut resources, &Component::default());

        assert_eq!(expected, calculated);
    }

    #[test]
    fn calculates_two_lines() {
        let input = [
            "8 A => 1 FUEL",
            "2 ORE => 1 A"
        ]
        .iter()
        .map(|l| String::from(*l))
        .collect();

        let expected = 16;
        let mut resources = HashMap::new();

        let calculated =
            calculate_receipt(&parse_input(&input), &mut resources, &Component::default());

        assert_eq!(expected, calculated);
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

        let calculated =
            calculate_receipt(&parse_input(&input), &mut resources, &Component::default());

        assert_eq!(expected, calculated);
    }
}