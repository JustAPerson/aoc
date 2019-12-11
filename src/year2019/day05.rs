use crate::year2019::intcode::{Computer, Word};

pub fn run() {
    let code = std::fs::read_to_string("inputs/year2019/day05.txt")
        .map(Computer::decode)
        .unwrap();

    let answer1 = part1(&code);
    let answer2 = part2(&code);
    println!("year2019 day05 part1 {}", answer1);
    println!("year2019 day05 part2 {}", answer2);
}

fn part1(input: &Vec<Word>) -> Word {
    let mut computer = Computer::new();
    computer.reset(input);
    computer.set_input(&[1]);
    computer.exec();

    let (last, others) = computer.get_output().split_last().unwrap();
    assert!(others.iter().all(|t| *t == 0));

    *last
}

fn part2(input: &Vec<Word>) -> Word {
    let mut computer = Computer::new();
    computer.reset(input);
    computer.set_input(&[5]);
    computer.exec();

    let output = computer.get_output();
    assert_eq!(output.len(), 1);

    output[0]
}

#[test]
fn test_part2() {
    fn test(input: Word, list: &[Word]) -> Word {
        let mut computer = Computer::new();
        computer.reset(list);
        computer.set_input(&[input]);
        computer.exec();
        *computer.get_output().last().unwrap()
    }
    let pos_eq_8 = &[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    assert_eq!(test(8, pos_eq_8), 1);
    assert_eq!(test(7, pos_eq_8), 0);

    let pos_lt_8 = &[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    assert_eq!(test(7, pos_lt_8), 1);
    assert_eq!(test(8, pos_lt_8), 0);

    let imm_eq_8 = &[3, 3, 1108, -1, 8, 3, 4, 3, 99];
    assert_eq!(test(8, imm_eq_8), 1);
    assert_eq!(test(9, imm_eq_8), 0);

    let imm_lt_8 = &[3, 3, 1107, -1, 8, 3, 4, 3, 99];
    assert_eq!(test(7, imm_lt_8), 1);
    assert_eq!(test(8, imm_lt_8), 0);

    let pos_eq_0 = &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    assert_eq!(test(0, pos_eq_0), 0);
    assert_eq!(test(1, pos_eq_0), 1);

    let imm_eq_0 = &[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
    assert_eq!(test(0, imm_eq_0), 0);
    assert_eq!(test(1, imm_eq_0), 1);

    let cmp_8 = &[
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];
    assert_eq!(test(7, cmp_8), 999);
    assert_eq!(test(8, cmp_8), 1000);
    assert_eq!(test(9, cmp_8), 1001);
}
