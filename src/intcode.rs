use crate::util;

static mut RESULT: i32 = 0;

#[derive(Debug, PartialEq)]
enum ParameterMode {
    PositionMode,
    ImmediateMode,
}

impl ParameterMode {
    fn get_value(&self, idx: usize, mem: &[i32]) -> i32 {
        match self {
            ParameterMode::PositionMode => mem[mem[idx] as usize],
            ParameterMode::ImmediateMode => mem[idx],
        }
    }

    fn get_destination(&self, idx: usize, mem: &[i32]) -> usize {
        match self {
            ParameterMode::PositionMode => mem[idx] as usize,
            ParameterMode::ImmediateMode => {
                panic!("How to handle this? is this valid?!");
            },
        }
    }
}

impl From<u8> for ParameterMode {
    fn from(value: u8) -> Self {
        match value {
            0 => ParameterMode::PositionMode,
            1 => ParameterMode::ImmediateMode,
            _ => panic!("Invalid parameter mode: {}", value),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Opcode {
    Add {
        a: ParameterMode,
        b: ParameterMode,
        dest: ParameterMode,
    },
    Multiply {
        a: ParameterMode,
        b: ParameterMode,
        dest: ParameterMode,
    },
    Halt,
    Input {
        mode: ParameterMode,
    },
    Output {
        mode: ParameterMode,
    },
    JumpIfTrue {
        a: ParameterMode,
        b: ParameterMode,
    },
    JumpIfFalse {
        a: ParameterMode,
        b: ParameterMode,
    },
    LessThan {
        a: ParameterMode,
        b: ParameterMode,
        dest: ParameterMode,
    },
    Equals {
        a: ParameterMode,
        b: ParameterMode,
        dest: ParameterMode,
    },
}

impl Opcode {
    fn parse(n: i32) -> Self {
        let opcode_str = util::left_pad(n, 5, '0');
        println!("opcode: {}", opcode_str);
        let c_mode = (&opcode_str[0..1]).parse::<u8>().unwrap();
        let b_mode = (&opcode_str[1..2]).parse::<u8>().unwrap();
        let a_mode = (&opcode_str[2..3]).parse::<u8>().unwrap();
        let oc     = (&opcode_str[3..5]).parse::<u8>().unwrap();

        match oc {
            1 => Opcode::Add {
                a: a_mode.into(),
                b: b_mode.into(),
                dest: c_mode.into(),
            },
            2 => Opcode::Multiply {
                a: a_mode.into(),
                b: b_mode.into(),
                dest: c_mode.into(),
            },
            3 => Opcode::Input {
                mode: a_mode.into(),
            },
            4 => Opcode::Output {
                mode: a_mode.into(),
            },
            5 => Opcode::JumpIfTrue {
                a: a_mode.into(),
                b: b_mode.into(),
            },
            6 => Opcode::JumpIfFalse {
                a: a_mode.into(),
                b: b_mode.into(),
            },
            7 => Opcode::LessThan {
                a: a_mode.into(),
                b: b_mode.into(),
                dest: c_mode.into(),
            },
            8 => Opcode::Equals {
                a: a_mode.into(),
                b: b_mode.into(),
                dest: c_mode.into(),
            },
            99 => Opcode::Halt,
            _ => panic!("Invalid opcode: {}", oc),
        }
    }

    fn advance(&self) -> usize {
        match self {
            Opcode::Add { .. } => 4,
            Opcode::Multiply { .. } => 4,
            Opcode::Input { .. } => 2,
            Opcode::Output { .. } => 2,
            Opcode::JumpIfTrue { .. } => 3,
            Opcode::JumpIfFalse { .. } => 3,
            Opcode::LessThan { .. } => 4,
            Opcode::Equals { .. } => 4,
            _ => panic!("Invalid opcode to advance {:?}", self),
        }
    }

    /// Process operation and advance program counter
    fn compute(&self, pc: &mut usize, mem: &mut Vec<i32>, input: &[i32],
            in_idx: &mut usize) -> Option<i32> {
        match self {
            Opcode::Add { a, b, dest } => {
                let x = a.get_value(*pc + 1, mem);
                let y = b.get_value(*pc + 2, mem);
                let dest = dest.get_destination(*pc + 3, mem);
                mem[dest] = x + y;
                *pc += self.advance();
            },
            Opcode::Multiply { a, b, dest } => {
                let x = a.get_value(*pc + 1, mem);
                let y = b.get_value(*pc + 2, mem);
                let dest = dest.get_destination(*pc + 3, mem);
                mem[dest] = x * y;
                *pc += self.advance();
            },
            Opcode::Input { mode } => {
                let dest = mode.get_destination(*pc + 1, mem);
                mem[dest] = input[*in_idx];
                *pc += self.advance();
                *in_idx += 1;
            },
            Opcode::Output { mode } => {
                let v = mode.get_value(*pc + 1, mem);
                *pc += self.advance();
                return Some(v);
                // unsafe {
                //     RESULT = v;
                // }
                // println!("{}", v);
            },
            Opcode::JumpIfTrue { a, b } => {
                let x = a.get_value(*pc + 1, mem);
                let y = b.get_value(*pc + 2, mem);
                if x != 0 {
                    *pc = y as usize;
                } else {
                    *pc += self.advance();
                }
            },
            Opcode::JumpIfFalse { a, b } => {
                let x = a.get_value(*pc + 1, mem);
                let y = b.get_value(*pc + 2, mem);
                if x == 0 {
                    *pc = y as usize;
                } else {
                    *pc += self.advance();
                }
            },
            Opcode::LessThan { a, b, dest } => {
                let x = a.get_value(*pc + 1, mem);
                let y = b.get_value(*pc + 2, mem);
                let dest = dest.get_destination(*pc + 3, mem);
                if x < y {
                    mem[dest] = 1;
                } else {
                    mem[dest] = 0;
                }
                *pc += self.advance();
            },
            Opcode::Equals { a, b, dest } => {
                let x = a.get_value(*pc + 1, mem);
                let y = b.get_value(*pc + 2, mem);
                let dest = dest.get_destination(*pc + 3, mem);
                if x == y {
                    mem[dest] = 1;
                } else {
                    mem[dest] = 0;
                }
                *pc += self.advance();
            },
            _ => panic!("Invalid opcode to compute {:?}", self),
        }

        None
    }
}

pub fn run_prog(mem: &mut Vec<i32>, input: Vec<i32>) -> Vec<i32> {
    let mut output = vec![];
    let mut in_idx = 0;
    let mut pc = 0;
    loop {
        let opcode = Opcode::parse(mem[pc]);
        if opcode == Opcode::Halt { break; }
        if let Some(ret) = opcode.compute(&mut pc, mem, &input, &mut in_idx) {
            output.push(ret);
        }
    }

    output
}

