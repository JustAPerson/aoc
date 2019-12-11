pub fn run() {
    let grid = std::fs::read_to_string("inputs/year2019/day10.txt")
        .map(|s| Grid::from_str(&s))
        .unwrap();

    println!("year2019 day10 part1 {}", part1(&grid));
    println!("year2019 day10 part2 {}", part2(&grid));
}

fn part1(grid: &Grid) -> usize {
    grid.best_station().1
}

fn part2(grid: &Grid) -> isize {
    let (x, y) = grid.vaporization_order(grid.best_station().0)[199];
    x * 100 + y
}

struct Grid {
    data: Vec<Vec<bool>>,
    asteroids: Vec<Pos>,
    width: isize,
    height: isize,
}

impl Grid {
    fn from_str(input: &str) -> Grid {
        let width = input.lines().map(str::len).max().unwrap();
        let height = input.lines().count();
        assert!(input.lines().all(|t| t.len() == width));

        Grid {
            data: input
                .lines()
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect(),
            asteroids: input
                .lines()
                .enumerate()
                .map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|(_, c)| *c == '#')
                        .map(move |(x, _)| (x as isize, y as isize))
                })
                .flatten()
                .collect(),
            width: width as isize,
            height: height as isize,
        }
    }

    fn contains(&self, pos: Pos) -> bool {
        pos.0 >= 0 && pos.0 < self.width && pos.1 >= 0 && pos.1 < self.height
    }

    /// Count number of asteroids visible from this starting point
    fn visibility(&self, start: Pos) -> usize {
        let mut visible = self.data.clone();
        visible[start.1 as usize][start.0 as usize] = false;

        for &asteroid in &self.asteroids {
            if asteroid == start {
                continue;
            }
            if !visible[asteroid.1 as usize][asteroid.0 as usize] {
                continue;
            }

            let mut diff = sub(asteroid, start);
            let mut curr = asteroid;

            // perform gcd so if our diff is 3,3, we don't skip over something at 4,4
            let gcd = gcd(diff.0, diff.1).abs();
            diff.0 /= gcd;
            diff.1 /= gcd;

            curr = add(curr, diff);
            while self.contains(curr) {
                visible[curr.1 as usize][curr.0 as usize] = false;
                curr = add(curr, diff);
            }
        }
        visible.iter().flatten().filter(|p| **p).count()
    }

    /// Find the asteroid that can view the most other asteroids
    fn best_station(&self) -> (Pos, usize) {
        self.asteroids
            .iter()
            .map(|&pos| (pos, self.visibility(pos)))
            .max_by_key(|&(_, n)| n)
            .unwrap()
    }

    /// Calculate lists of asteroids at each heading from start
    fn angles(&self, start: Pos) -> Vec<(TotalF32, Vec<Pos>)> {
        let mut angles = std::collections::HashMap::<_, Vec<Pos>>::new();
        for &asteroid in &self.asteroids {
            if asteroid == start {
                continue;
            }

            let dx = (asteroid.0 - start.0) as f32;
            let dy = (asteroid.1 - start.1) as f32;
            // atan calculates relative to X axis, we want relative to Y axis
            let angle = (-dx).atan2(dy);
            let list = angles.entry(TotalF32(angle)).or_default();
            list.push(asteroid);
        }
        let mut output = angles.into_iter().collect::<Vec<_>>();
        output.sort_by_key(|&(a, _)| a);
        output
            .iter_mut() // sort each list by negative distance so closest is first to pop()
            .for_each(|(_, list)| list.sort_by_key(|p| -dist(*p, start)));
        output
    }

    /// Determine what order to destroy asteroids based on given start
    fn vaporization_order(&self, start: Pos) -> Vec<Pos> {
        let mut angles = self.angles(start);
        let mut output = Vec::new();

        let mut done = false;
        while !done {
            done = true;

            for (_, in_line) in &mut angles {
                if let Some(closest) = in_line.pop() {
                    done = false;
                    output.push(closest);
                }
            }
        }

        output
    }
}

