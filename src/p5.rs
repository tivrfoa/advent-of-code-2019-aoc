use crate::util;

static mut P1_RESULT: i32 = 0;

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

// #[derive(Debug, PartialEq)]
// struct ValueMode {
//     value: i32,
//     mode: ParameterMode,
// }

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

pub fn p1() -> i32 {
    solve(1)
}

pub fn p2() -> i32 {
    solve(5)
}

static IN: &'static str = "3,225,1,225,6,6,1100,1,238,225,104,0,1002,43,69,224,101,-483,224,224,4,224,1002,223,8,223,1001,224,5,224,1,224,223,223,1101,67,60,225,1102,5,59,225,1101,7,16,225,1102,49,72,225,101,93,39,224,101,-98,224,224,4,224,102,8,223,223,1001,224,6,224,1,224,223,223,1102,35,82,225,2,166,36,224,101,-4260,224,224,4,224,102,8,223,223,101,5,224,224,1,223,224,223,102,66,48,224,1001,224,-4752,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1001,73,20,224,1001,224,-55,224,4,224,102,8,223,223,101,7,224,224,1,223,224,223,1102,18,41,224,1001,224,-738,224,4,224,102,8,223,223,101,6,224,224,1,224,223,223,1101,68,71,225,1102,5,66,225,1101,27,5,225,1101,54,63,224,1001,224,-117,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1,170,174,224,101,-71,224,224,4,224,1002,223,8,223,1001,224,4,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1007,226,226,224,1002,223,2,223,1006,224,329,1001,223,1,223,1007,226,677,224,102,2,223,223,1006,224,344,1001,223,1,223,108,677,677,224,102,2,223,223,1005,224,359,1001,223,1,223,1007,677,677,224,1002,223,2,223,1006,224,374,101,1,223,223,8,677,226,224,1002,223,2,223,1006,224,389,101,1,223,223,7,226,226,224,1002,223,2,223,1005,224,404,101,1,223,223,7,677,226,224,102,2,223,223,1005,224,419,1001,223,1,223,8,226,677,224,1002,223,2,223,1005,224,434,101,1,223,223,1008,226,677,224,102,2,223,223,1006,224,449,1001,223,1,223,7,226,677,224,1002,223,2,223,1006,224,464,1001,223,1,223,108,677,226,224,102,2,223,223,1005,224,479,101,1,223,223,108,226,226,224,1002,223,2,223,1006,224,494,101,1,223,223,8,226,226,224,1002,223,2,223,1005,224,509,1001,223,1,223,1107,677,226,224,102,2,223,223,1005,224,524,1001,223,1,223,1107,226,226,224,102,2,223,223,1005,224,539,1001,223,1,223,1108,677,677,224,1002,223,2,223,1006,224,554,101,1,223,223,107,226,677,224,102,2,223,223,1005,224,569,1001,223,1,223,1108,226,677,224,1002,223,2,223,1005,224,584,1001,223,1,223,1107,226,677,224,1002,223,2,223,1005,224,599,1001,223,1,223,1008,226,226,224,1002,223,2,223,1005,224,614,101,1,223,223,107,226,226,224,102,2,223,223,1006,224,629,1001,223,1,223,1008,677,677,224,1002,223,2,223,1006,224,644,101,1,223,223,107,677,677,224,1002,223,2,223,1005,224,659,101,1,223,223,1108,677,226,224,1002,223,2,223,1006,224,674,1001,223,1,223,4,223,99,226";
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(12234644, p1());
    }
}

