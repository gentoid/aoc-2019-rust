use {crate::read_input::read_lines, std::collections::HashMap};

static ORE: &str = "ORE";
static FUEL: &str = "FUEL";

pub fn aoc_14_01() -> usize {
    let receipt = parse_input(&read_lines(14));
    calculate_receipt(&receipt, &mut HashMap::new(), &Component::default())
}

pub fn aoc_14_02() -> usize {
    let receipt = parse_input(&read_lines(14));
    calculated_fuel_from_1_trln_ores(&receipt) - 1
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
}

fn parse_input(lines: &[String]) -> Receipt {
    let mut result = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split("=>").collect();
        let left_part = parts[0];
        let right_part = parts[1];
        let receipt_line = ReceiptLine {
            ingredients: parse_multiple_component(left_part),
            result: parse_single_component(right_part),
        };
        result.insert(receipt_line.result.name.clone(), receipt_line);
    }

    result
}

fn calculated_fuel_from_1_trln_ores(receipt: &Receipt) -> usize {
    let ores = 1_000_000_000_000.0;
    let per_fuel = calculate_receipt_with_fractions(&receipt, FUEL);

    (ores / per_fuel).floor() as usize
}

fn calculate_receipt_with_fractions(receipt: &Receipt, resource_name: &str) -> f64 {
    if resource_name == ORE {
        return 1.0;
    }

    let mut ore_required = 0.0;

    let receipt_line = receipt.get(resource_name.into()).unwrap();
    for ingredient in receipt_line.ingredients.iter() {
        ore_required += ingredient.quantity as f64 / receipt_line.result.quantity as f64
            * calculate_receipt_with_fractions(receipt, &ingredient.name);
    }

    ore_required
}

fn calculate_receipt(receipt: &Receipt, resources: &mut Resources, produce: &Component) -> usize {
    if produce.name == ORE {
        return produce.quantity;
    }

    let mut ore_required = 0;

    if !resources.contains_key(&produce.name) {
        resources.insert(produce.name.clone(), 0);
    }

    loop {
        let available = resources.get(&produce.name).unwrap();
        if *available >= produce.quantity {
            resources
                .get_mut(&produce.name)
                .map(|quantity| *quantity -= produce.quantity);

            break;
        } else {
            ore_required += produce_resource(receipt, resources, produce);
        }
    }

    ore_required
}

