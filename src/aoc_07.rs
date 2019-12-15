use crate::opcode_computer::Program;

pub fn aoc_07_01() -> u32 {
    0
}
pub fn aoc_07_02() -> u32 {
    0
}

fn amplifier(phase_settings: &Vec<u32>, program: &Vec<isize>) -> isize {
    let mut input = 0;
    for phase in phase_settings {
        let mut computer = Program::new(
            program.clone(),
            Some(vec![phase.clone() as isize, input.clone()]),
        );
        computer.run();
        input = computer.output[0];
    }

    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_first_example_program() {
        let phase_settings = vec![4, 3, 2, 1, 0];
        let program = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let max_signal = 43210;

        let signal = amplifier(&phase_settings, &program);
        assert_eq!(signal, max_signal);
    }

    #[test]
    fn runs_second_example_program() {
        let phase_settings = vec![0, 1, 2, 3, 4];
        let program = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let max_signal = 54321;

        let signal = amplifier(&phase_settings, &program);
        assert_eq!(signal, max_signal);
    }

    #[test]
    fn runs_third_example_program() {
        let phase_settings = vec![1, 0, 4, 3, 2];
        let program = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let max_signal = 65210;

        let signal = amplifier(&phase_settings, &program);
        assert_eq!(signal, max_signal);
    }
}
