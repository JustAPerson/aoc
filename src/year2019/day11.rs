use crate::year2019::intcode::{Computer, Word};

pub fn run() {
    let code = std::fs::read_to_string("inputs/year2019/day11.txt")
        .map(Computer::decode)
        .unwrap();

    println!("year2019 day11 part1 {}", part1(&code));
    println!("year2019 day11 part2\n{}", part2(&code));
}

type Pos = (isize, isize);
static DIRS: [Pos; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn paint(code: &[Word], start: Word) -> std::collections::HashMap<Pos, Word> {
    let mut computer = Computer::with_program(code, &[]);
    let mut hull = std::collections::HashMap::new();
    hull.insert((0, 0), start);

    let mut pos = (0, 0);
    let mut dir = 0isize;
    let mut done = false;
    while !done {
        let current: &mut Word = hull.entry(pos).or_insert(0);
        computer.get_input().push_back(*current);
        done = computer.exec();

        let turn = computer.get_output().pop().unwrap();
        let paint = computer.get_output().pop().unwrap();
        *current = paint;

        dir = (4 + dir + if turn == 1 { 1 } else { -1 }) % 4;
        let (dx, dy) = DIRS[dir as usize];
        pos.0 += dx;
        pos.1 += dy;
    }

    hull
}

fn part1(code: &[Word]) -> usize {
    let hull = paint(code, 0);
    hull.len()
}

fn part2(code: &[Word]) -> String {
    let hull = paint(code, 1);

    let min_x = *hull.keys().map(|(x, _)| x).min().unwrap();
    let max_x = *hull.keys().map(|(x, _)| x).max().unwrap();
    let min_y = *hull.keys().map(|(_, y)| y).min().unwrap();
    let max_y = *hull.keys().map(|(_, y)| y).max().unwrap();

    let mut output = String::new();
    for y in (min_y..(max_y + 1)).rev() {
        for x in min_x..(max_x + 1) {
            output.push(if hull.get(&(x, y)).cloned().unwrap_or(0) == 1 {
                'X'
            } else {
                ' '
            })
        }
        output.push('\n')
    }

    output
}
