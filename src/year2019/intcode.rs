pub type Word = isize;

pub struct Program {
    data: Vec<Word>,
    input: Vec<Word>,
    output: Vec<Word>,
    pc: usize,
}

impl Program {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            input: Vec::new(),
            output: Vec::new(),
            pc: 0,
        }
    }

    pub fn reset(&mut self, source: &[Word]) {
        self.data.clear();
        self.data.extend_from_slice(source);
        self.output.clear();
        self.pc = 0;
    }

    pub fn set_input(&mut self, input: &[Word]) {
        self.input.clear();
        self.input.extend_from_slice(input);
        self.input.reverse(); // so we can easily pop elements in forward order
    }

    pub fn get_output(&self) -> &[Word] {
        &self.output
    }

    pub fn exec(&mut self) {
        while self.pc < self.data.len() {
            let opcode = Opcode::decode(self.data[self.pc]);
            match opcode {
                Opcode::Add { src1, src2 } => {
                    *self.param_out(3) = self.param(src1, 1) + self.param(src2, 2)
                }
                Opcode::Mul { src1, src2 } => {
                    *self.param_out(3) = self.param(src1, 1) * self.param(src2, 2)
                }
                Opcode::In => *self.param_out(1) = self.input.pop().unwrap(),
                Opcode::Out { src } => self.output.push(self.param(src, 1)),
                Opcode::JmpT { src1, src2 } => {
                    if self.param(src1, 1) != 0 {
                        debug_assert!(self.param(src2, 2) >= 0);
                        self.pc = self.param(src2, 2) as usize;
                        continue;
                    }
                }
                Opcode::JmpF { src1, src2 } => {
                    if self.param(src1, 1) == 0 {
                        debug_assert!(self.param(src2, 2) >= 0);
                        self.pc = self.param(src2, 2) as usize;
                        continue;
                    }
                }
                Opcode::Lt { src1, src2 } => {
                    *self.param_out(3) = if self.param(src1, 1) < self.param(src2, 2) {
                        1
                    } else {
                        0
                    }
                }
                Opcode::Eq { src1, src2 } => {
                    *self.param_out(3) = if self.param(src1, 1) == self.param(src2, 2) {
                        1
                    } else {
                        0
                    }
                }
                Opcode::End => {}
            }

            self.pc += opcode.len();

            if opcode == Opcode::End {
                break;
            }
        }
    }

    fn param_out(&mut self, offset: usize) -> &mut Word {
        let addr = self.data[self.pc + offset] as usize;
        &mut self.data[addr]
    }

    fn param(&self, param: Param, offset: usize) -> Word {
        match param {
            Param::Addr => self.data[self.data[self.pc + offset] as usize],
            Param::Imm => self.data[self.pc + offset],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Param {
    Addr,
    Imm,
}

impl From<Word> for Param {
    fn from(word: Word) -> Param {
        match word {
            0 => Param::Addr,
            1 => Param::Imm,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Opcode {
    Add { src1: Param, src2: Param },
    Mul { src1: Param, src2: Param },
    In,
    Out { src: Param },
    JmpT { src1: Param, src2: Param },
    JmpF { src1: Param, src2: Param },
    Lt { src1: Param, src2: Param },
    Eq { src1: Param, src2: Param },
    End,
}

impl Opcode {
    fn len(&self) -> usize {
        match *self {
            Opcode::Add { .. } | Opcode::Mul { .. } => 4,
            Opcode::In | Opcode::Out { .. } => 2,
            Opcode::JmpT { .. } | Opcode::JmpF { .. } => 3,
            Opcode::Lt { .. } | Opcode::Eq { .. } => 4,
            Opcode::End => 1,
        }
    }

    fn decode(input: Word) -> Opcode {
        let opcode = get_digits_base10(input, 0, 2);

        let param1 = || get_digits_base10(input, 2, 1).into();
        let param2 = || get_digits_base10(input, 3, 1).into();

        match opcode {
            01 => Opcode::Add {
                src1: param1(),
                src2: param2(),
            },
            02 => Opcode::Mul {
                src1: param1(),
                src2: param2(),
            },
            03 => Opcode::In,
            04 => Opcode::Out { src: param1() },
            05 => Opcode::JmpT {
                src1: param1(),
                src2: param2(),
            },
            06 => Opcode::JmpF {
                src1: param1(),
                src2: param2(),
            },
            07 => Opcode::Lt {
                src1: param1(),
                src2: param2(),
            },
            08 => Opcode::Eq {
                src1: param1(),
                src2: param2(),
            },
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
