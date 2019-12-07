pub fn run() {
    let input: Vec<usize> = std::fs::read_to_string("inputs/year2019/day02.txt")
        .unwrap()
        .trim()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();
    let answer1 = part1(&input);
    let answer2 = part2(&input);
    println!("year2019 day02 part1 {}", answer1);
    println!("year2019 day02 part2 {}", answer2);
}

fn part1(input: &[usize]) -> usize {
    let mut program = input.to_vec();
    program[1] = 12;
    program[2] = 02;
    execute(&mut program);
    program[0]
}

fn part2(input: &[usize]) -> usize {
    let mut program = input.to_vec();

    for i in 0..100 {
        for j in 0..100 {
            program.copy_from_slice(input);

            program[1] = i;
            program[2] = j;

            execute(&mut program);

            if program[0] == 19690720 {
                return 100 * i + j;
            }
        }
    }

    unreachable!();
}

fn execute(program: &mut [usize]) {
    let mut i = 0;
    while i < program.len() {
        if program[i] == 99 {
            break;
        }

        match &program[i..i + 4] {
            &[1, src1, src2, dst] => program[dst] = program[src1] + program[src2],
            &[2, src1, src2, dst] => program[dst] = program[src1] * program[src2],
            _ => panic!(),
        }
        i += 4;
    }
}

#[test]
fn test_execute() {
    fn test(input: &[usize], output: &[usize]) {
        let mut input = input.to_vec();
        execute(&mut input);
        assert_eq!(input, output);
    }

    test(&[1, 0, 0, 0, 99], &[2, 0, 0, 0, 99]);
    test(&[2, 3, 0, 3, 99], &[2, 3, 0, 6, 99]);
    test(&[2, 4, 4, 5, 99, 0], &[2, 4, 4, 5, 99, 9801]);
    test(
        &[1, 1, 1, 4, 99, 5, 6, 0, 99],
        &[30, 1, 1, 4, 2, 5, 6, 0, 99],
    );
}
