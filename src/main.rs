mod aoc_01;
mod aoc_02;
mod aoc_03;
mod aoc_04;
mod aoc_05;
mod aoc_06;
mod aoc_07;
mod aoc_08;
mod aoc_09;
mod aoc_10;
mod opcode_computer;

fn main() {
    println!("01 / 01: {}", aoc_01::aoc_01_01());
    println!("01 / 02: {}", aoc_01::aoc_01_02());
    println!("02 / 01: {}", aoc_02::aoc_02_01());
    println!("02 / 02: {}", aoc_02::aoc_02_02());
    println!("03 / 01: {}", aoc_03::aoc_03_01());
    println!("03 / 02: {}", aoc_03::aoc_03_02());
    println!("04 / 01: {}", aoc_04::aoc_04_01());
    println!("04 / 02: {}", aoc_04::aoc_04_02());
    println!("05 / 01: {}", aoc_05::aoc_05_01());
    println!("05 / 02: {}", aoc_05::aoc_05_02());
    println!("06 / 01: {}", aoc_06::aoc_06_01());
    println!("06 / 02: {}", aoc_06::aoc_06_02());
    println!("07 / 01: {}", aoc_07::aoc_07_01());
    println!("07 / 02: {}", aoc_07::aoc_07_02());
    println!("08 / 01: {}", aoc_08::aoc_08_01());
    println!("08 / 02:");
    println!("{:#?}", aoc_08::aoc_08_02());
    println!("09 / 01: {}", aoc_09::aoc_09_01());
    println!("09 / 02: {}", aoc_09::aoc_09_02());
    println!("10 / 01: {:?}", aoc_10::aoc_10_01());
}
