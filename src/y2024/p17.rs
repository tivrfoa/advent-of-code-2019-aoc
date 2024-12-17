use std::cmp::Reverse;
use std::collections::*;
use crate::util::*;

#[allow(dead_code)]
fn parse(input: &str) -> ([usize; 3], Vec<usize>) {
    let mut lines = input.lines();
    let a = lines.next().unwrap();
    let b = lines.next().unwrap();
    let c = lines.next().unwrap();
    let _ = lines.next().unwrap();
    let program = lines.next().unwrap();

    let a = a.split_once(": ").unwrap().1.to_usize();
    let b = b.split_once(": ").unwrap().1.to_usize();
    let c = c.split_once(": ").unwrap().1.to_usize();

    let program: Vec<usize> = program.split_once(": ").unwrap().1.split_to_digits(',');

    ([a, b, c], program)
}

const A: usize = 8;
const B: usize = 9;
const C: usize = 10;

const adv: usize = 0;
const bxl: usize = 1;
const bst: usize = 2;
const jnz: usize = 3;
const bxc: usize = 4;
const out: usize = 5;
const bdv: usize = 6;
const cdv: usize = 7;

#[derive(Clone)]
struct Computer {
    registers: [usize; 3],
    program: Vec<usize>,
    ip: usize,
    outputs: Vec<usize>,
}

impl Computer {
    fn get_combo_operand(&self) -> usize {
        let operand = self.program[self.ip + 1];
        if operand <= 3 {
            operand
        } else {
            self.registers[operand as usize - 4]
        }
    }

    fn process_opcode(&mut self) {
        let ip = self.ip;
        let opcode = self.program[ip];
        let operand = self.program[ip + 1];
        let mut jumped = false;
        match opcode {
            adv => {
                // division
                // A /= 2usize.pow(process_operand(operand))
                self.registers[0] /= 2usize.pow(self.get_combo_operand() as u32);
            }
            bxl => {
                // B ^= operand
                self.registers[1] ^= operand;
            }
            bst => {
                // B = combo % 8
                self.registers[1] = self.get_combo_operand() % 8;
            }
            jnz => {
                if self.registers[0] != 0 {
                    jumped = true;
                    self.ip = operand as usize;
                }
            }
            bxc => {
                // B ^= C
                self.registers[1] ^= self.registers[2];
            }
            out => {
                self.outputs.push(self.get_combo_operand() % 8);
            }
            bdv => {
                self.registers[1] = self.registers[0] / 2usize.pow(self.get_combo_operand() as u32);
            }
            cdv => {
                self.registers[2] = self.registers[0] / 2usize.pow(self.get_combo_operand() as u32);
            }
            _ => panic!("Invalid opcode: {opcode}"),
        }

        if !jumped {
            self.ip += 2;
        }
    }

    fn process_opcode_p2(&mut self) -> bool {
        let ip = self.ip;
        let opcode = self.program[ip];
        let operand = self.program[ip + 1];
        let mut jumped = false;
        match opcode {
            adv => {
                // division
                // A /= 2usize.pow(process_operand(operand))
                self.registers[0] /= 2usize.pow(self.get_combo_operand() as u32);
            }
            bxl => {
                // B ^= operand
                self.registers[1] ^= operand;
            }
            bst => {
                // B = combo % 8
                self.registers[1] = self.get_combo_operand() % 8;
            }
            jnz => {
                if self.registers[0] != 0 {
                    jumped = true;
                    self.ip = operand as usize;
                }
            }
            bxc => {
                // B ^= C
                self.registers[1] ^= self.registers[2];
            }
            out => {
                if self.outputs.len() == self.program.len() {
                    return false;
                }
                let v = self.get_combo_operand() % 8;
                let out_idx = self.outputs.len();
                if v != self.program[out_idx] {
                    return false;
                }
                self.outputs.push(v);
            }
            bdv => {
                self.registers[1] = self.registers[0] / 2usize.pow(self.get_combo_operand() as u32);
            }
            cdv => {
                self.registers[2] = self.registers[0] / 2usize.pow(self.get_combo_operand() as u32);
            }
            _ => panic!("Invalid opcode: {opcode}"),
        }

        if !jumped {
            self.ip += 2;
        }

        true
    }
}

/// 3 bit computer -> max 111 -> 7 decimal
/// Registers: A, B and C can hold any integer
///
/// opcode is followed by an operand (3 bit)
pub fn p1(input: &str) -> String {
    let mut ret = String::new();
    let (registers, program) = parse(input);
    let plen = program.len();
    let mut computer = Computer {
        registers,
        program,
        ip: 0,
        outputs: vec![],
    };

    while computer.ip < plen {
        computer.process_opcode();
    }

    for o in computer.outputs {
        if !ret.is_empty() { ret.push(','); }
        ret.push_str(&mut o.to_string());
    }

    ret
}

fn solve(l: usize, r: usize, fresh_computer: Computer) -> Option<usize> {
    for a in l..=r {
        let mut computer = fresh_computer.clone();
        computer.registers[0] = a;
        let plen = computer.program.len();
        loop {
            if !computer.process_opcode_p2() {
                break;
            }
            if computer.outputs.len() == plen {
                return Some(a);
            }
            if computer.ip == plen {
                break;
            }
        }
    }

    None
}

pub fn p2(input: &str) -> usize {
    let mut ret = String::new();
    let (registers, program) = parse(input);
    let plen = program.len();
    let fresh_computer = Computer {
        registers: registers.clone(),
        program,
        ip: 0,
        outputs: vec![],
    };

    use std::thread::{self, available_parallelism};
    let num_cpus = available_parallelism().unwrap().get() - 3;
    dbg!(num_cpus);
    let total_range: usize = 1_000_000_000_000;
    let chunk_size = total_range / num_cpus;

    let mut threads = vec![];

    for i in 0..num_cpus {
        let left = i * chunk_size;
        let right = if i == num_cpus - 1 {
            total_range
        } else {
            (i + 1) * chunk_size
        };
        dbg!(i, left, right);

        let computer = fresh_computer.clone();
        threads.push(thread::spawn(move || {
            solve(left, right, computer)
        }));
    }

    let mut min_value: Option<usize> = None;

    for t in threads {
        if let Ok(result) = t.join() {
            if let Some(value) = result {
                println!("{}", value);
                min_value = match min_value {
                    Some(mv) => Some(mv.min(value)),
                    None => Some(value),
                };
            }
        }
    }

    if let Some(mv) = min_value {
        println!("Minimum value: {}", mv);
        return mv;
    }

    panic!("Mission failed!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sample() {
        assert_eq!("4,6,3,5,6,3,5,2,1,0".to_string(), p1(SAMPLE));
    }

    #[test]
    fn test_p1_in() {
        assert_eq!("2,1,0,1,7,2,5,0,3".to_string(), p1(IN));
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(117440, p2("Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"));
    }

    #[test]
    fn test_p2_in() {
        assert_eq!(171, p2(IN));
    }
}


// -------------------------- INPUT

pub static SAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

pub static IN: &str = "Register A: 52042868
Register B: 0
Register C: 0

Program: 2,4,1,7,7,5,0,3,4,4,1,7,5,5,3,0";
