pub fn run() {
    let input = std::fs::read_to_string("inputs/year2019/day03.txt")
        .map(parse)
        .unwrap();

    let answer1 = part1(&input);
    let answer2 = part2(&input);
    println!("year2019 day03 part1 {}", answer1);
    println!("year2019 day03 part2 {}", answer2);
}

fn parse(input: impl AsRef<str>) -> Vec<Vec<Segment>> {
    input
        .as_ref()
        .lines()
        .map(|line| line.split(",").map(Segment::from_str).collect())
        .collect()
}

#[derive(Debug)]
struct Segment {
    direction: (isize, isize),
    distance: usize,
}

impl Segment {
    fn from_str(input: &str) -> Segment {
        let (head, tail) = input.split_at(1);
        Segment {
            distance: tail.parse().unwrap(),
            direction: match head {
                "L" => (-1, 0),
                "R" => (1, 0),
                "U" => (0, 1),
                "D" => (0, -1),
                _ => panic!(),
            },
        }
    }
}

struct Intersection {
    manhattan: isize,
    steps: isize,
}

fn find_intersections(input: &Vec<Vec<Segment>>) -> Vec<Intersection> {
    let mut map = std::collections::HashMap::new();
    let mut intersections = Vec::new();

    for (index, path) in input.iter().enumerate() {
        let mut pos = (0, 0);
        let mut steps = 0;
        for Segment {
            direction,
            distance,
        } in path
        {
            for _ in 0..*distance {
                pos.0 += direction.0;
                pos.1 += direction.1;
                steps += 1;
                let exists = map.insert(pos, (index, steps));
                match exists {
                    Some((other_index, other_steps)) if index != other_index => {
                        intersections.push(Intersection {
                            manhattan: pos.0.abs() + pos.1.abs(),
                            steps: steps + other_steps,
                        })
                    }
                    _ => {}
                }
            }
        }
    }

    intersections
}
fn part1(input: &Vec<Vec<Segment>>) -> isize {
    find_intersections(input)
        .iter()
        .map(|i| i.manhattan)
        .min()
        .unwrap()
}

fn part2(input: &Vec<Vec<Segment>>) -> isize {
    find_intersections(input)
        .iter()
        .map(|i| i.steps)
        .min()
        .unwrap()
}

#[test]
fn test_part1() {
    fn test(input: String, expected: isize) {
        assert_eq!(part1(&parse(input)), expected);
    }
    test(
        "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
         U62,R66,U55,R34,D71,R55,D58,R83"
            .to_owned(),
        159,
    );
    test(
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
         U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            .to_owned(),
        135,
    );
}

#[test]
fn test_part2() {
    fn test(input: String, expected: isize) {
        assert_eq!(part2(&parse(input)), expected);
    }
    test(
        "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
         U62,R66,U55,R34,D71,R55,D58,R83"
            .to_owned(),
        610,
    );
    test(
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
         U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            .to_owned(),
        410,
    );
}
