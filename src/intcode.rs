use crate::util;

#[derive(Clone, Copy, Debug, PartialEq)]
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
            }
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
        // println!("opcode: {}", opcode_str);
        let c_mode = (&opcode_str[0..1]).parse::<u8>().unwrap();
        let b_mode = (&opcode_str[1..2]).parse::<u8>().unwrap();
        let a_mode = (&opcode_str[2..3]).parse::<u8>().unwrap();
        let oc = (&opcode_str[3..5]).parse::<u8>().unwrap();

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
}

#[derive(Clone)]
pub struct Program {
    pub mem: Vec<i32>,
    input: Vec<i32>,
    pub output: Vec<i32>,
    in_idx: usize,
    pc: usize,
}

impl Program {
    /// Process operation and advance program counter
    /// @return optional output
    fn process_opcode(&mut self, opcode: Opcode) -> RunStatus {
        match opcode {
            Opcode::Add { a, b, dest } => {
                let x = a.get_value(self.pc + 1, &self.mem);
                let y = b.get_value(self.pc + 2, &self.mem);
                let dest = dest.get_destination(self.pc + 3, &self.mem);
                self.mem[dest] = x + y;
                self.pc += opcode.advance();
            }
            Opcode::Multiply { a, b, dest } => {
                let x = a.get_value(self.pc + 1, &self.mem);
                let y = b.get_value(self.pc + 2, &self.mem);
                let dest = dest.get_destination(self.pc + 3, &self.mem);
                self.mem[dest] = x * y;
                self.pc += opcode.advance();
            }
            Opcode::Input { mode } => {
                // println!("   reading input");
                if self.in_idx == self.input.len() {
                    return RunStatus::NeedInput;
                }
                let dest = mode.get_destination(self.pc + 1, &self.mem);
                self.mem[dest] = self.input[self.in_idx];
                self.pc += opcode.advance();
                self.in_idx += 1;
            }
            Opcode::Output { mode } => {
                let v = mode.get_value(self.pc + 1, &self.mem);
                self.pc += opcode.advance();
                self.output.push(v);
                return RunStatus::Output(v);
            }
            Opcode::JumpIfTrue { a, b } => {
                let x = a.get_value(self.pc + 1, &self.mem);
                let y = b.get_value(self.pc + 2, &self.mem);
                if x != 0 {
                    self.pc = y as usize;
                } else {
                    self.pc += opcode.advance();
                }
            }
            Opcode::JumpIfFalse { a, b } => {
                let x = a.get_value(self.pc + 1, &self.mem);
                let y = b.get_value(self.pc + 2, &self.mem);
                if x == 0 {
                    self.pc = y as usize;
                } else {
                    self.pc += opcode.advance();
                }
            }
            Opcode::LessThan { a, b, dest } => {
                let x = a.get_value(self.pc + 1, &self.mem);
                let y = b.get_value(self.pc + 2, &self.mem);
                let dest = dest.get_destination(self.pc + 3, &self.mem);
                if x < y {
                    self.mem[dest] = 1;
                } else {
                    self.mem[dest] = 0;
                }
                self.pc += opcode.advance();
            }
            Opcode::Equals { a, b, dest } => {
                let x = a.get_value(self.pc + 1, &self.mem);
                let y = b.get_value(self.pc + 2, &self.mem);
                let dest = dest.get_destination(self.pc + 3, &self.mem);
                if x == y {
                    self.mem[dest] = 1;
                } else {
                    self.mem[dest] = 0;
                }
                self.pc += opcode.advance();
            }
            _ => panic!("Invalid opcode to compute {:?}", opcode),
        }

        RunStatus::NoOutput
    }

    pub fn new(mem: Vec<i32>) -> Self {
        Self {
            mem,
            input: vec![],
            output: vec![],
            pc: 0,
            in_idx: 0,
        }
    }

    pub fn run(&mut self, mut input: Vec<i32>) -> RunStatus {
        use RunStatus::*;

        self.input.append(&mut input);
        let prev_out_len = self.output.len();
        loop {
            let opcode = Opcode::parse(self.mem[self.pc]);
            if opcode == Opcode::Halt {
                break;
            }
            if self.process_opcode(opcode) == NeedInput {
                return NeedInput;
            }
        }

        if prev_out_len == self.output.len() {
            NoOutput
        } else {
            Output(*self.output.last().unwrap())
        }
    }
}

#[derive(PartialEq)]
pub enum RunStatus {
    NeedInput,
    Output(i32),
    NoOutput,
}
