use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = std::fs::read_to_string("inputs/year2019/day06.txt").unwrap();
    let pairs: Vec<[&str; 2]> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(")");
            [parts.next().unwrap(), parts.next().unwrap()]
        })
        .collect();
    let orbits = pairs
        .iter()
        .map(|&[center, satellite]| (satellite, center))
        .collect::<HashMap<_, _>>();
    println!("year2019 day06 part1 {}", part1(&pairs, &orbits));
    println!("year2019 day06 part2 {}", part2(&orbits));
}

fn distance<'a>(
    satellite: &'a str,
    orbits: &HashMap<&'a str, &'a str>,
    distances: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(d) = distances.get(satellite) {
        *d
    } else {
        let d = distance(orbits[satellite], orbits, distances) + 1;
        distances.insert(satellite, d);
        d
    }
}

fn part1(pairs: &[[&str; 2]], orbits: &HashMap<&str, &str>) -> usize {
    let mut distances = HashMap::new();
    distances.insert("COM", 0);
    pairs
        .iter()
        .flatten()
        .collect::<HashSet<_>>()
        .iter()
        .map(|object| distance(object, &orbits, &mut distances))
        .sum()
}

fn path<'a>(satellite: &'a str, orbits: &HashMap<&'a str, &'a str>) -> Vec<&'a str> {
    if satellite == "COM" {
        return vec!["COM"];
    }
    let mut p = path(orbits[satellite], orbits);
    p.push(satellite);
    p
}

fn part2(orbits: &HashMap<&str, &str>) -> usize {
    let path_san = path("SAN", &orbits);
    let path_you = path("YOU", &orbits);

    let common = path_san
        .iter()
        .zip(path_you.iter())
        .take_while(|(a, b)| a == b)
        .count();
    let common_to_san = path_san.len() - common - 1;
    let common_to_you = path_you.len() - common - 1;
    common_to_san + common_to_you
}
