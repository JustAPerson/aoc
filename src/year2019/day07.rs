use std::str::FromStr;
use crate::year2019::intcode::{Computer, Word};

pub fn run() {
    let code: Vec<Word> = std::fs::read_to_string("inputs/year2019/day07.txt")
        .unwrap()
        .trim()
        .split(",")
        .map(Word::from_str)
        .collect::<Result<_, _>>()
        .unwrap();

    println!("year2019 day07 part1 {}", part1(&code));
    println!("year2019 day07 part2 {}", part2(&code));
}

fn permutations<T: Clone>(choices: &[T]) -> Vec<Vec<T>> {
    if choices.len() == 1 {
        vec![vec![choices[0].clone()]]
    } else {
        (0..choices.len())
            .into_iter()
            .map(|i| {
                let mut new_choices = choices.to_vec();
                new_choices.remove(i);
                let mut p = permutations(&new_choices);
                p.iter_mut().for_each(|p| p.push(choices[i].clone()));
                p
            })
            .flatten()
            .collect()
    }
}

fn part1(code: &[Word]) -> Word {
    permutations(&[0 as Word, 1, 2, 3, 4])
        .into_iter()
        .map(|phases| {
            phases.into_iter().fold(0 as Word, |input, phase| {
                Computer::run_program(code, &[phase, input])[0]
            })
        })
        .max()
        .unwrap()
}

fn part2(code: &[Word]) -> Word {
    permutations(&[5 as Word, 6, 7, 8, 9])
        .into_iter()
        .map(|phases| {
            let mut computers = phases
                .into_iter()
                .map(|phase| Computer::with_program(code, &[phase]))
                .collect::<Vec<_>>();
            let mut input = 0;
            loop {
                for i in 0..5 {
                    computers[i].get_input().push_back(input);
                    let done = computers[i].exec();
                    input = *computers[i].get_output().last().unwrap();
                    if i == 4 && done {
                        return input;
                    }
                }
            }
        })
        .max()
        .unwrap()
}

#[test]
fn test_part1() {
    assert_eq!(
        part1(&[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]),
        43210
    );
    assert_eq!(
        part1(&[
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0
        ]),
        54321
    );
    assert_eq!(
        part1(&[
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
        ]),
        65210
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(&[
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5
        ]),
        139629729
    );
    assert_eq!(
        part2(&[
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
        ]),
        18216
    );
}