fn produce_resource(receipt: &Receipt, resources: &mut Resources, component: &Component) -> usize {
    let mut ore_required = 0;

    let receipt_line = receipt.get(&component.name).unwrap();
    for component in receipt_line.ingredients.iter() {
        ore_required += calculate_receipt(receipt, resources, component);
    }

    resources
        .get_mut(&component.name)
        .map(|quantity| *quantity += receipt_line.result.quantity);

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

    fn str_to_string(vector: &[&str]) -> Vec<String> {
        vector.iter().map(|l| String::from(*l)).collect()
    }

    fn smapp_example() -> Vec<String> {
        str_to_string(&vec![
            "10 ORE => 10 A",
            "1 ORE => 1 B",
            "7 A, 1 B => 1 C",
            "7 A, 1 C => 1 D",
            "7 A, 1 D => 1 E",
            "7 A, 1 E => 1 FUEL",
        ])
    }

    fn first_larger_example() -> Vec<String> {
        str_to_string(&vec![
            "157 ORE => 5 NZVS",
            "165 ORE => 6 DCFZ",
            "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL",
            "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ",
            "179 ORE => 7 PSHF",
            "177 ORE => 5 HKGWZ",
            "7 DCFZ, 7 PSHF => 2 XJWVT",
            "165 ORE => 2 GPVTF",
            "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        ])
    }

    fn second_larger_example() -> Vec<String> {
        str_to_string(&vec![
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG",
            "17 NVRVD, 3 JNWZP => 8 VPVL",
            "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL",
            "22 VJHF, 37 MNCFX => 5 FWMGM",
            "139 ORE => 4 NVRVD",
            "144 ORE => 7 JNWZP",
            "5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC",
            "5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV",
            "145 ORE => 6 MNCFX",
            "1 NVRVD => 8 CXFTF",
            "1 VJHF, 6 MNCFX => 4 RFSQX",
            "176 ORE => 6 VJHF",
        ])
    }

    fn third_larger_example() -> Vec<String> {
        str_to_string(&vec![
            "171 ORE => 8 CNZTR",
            "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL",
            "114 ORE => 4 BHXH",
            "14 VRPVC => 6 BMBT",
            "6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL",
            "6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT",
            "15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW",
            "13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW",
            "5 BMBT => 4 WPTQ",
            "189 ORE => 9 KTJDG",
            "1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP",
            "12 VRPVC, 27 CNZTR => 2 XDBXC",
            "15 KTJDG, 12 BHXH => 5 XCVML",
            "3 BHXH, 2 VRPVC => 7 MZWV",
            "121 ORE => 7 VRPVC",
            "7 XCVML => 6 RJRHP",
            "5 BHXH, 4 VRPVC => 5 LTCX",
        ])
    }

    fn calculate(input: &[String]) -> usize {
        let mut resources = HashMap::new();
        calculate_receipt(&parse_input(input), &mut resources, &Component::default())
    }

    fn calculate_fuel(input: &[String]) -> usize {
        calculated_fuel_from_1_trln_ores(&parse_input(input))
    }

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
        let input = smapp_example();

        let mut expected: Receipt = HashMap::new();
        expected.insert(
            "A".into(),
            ReceiptLine {
                result: Component::new(10, "A"),
                ingredients: Component::from_vec(vec![(10, "ORE")]),
            },
        );
        expected.insert(
            "B".into(),
            ReceiptLine {
                result: Component::new(1, "B"),
                ingredients: Component::from_vec(vec![(1, "ORE")]),
            },
        );
        expected.insert(
            "C".into(),
            ReceiptLine {
                result: Component::new(1, "C"),
                ingredients: Component::from_vec(vec![(7, "A"), (1, "B")]),
            },
        );
        expected.insert(
            "D".into(),
            ReceiptLine {
                result: Component::new(1, "D"),
                ingredients: Component::from_vec(vec![(7, "A"), (1, "C")]),
            },
        );
        expected.insert(
            "E".into(),
            ReceiptLine {
                result: Component::new(1, "E"),
                ingredients: Component::from_vec(vec![(7, "A"), (1, "D")]),
            },
        );
        expected.insert(
            "FUEL".into(),
            ReceiptLine {
                result: Component::new(1, "FUEL"),
                ingredients: Component::from_vec(vec![(7, "A"), (1, "E")]),
            },
        );

        let parsed = parse_input(&input);

        assert_eq!(expected, parsed);
    }

    #[test]
    fn calculates_one_line() {
        assert_eq!(8, calculate(&str_to_string(&["8 ORE => 1 FUEL"])));
    }

    #[test]
    fn calculates_two_lines() {
        let input = str_to_string(&["8 A => 1 FUEL", "2 ORE => 1 A"]);
        assert_eq!(16, calculate(&input));
    }

    #[test]
    fn solves_small_example() {
        assert_eq!(31, calculate(&smapp_example()));
    }

    #[test]
    fn solves_1st_larger_example() {
        assert_eq!(13312, calculate(&first_larger_example()));
    }

    #[test]
    fn solves_2nd_larger_example() {
        assert_eq!(180697, calculate(&second_larger_example()));
    }

    #[test]
    fn solves_3rd_larger_example() {
        assert_eq!(2210736, calculate(&third_larger_example()));
    }

    #[test]
    fn calculates_fuel_1st_larger_example() {
        assert_eq!(82892753, calculate_fuel(&first_larger_example()));
    }

    #[test]
    fn calculates_fuel_2nd_larger_example() {
        assert_eq!(5586022, calculate_fuel(&second_larger_example()));
    }

    #[test]
    fn calculates_fuel_3rd_larger_example() {
        assert_eq!(460664, calculate_fuel(&third_larger_example()));
    }
}
