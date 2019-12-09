use crate::year2019::intcode::{Program, Word};
use std::str::FromStr;

pub fn run() {
    let code: Vec<Word> = std::fs::read_to_string("inputs/year2019/day09.txt")
        .unwrap()
        .trim()
        .split(",")
        .map(Word::from_str)
        .collect::<Result<_, _>>()
        .unwrap();

    println!("year2019 day09 part1 {}", part1(&code));
    println!("year2019 day09 part2 {}", part2(&code));
}

fn part1(code: &[Word]) -> Word {
    Program::run_program(code, &[1])[0]
}

fn part2(code: &[Word]) -> Word {
    Program::run_program(code, &[2])[0]
}

#[test]
fn test_part1() {
    fn test(list: &[Word]) -> Vec<Word> {
        let mut program = Program::new();
        program.reset(list);
        program.exec();
        program.get_output().to_vec()
    }

    let quine = &[
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    assert_eq!(test(quine), quine);

    let big = &[1102, 34915192, 34915192, 7, 4, 7, 99, 0];
    assert_eq!(test(big)[0].to_string().len(), 16);

    let mid = &[104, 1125899906842624, 99];
    assert_eq!(test(mid)[0], 1125899906842624);
}
