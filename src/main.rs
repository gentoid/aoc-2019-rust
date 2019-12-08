mod aoc_01;
mod aoc_02;
mod aoc_03;
mod aoc_04;
mod aoc_05;
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
}
