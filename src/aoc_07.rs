use {crate::opcode_computer::OpcodeComputer, std::fs};

fn read_and_parse() -> Vec<isize> {
    let content = fs::read_to_string("input-07.txt").unwrap();
    content
        .trim()
        .split(",")
        .map(|string| isize::from_str_radix(string.as_ref(), 10).unwrap())
        .collect()
}

pub fn aoc_07_01() -> isize {
    let program = read_and_parse();
    let (signal, _) = find_max_signal(&program);

    signal
}
pub fn aoc_07_02() -> u32 {
    0
}

fn find_max_signal(program: &Vec<isize>) -> (isize, Vec<isize>) {
    let mut optimal_phases = vec![0, 1, 2, 3, 4];
    let mut signal = 0;

    let mut all_phases = swap_4(&optimal_phases);
    all_phases.insert(0, optimal_phases.clone());

    for phases in all_phases.clone() {
        let new_signal = amplifier(&phases, &program);
        if new_signal > signal {
            signal = new_signal;
            optimal_phases = phases;
        }
    }

    (signal, optimal_phases)
}

fn swap_4(seq: &Vec<isize>) -> Vec<Vec<isize>> {
    let mut seqs = vec![];

    let mut new_seqs = swap_3(seq);
    let seq = new_seqs.last().unwrap().clone();
    seqs.append(&mut new_seqs);

    let mut seq = seq.clone();
    seq.swap(0, 4);
    seqs.push(seq.clone());
    let mut new_seqs = swap_3(&seq);
    let seq = new_seqs.last().unwrap().clone();
    seqs.append(&mut new_seqs);

    let mut seq = seq.clone();
    seq.swap(0, 4);
    seqs.push(seq.clone());
    let mut new_seqs = swap_3(&seq);
    let seq = new_seqs.last().unwrap().clone();
    seqs.append(&mut new_seqs);

    let mut seq = seq.clone();
    seq.swap(0, 4);
    seqs.push(seq.clone());
    let mut new_seqs = swap_3(&seq);
    let seq = new_seqs.last().unwrap().clone();
    seqs.append(&mut new_seqs);

    let mut seq = seq.clone();
    seq.swap(0, 4);
    seqs.push(seq.clone());
    let mut new_seqs = swap_3(&seq);
    seqs.append(&mut new_seqs);

    seqs
}

fn swap_3(seq: &Vec<isize>) -> Vec<Vec<isize>> {
    let mut seqs = vec![];

    let mut new_seqs = swap_2(seq);
    let seq = new_seqs.last().unwrap().clone();
    seqs.append(&mut new_seqs);

    let mut seq = seq.clone();
    seq.swap(1, 3);
    seqs.push(seq.clone());
    let mut new_seqs = swap_2(&seq);
    let seq = new_seqs.last().unwrap().clone();
    seqs.append(&mut new_seqs);

    let mut seq = seq.clone();
    seq.swap(1, 3);
    seqs.push(seq.clone());
    let mut new_seqs = swap_2(&seq);
    let seq = new_seqs.last().unwrap().clone();
    seqs.append(&mut new_seqs);

    let mut seq = seq.clone();
    seq.swap(0, 3);
    seqs.push(seq.clone());
    let mut new_seqs = swap_2(&seq);
    seqs.append(&mut new_seqs);

    seqs
}

fn swap_2(seq: &Vec<isize>) -> Vec<Vec<isize>> {
    let mut seqs = vec![];

    let mut seq = seq.clone();
    seq.swap(0, 1);
    seqs.push(seq.clone());

    let mut seq = seq.clone();
    seq.swap(1, 2);
    seqs.push(seq.clone());

    let mut seq = seq.clone();
    seq.swap(0, 1);
    seqs.push(seq.clone());

    let mut seq = seq.clone();
    seq.swap(1, 2);
    seqs.push(seq.clone());

    let mut seq = seq.clone();
    seq.swap(0, 1);
    seqs.push(seq);

    seqs
}

fn amplifier(phase_settings: &Vec<isize>, program: &Vec<isize>) -> isize {
    let mut input = 0;
    for phase in phase_settings {
        let mut computer = OpcodeComputer::new(program.clone());
        computer.add_input(&phase).add_input(&input).run();
        input = computer.get_output().unwrap();
    }

    input
}

fn looped_amplifier(phase_settings: &Vec<isize>, program: &Vec<isize>) -> isize {
    let mut comps = vec![];

    // Setup
    for phase in phase_settings {
        let mut comp = OpcodeComputer::new(program.clone());
        comp.add_input(&phase);
        comps.push(comp);
    }

    // Run comps
    let mut index: usize = 0;
    let mut input: isize = 0;
    while !comps.iter().all(|comp| comp.halted()) {
        if index >= phase_settings.len() {
            index = 0;
        }
        comps[index].add_input(&input).run();
        input = comps[index].get_output().expect(&format!("There's no output for: {:?}", comps[index]));
        index += 1;
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

    #[test]
    fn finds_max_signal_for_the_first_example() {
        let phase_settings = vec![4, 3, 2, 1, 0];
        let program = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let max_signal = 43210;

        let found = find_max_signal(&program);
        assert_eq!(found, (max_signal, phase_settings));
    }

    #[test]
    fn finds_max_signal_for_the_second_example() {
        let phase_settings = vec![0, 1, 2, 3, 4];
        let program = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let max_signal = 54321;

        let found = find_max_signal(&program);
        assert_eq!(found, (max_signal, phase_settings));
    }

    #[test]
    fn finds_max_signal_for_the_third_example() {
        let phase_settings = vec![1, 0, 4, 3, 2];
        let program = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let max_signal = 65210;

        let found = find_max_signal(&program);
        assert_eq!(found, (max_signal, phase_settings));
    }

    #[test]
    fn looped_amp_runs_first_example_program() {
        let phase_settings = vec![9, 8, 7, 6, 5];
        let program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let max_signal = 139629729;

        let signal = looped_amplifier(&phase_settings, &program);
        assert_eq!(signal, max_signal);
    }
}
