pub fn run() {
    let input = std::fs::read_to_string("inputs/year2019/day08.txt").unwrap();
    let digits = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();
    let layers: Vec<&[u8]> = digits.chunks(25 * 6).collect();
    println!("year2019 day08 part1 {}", part1(&layers));
    println!("year2019 day08 part2\n{}", part2(&layers));
}

fn part1(layers: &[&[u8]]) -> usize {
    let safest = layers
        .iter()
        .min_by_key(|t| t.iter().filter(|d| **d == 0).count())
        .unwrap();
    let ones = safest.iter().filter(|d| **d == 1).count();
    let twos = safest.iter().filter(|d| **d == 2).count();
    ones * twos
}

fn part2(layers: &[&[u8]]) -> String {
    let mut buffer = [2; 25 * 6];
    for layer in layers {
        for (dest, input) in buffer.iter_mut().zip(layer.iter()) {
            if *dest == 2 && *input < 2 {
                *dest = *input;
            }
        }
    }

    let mut output = String::new();
    for line in buffer.chunks(25) {
        output.push_str("    ");
        for digit in line {
            output.push(if *digit == 0 { ' ' } else { 'X' });
        }
        output.push('\n');
    }

    output
}
