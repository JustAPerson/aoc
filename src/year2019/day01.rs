pub fn run() {
    let input: Vec<u64> = std::fs::read_to_string("inputs/year2019/day01.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let answer1 = part1(&input);
    let answer2 = part2(&input);
    println!("year2019 day01 part1 {}", answer1);
    println!("year2019 day01 part2 {}", answer2);
}

fn calc_fuel(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

fn part1(input: &[u64]) -> u64 {
    input.iter().cloned().map(calc_fuel).sum()
}

fn extrapolate_fuel(mass: u64) -> u64 {
    let mut unaccounted = mass;
    let mut total = 0;
    while unaccounted > 0 {
        let fuel = calc_fuel(unaccounted);

        total += fuel;
        unaccounted = fuel;
    }

    total
}

fn part2(input: &[u64]) -> u64 {
    input.iter().cloned().map(extrapolate_fuel).sum()
}

#[test]
fn test_extrapolate_fuel() {
    assert_eq!(extrapolate_fuel(1969), 966);
    assert_eq!(extrapolate_fuel(100756), 50346);
}
