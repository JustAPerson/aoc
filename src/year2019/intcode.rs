use std::collections::VecDeque;

pub type Word = isize;

#[derive(Clone, Debug)]
pub struct Computer {
    data: Vec<Word>,
    input: VecDeque<Word>,
    output: Vec<Word>,
    pc: Word,
    rel_base: Word,
}

impl Computer {
    pub fn decode<T: AsRef<str>>(input: T) -> Vec<Word> {
        use std::str::FromStr;
        input
            .as_ref()
            .trim()
            .split(",")
            .map(Word::from_str)
            .collect::<Result<Vec<Word>, _>>()
            .unwrap()
    }

    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            input: VecDeque::new(),
            output: Vec::new(),
            pc: 0,
            rel_base: 0,
        }
    }

    pub fn with_program(code: &[Word], input: &[Word]) -> Self {
        let mut program = Computer::new();
        program.reset(code);
        program.set_input(input);
        program
    }

    pub fn run_program(code: &[Word], input: &[Word]) -> Vec<Word> {
        let mut program = Computer::new();
        program.reset(code);
        program.set_input(input);
        let done = program.exec();
        assert!(done);
        program.output
    }

    pub fn reset(&mut self, code: &[Word]) {
        self.data.clear();
        self.data.extend_from_slice(code);
        self.output.clear();
        self.pc = 0;
        self.rel_base = 0;
    }

    pub fn set_input(&mut self, input: &[Word]) {
        self.input = input.iter().cloned().collect();
    }

    pub fn get_input(&mut self) -> &mut VecDeque<Word> {
        &mut self.input
    }

    pub fn get_output(&mut self) -> &mut Vec<Word> {
        &mut self.output
    }

    pub fn exec(&mut self) -> bool {
        loop {
            let opcode = self.get_opcode();
            match opcode {
                Opcode::Add => {
                    *self.param_out(3) = self.param_in(1) + self.param_in(2);
                }
                Opcode::Mul => {
                    *self.param_out(3) = self.param_in(1) * self.param_in(2);
                }
                Opcode::In => {
                    if self.input.is_empty() {
                        // yield until more input
                        return false;
                    }
                    *self.param_out(1) = self.input.pop_front().unwrap()
                }
                Opcode::Out => {
                    let val = self.param_in(1);
                    self.output.push(val)
                }
                Opcode::JmpT => {
                    if self.param_in(1) != 0 {
                        debug_assert!(self.param_in(2) >= 0);
                        self.pc = self.param_in(2);
                        continue;
                    }
                }
                Opcode::JmpF => {
                    if self.param_in(1) == 0 {
                        debug_assert!(self.param_in(2) >= 0);
                        self.pc = self.param_in(2);
                        continue;
                    }
                }
                Opcode::Lt => *self.param_out(3) = (self.param_in(1) < self.param_in(2)) as Word,
                Opcode::Eq => *self.param_out(3) = (self.param_in(1) == self.param_in(2)) as Word,
                Opcode::SetRel => {
                    self.rel_base += self.param_in(1);
                }
                Opcode::End => {}
            }

            self.pc += opcode.len();

            if opcode == Opcode::End {
                return true;
            }
        }
    }

    fn read(&mut self, addr: Word) -> Word {
        let addr = addr as usize;
        if addr >= self.data.len() {
            self.data.resize(addr + 1, 0)
        }
        self.data[addr]
    }

    fn get_opcode(&self) -> Opcode {
        get_digits_base10(self.data[self.pc as usize], 0, 2).into()
    }

    fn get_param_mode(&self, offset: u32) -> Param {
        get_digits_base10(self.data[self.pc as usize], 2 - 1 + offset, 1).into()
    }

    fn param_out(&mut self, offset: u32) -> &mut Word {
        let param = self.read(self.pc + offset as Word);

        let addr = match self.get_param_mode(offset) {
            Param::Addr => param as usize,
            Param::Rel => (self.rel_base + param) as usize,
            Param::Imm => panic!("Invalid destination parameter"),
        };

        if addr >= self.data.len() {
            self.data.resize(addr + 1, 0)
        }
        &mut self.data[addr]
    }

    fn param_in(&mut self, offset: u32) -> Word {
        let param = self.read(self.pc + offset as Word);

        match self.get_param_mode(offset) {
            Param::Addr => self.read(param),
            Param::Imm => param,
            Param::Rel => self.read(self.rel_base + param),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Param {
    Addr,
    Imm,
    Rel,
}

impl From<Word> for Param {
    fn from(word: Word) -> Param {
        match word {
            0 => Param::Addr,
            1 => Param::Imm,
            2 => Param::Rel,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Opcode {
    Add,
    Mul,
    In,
    Out,
    JmpT,
    JmpF,
    Lt,
    Eq,
    SetRel,
    End,
}

impl Opcode {
    fn len(&self) -> Word {
        match *self {
            Opcode::Add | Opcode::Mul => 4,
            Opcode::In | Opcode::Out => 2,
            Opcode::JmpT | Opcode::JmpF => 3,
            Opcode::Lt | Opcode::Eq => 4,
            Opcode::SetRel => 2,
            Opcode::End => 1,
        }
    }
}

impl From<Word> for Opcode {
    fn from(word: Word) -> Opcode {
        match word {
            01 => Opcode::Add,
            02 => Opcode::Mul,
            03 => Opcode::In,
            04 => Opcode::Out,
            05 => Opcode::JmpT,
            06 => Opcode::JmpF,
            07 => Opcode::Lt,
            08 => Opcode::Eq,
            09 => Opcode::SetRel,
            99 => Opcode::End,
            op => panic!("invalid opcode: {}", op),
        }
    }
}

pub fn get_digits_base10(mut x: Word, offset: u32, width: u32) -> Word {
    let width = (10 as Word).pow(width);
    x /= (10 as Word).pow(offset);
    x - width * (x / width)
}

#[test]
fn test_get_digits_base10() {
    assert_eq!(get_digits_base10(12345, 0, 2), 45);
    assert_eq!(get_digits_base10(12345, 1, 2), 34);
    assert_eq!(get_digits_base10(12345, 2, 1), 3);
    assert_eq!(get_digits_base10(12345, 3, 1), 2);
    assert_eq!(get_digits_base10(12345, 4, 2), 1);
    assert_eq!(get_digits_base10(12345, 5, 1), 0);
}
