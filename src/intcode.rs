use crate::util;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
enum ParameterMode {
    PositionMode,
    ImmediateMode,
    RelativeMode,
}

impl From<u8> for ParameterMode {
    fn from(value: u8) -> Self {
        match value {
            0 => ParameterMode::PositionMode,
            1 => ParameterMode::ImmediateMode,
            2 => ParameterMode::RelativeMode,
            _ => panic!("Invalid parameter mode: {}", value),
        }
    }
}

use ParameterMode::*;

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
    RelativeBaseOffset {
        a: ParameterMode,
    },
}

impl Opcode {
    fn parse(n: i32) -> Self {
        let opcode_str = util::left_pad(n, 5, '0');
        // println!("opcode: {}", opcode_str);
        let c_mode = opcode_str[0..1].parse::<u8>().unwrap();
        let b_mode = opcode_str[1..2].parse::<u8>().unwrap();
        let a_mode = opcode_str[2..3].parse::<u8>().unwrap();
        let oc = opcode_str[3..5].parse::<u8>().unwrap();

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
            9 => Opcode::RelativeBaseOffset {
                a: a_mode.into(),
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
            Opcode::RelativeBaseOffset { .. } => 2,
            _ => panic!("Invalid opcode to advance {:?}", self),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program {
    pub mem: HashMap<usize, i64>,
    input: Vec<i64>,
    pub output: Vec<i64>,
    in_idx: usize,
    pc: usize,
    relative_base: i64,
}

impl Program {
    fn get_value(&self, idx: usize, parameter_mode: ParameterMode) -> i64 {
        match parameter_mode {
            PositionMode => match self.mem.get(&idx) {
                Some(v) => match self.mem.get(&(*v as usize)) {
                    Some(v) => *v,
                    None => self.mem[&0],
                }
                None => todo!(),
            }
            ImmediateMode => match self.mem.get(&idx) {
                Some(v) => *v,
                None => 0,
            },
            RelativeMode => match self.mem.get(&idx) {
                Some(v) => self.mem[&((*v + self.relative_base) as usize)],
                None => todo!(),
            }
        }
    }

    fn get_destination(&self, idx: usize, parameter_mode: ParameterMode) -> usize {
        match parameter_mode {
            PositionMode => match self.mem.get(&idx) {
                Some(v) => *v as usize,
                None => 0,
            },
            ImmediateMode => {
                panic!("How to handle this? is this valid?!");
            }
            RelativeMode => match self.mem.get(&idx) {
                Some(v) => (*v + self.relative_base) as usize,
                None => todo!(),
            },
        }
    }

    /// Process operation and advance program counter
    /// @return optional output
    fn process_opcode(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::Add { a, b, dest } => {
                let x = self.get_value(self.pc + 1, a);
                let y = self.get_value(self.pc + 2, b);
                let dest = self.get_destination(self.pc + 3, dest);
                self.mem.insert(dest, x + y);
                self.pc += opcode.advance();
            }
            Opcode::Multiply { a, b, dest } => {
                let x = self.get_value(self.pc + 1, a);
                let y = self.get_value(self.pc + 2, b);
                let dest = self.get_destination(self.pc + 3, dest);
                self.mem.insert(dest, x * y);
                self.pc += opcode.advance();
            }
            Opcode::Input { mode } => {
                // println!("   reading input");
                let dest = self.get_destination(self.pc + 1, mode);
                self.mem.insert(dest, self.input[self.in_idx]);
                self.pc += opcode.advance();
                self.in_idx += 1;
            }
            Opcode::Output { mode } => {
                let v = self.get_value(self.pc + 1, mode);
                self.pc += opcode.advance();
                // println!("{v}");
                self.output.push(v);
            }
            Opcode::JumpIfTrue { a, b } => {
                let x = self.get_value(self.pc + 1, a);
                let y = self.get_value(self.pc + 2, b);
                if x != 0 {
                    self.pc = y as usize;
                } else {
                    self.pc += opcode.advance();
                }
            }
            Opcode::JumpIfFalse { a, b } => {
                let x = self.get_value(self.pc + 1, a);
                let y = self.get_value(self.pc + 2, b);
                if x == 0 {
                    self.pc = y as usize;
                } else {
                    self.pc += opcode.advance();
                }
            }
            Opcode::LessThan { a, b, dest } => {
                let x = self.get_value(self.pc + 1, a);
                let y = self.get_value(self.pc + 2, b);
                let dest = self.get_destination(self.pc + 3, dest);
                if x < y {
                    self.mem.insert(dest, 1);
                } else {
                    self.mem.insert(dest, 0);
                }
                self.pc += opcode.advance();
            }
            Opcode::Equals { a, b, dest } => {
                let x = self.get_value(self.pc + 1, a);
                let y = self.get_value(self.pc + 2, b);
                let dest = self.get_destination(self.pc + 3, dest);
                if x == y {
                    self.mem.insert(dest, 1);
                } else {
                    self.mem.insert(dest, 0);
                }
                self.pc += opcode.advance();
            }
            Opcode::RelativeBaseOffset { a } => {
                let v = self.get_value(self.pc + 1, a);
                self.pc += opcode.advance();
                self.relative_base += v;
            }
            _ => panic!("Invalid opcode to compute {:?}", opcode),
        }
    }

    pub fn new(mem: HashMap<usize, i64>) -> Self {
        Self {
            mem,
            input: vec![],
            output: vec![],
            pc: 0,
            in_idx: 0,
            relative_base: 0,
        }
    }

    pub fn from_input(input: &str) -> Self {
        let mem_vec: Vec<i64> = input
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        let mut mem: HashMap<usize, i64> = HashMap::with_capacity(mem_vec.len() * 2);
        for (i, v) in mem_vec.into_iter().enumerate() {
            mem.insert(i, v);
        }

        Self::new(mem)
    }

    pub fn run_input(&mut self, input: i64) -> RunStatus {
        self.input.push(input);
        self.run()
    }

    pub fn run_inputs(&mut self, mut input: Vec<i64>) -> RunStatus {
        self.input.append(&mut input);
        self.run()
    }

    pub fn run(&mut self) -> RunStatus {
        loop {
            let opcode = Opcode::parse(self.mem[&self.pc] as i32);

            match opcode {
                Opcode::Halt => return RunStatus::Halted,
                Opcode::Input { mode: _ } if self.in_idx == self.input.len() =>
                        return RunStatus::NeedInput,
                _ => self.process_opcode(opcode),
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RunStatus {
    NeedInput,
    Halted,
}

