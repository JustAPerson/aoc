pub fn run() {
    let input: Vec<Code> = std::fs::read_to_string("inputs/year2019/day04.txt")
        .unwrap()
        .trim()
        .split("-")
        .map(Code::from_str)
        .collect::<Vec<_>>();
    let a = input[0];
    let b = input[1];
    println!("year2019 day04 part1 {}", part1(a, b));
    println!("year2019 day04 part2 {}", part2(a, b));
}

#[derive(Clone, Copy, Debug)]
pub struct Code {
    digits: [u8; 6],
}

impl Code {
    fn from_str(s: &str) -> Code {
        assert!(s.parse::<usize>().is_ok());
        let mut i = 6;
        let mut code = Code { digits: [0; 6] };
        s.chars()
            .rev()
            .map(|d| {
                i -= 1;
                code.digits[i] = (d as u8) - ('0' as u8);
            })
            .count();
        while i > 0 {
            i -= 1;
            code.digits[i] = 0;
        }
        code
    }

    fn is_valid_part1(&self) -> bool {
        self.digits.windows(2).all(|two| two[0] <= two[1])
            && self.digits.windows(2).any(|two| two[0] == two[1])
    }

    fn is_valid_part2(&self, storage: &mut Vec<u8>) -> bool {
        storage.clear();
        storage.push(1);
        let mut last = self.digits[0];
        let mut i = 1;
        while i < 6 {
            if self.digits[i] == last {
                *storage.last_mut().unwrap() += 1;
            } else {
                last = self.digits[i];
                storage.push(1);
            }
            i += 1;
        }
        storage.iter().any(|x| *x == 2)
    }

    fn increment(&mut self) {
        let mut i = 5;
        loop {
            self.digits[i] += 1;
            if self.digits[i] < 10 {
                break;
            }
            self.digits[i] = 0;
            i -= 1;
        }
    }
}

fn part1(mut a: Code, b: Code) -> usize {
    let mut valid_codes = 0;
    while a.digits < b.digits {
        if a.is_valid_part1() {
            valid_codes += 1;
        }
        a.increment();
    }
    valid_codes
}

fn part2(mut a: Code, b: Code) -> usize {
    let mut valid_codes = 0;
    let mut storage = Vec::with_capacity(6);
    while a.digits < b.digits {
        if a.is_valid_part1() && a.is_valid_part2(&mut storage) {
            valid_codes += 1;
        }
        a.increment();
    }
    valid_codes
}