type Pos = (isize, isize);
fn add(lhs: Pos, rhs: Pos) -> Pos {
    (lhs.0 + rhs.0, lhs.1 + rhs.1)
}
fn sub(lhs: Pos, rhs: Pos) -> Pos {
    (lhs.0 - rhs.0, lhs.1 - rhs.1)
}
fn dist(to: Pos, from: Pos) -> isize {
    let (dx, dy) = sub(to, from);
    dx.abs() + dy.abs()
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TotalF32(f32);
impl TotalF32 {
    fn normalize(&self) -> f32 {
        if self.0.is_normal() {
            self.0
        } else if self.0.abs() == 0.0 {
            0.0
        } else {
            panic!();
        }
    }
}
impl std::cmp::Ord for TotalF32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        PartialOrd::partial_cmp(&self.0, &other.0).unwrap()
    }
}
impl std::cmp::Eq for TotalF32 {}
impl std::hash::Hash for TotalF32 {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        hasher.write_u32(self.normalize() as u32)
    }
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(5, 0), 5);
    assert_eq!(gcd(5, 10), 5);
    assert_eq!(gcd(5, 3), 1);
    assert_eq!(gcd(5, -10), 5);
    assert_eq!(gcd(-5, -10), -5);
    assert_eq!(gcd(-5, -10), -5);
}
#[test]
fn test_part1() {
    assert_eq!(
        Grid::from_str(
            ".#..#\n\
             .....\n\
             #####\n\
             ....#\n\
             ...##",
        )
        .best_station(),
        ((3, 4), 8)
    );
    assert_eq!(
        Grid::from_str(
            "......#.#.\n\
             #..#.#....\n\
             ..#######.\n\
             .#.#.###..\n\
             .#..#.....\n\
             ..#....#.#\n\
             #..#....#.\n\
             .##.#..###\n\
             ##...#..#.\n\
             .#....####",
        )
        .best_station(),
        ((5, 8), 33)
    );
    assert_eq!(
        Grid::from_str(
            "#.#...#.#.\n\
             .###....#.\n\
             .#....#...\n\
             ##.#.#.#.#\n\
             ....#.#.#.\n\
             .##..###.#\n\
             ..#...##..\n\
             ..##....##\n\
             ......#...\n\
             .####.###.",
        )
        .best_station(),
        ((1, 2), 35)
    );
    assert_eq!(
        Grid::from_str(
            ".#..#..###\n\
             ####.###.#\n\
             ....###.#.\n\
             ..###.##.#\n\
             ##.##.#.#.\n\
             ....###..#\n\
             ..#.#..#.#\n\
             #..#.#.###\n\
             .##...##.#\n\
             .....#.#..",
        )
        .best_station(),
        ((6, 3), 41)
    );
    assert_eq!(
        Grid::from_str(
            ".#..##.###...#######\n\
             ##.############..##.\n\
             .#.######.########.#\n\
             .###.#######.####.#.\n\
             #####.##.#.##.###.##\n\
             ..#####..#.#########\n\
             ####################\n\
             #.####....###.#.#.##\n\
             ##.#################\n\
             #####.##.###..####..\n\
             ..######..##.#######\n\
             ####.##.####...##..#\n\
             .#####..#.######.###\n\
             ##...#.##########...\n\
             #.##########.#######\n\
             .####.#.###.###.#.##\n\
             ....##.##.###..#####\n\
             .#.#.###########.###\n\
             #.#.#.#####.####.###\n\
             ###.##.####.##.#..##",
        )
        .best_station(),
        ((11, 13), 210)
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(&Grid::from_str(
            ".#..##.###...#######\n\
             ##.############..##.\n\
             .#.######.########.#\n\
             .###.#######.####.#.\n\
             #####.##.#.##.###.##\n\
             ..#####..#.#########\n\
             ####################\n\
             #.####....###.#.#.##\n\
             ##.#################\n\
             #####.##.###..####..\n\
             ..######..##.#######\n\
             ####.##.####...##..#\n\
             .#####..#.######.###\n\
             ##...#.##########...\n\
             #.##########.#######\n\
             .####.#.###.###.#.##\n\
             ....##.##.###..#####\n\
             .#.#.###########.###\n\
             #.#.#.#####.####.###\n\
             ###.##.####.##.#..##",
        )),
        802
    );
}
