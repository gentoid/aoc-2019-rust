fn input() -> [i32; 2] {
    [357253, 892942]
}

pub fn aoc_04_01() -> i32 {
    let input = input();
    BruteForce::new(input[0], input[1]).run()
}

type Tup6 = [i32; 6];

#[derive(Debug)]
struct BruteForce {
    current: Tup6,
    left_to_check: i32,
    possible_passwords_counter: i32,
}

impl BruteForce {
    pub fn new(from: i32, till: i32) -> Self {
        Self {
            current: to_digits(from),
            left_to_check: till - from,
            possible_passwords_counter: 0,
        }
    }

    pub fn run(&mut self) -> i32 {
        while self.left_to_check >= 0 {
            if self.valid() {
                self.possible_passwords_counter += 1;
            }
            self.next();
        }

        self.possible_passwords_counter
    }

    fn next(&mut self) {
        let max = self.current.len() - 1;
        self.current[max] += 1;
        for position in 0..=max {
            let rev_position = max - position;
            if self.current[rev_position] > 9 {
                self.current[rev_position] = 0;
                self.current[rev_position - 1] += 1;
            }
        }
        self.left_to_check -= 1;
    }

    fn valid(&self) -> bool {
        self.digits_not_decrease() && self.has_two_adjacent_digits()
    }

    fn digits_not_decrease(&self) -> bool {
        for posision in 0..(self.current.len() - 1) {
            if self.current[posision] > self.current[posision + 1] {
                return false;
            }
        }
        true
    }

    fn has_two_adjacent_digits(&self) -> bool {
        for position in 0..(self.current.len() - 1) {
            if self.current[position] == self.current[position + 1] {
                return true;
            }
        }
        false
    }
}

fn to_digits(num: i32) -> Tup6 {
    let mut num = num;
    let mut digits = init_digits();

    for position in 0..digits.len() {
        digits[digits.len() - 1 - position] = num % 10;
        num /= 10;
    }

    digits
}

fn init_digits() -> Tup6 {
    [0; 6]
}
