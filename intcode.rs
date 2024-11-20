use crate::util;

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
        println!("{}", opcode_str);
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
            Opcode::Output { .. } => 2,
            Opcode::JumpIfTrue { .. } => 3,
            Opcode::JumpIfFalse { .. } => 3,
            Opcode::LessThan { .. } => 4,
            Opcode::Equals { .. } => 4,
            _ => panic!("Invalid opcode to advance {:?}", self),
        }
    }

    /// Process operation and advance program counter
    fn compute(&self, pc: &mut usize, mem: &mut Vec<i32>) {
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
            Opcode::Output { mode } => {
                let v = mode.get_value(*pc + 1, mem);
                unsafe {
                    P1_RESULT = v;
                }
                println!("{}", v);
                *pc += self.advance();
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
    }
}

pub fn solve(system_id: i32) -> i32 {
    let mut mem: Vec<i32> = IN.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
    let dest = mem[1] as usize;
    mem[dest] = system_id;
    let mut pc = 2;

    loop {
        let opcode = Opcode::parse(mem[pc]);
        if opcode == Opcode::Halt { break; }
        opcode.compute(&mut pc, &mut mem);
    }

    unsafe {
        P1_RESULT
    }
}
