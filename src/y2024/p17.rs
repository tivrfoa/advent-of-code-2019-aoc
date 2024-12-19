use std::cmp::Reverse;
use std::collections::*;
use crate::util::*;
use rayon::prelude::*;

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
                self.registers[0] /= 2usize.pow(self.get_combo_operand() as u32);
            }
            bxl => {
                self.registers[1] ^= operand;
            }
            bst => {
                self.registers[1] = self.get_combo_operand() % 8;
            }
            jnz => {
                if self.registers[0] != 0 {
                    jumped = true;
                    self.ip = operand as usize;
                }
            }
            bxc => {
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

fn solve(program: &[usize], ans: usize) -> Option<usize> {
    println!("{:?} - {ans}", program);
    if program.is_empty() { return Some(ans); }
    for mut b in 1..=16 {
        let mut a = (ans << 3) | b;
        if a == 0 { a = 8; }
        let mut b = a % 8; // bst(2, 4)
        b = b ^ 7;         // bxl(1, 7)
        let c = a >> b;    // cdv(7, 5)
        // a = a >> 3;        // adv(0, 3)
        b = b ^ c;         // bxc(4, 4)
        b = b ^ 7;         // bxl(1, 7)
        if b % 8 == program[program.len() - 1] {
            match solve(&program[..program.len() - 1], a) {
                None => continue,
                Some(v) => {
                    println!("a: {a} sub: {v}");
                    return Some(v);
                }
            }
        }
    }

    None
}

pub fn p2_v01(input: &str) -> usize {
    let mut ret = String::new();
    let (_, program) = parse(input);

    if let Some(ans) = solve(&program, 0) {
        return ans;
    } else {
        panic!("Mission Failed!");
    }
}

fn run(program: &[usize], mut a: usize) -> bool {
    let start = a;
    let mut b = 0;
    let mut c = 0;

    for i in 0..program.len() {
        b = a % 8; // bst(2, 4)
        b = b ^ 7;         // bxl(1, 7)
        c = a >> b;    // cdv(7, 5)
        // c = a / 2usize.pow(b as u32);
        a = a >> 3;        // adv(0, 3)
        // a = a / 2usize.pow(3);        // adv(0, 3)
        b = b ^ c;         // bxc(4, 4)
        b = b ^ 7;         // bxl(1, 7)
        if b % 8 != program[i] {
            return false;
        }
    }

    println!("Found: {start}");
    true
}

pub fn p2(input: &str) -> usize {
    let (_, program) = parse(input);

    // for ast in 258962108549019usize..358962108549019 {
    //     // let a = ast * (ast >> 27);
    //     let a = ast;
    //     if run(&program, a) {
    //         return a;
    //     }
    // }

    (258_962_108_549_019usize..358962108549019).into_par_iter()
        .for_each(|a| {
            run(&program, a);
        });

    panic!("Mission Failed!");
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
    #[ignore]
    fn test_p2_sample() {
        assert_eq!(117440, p2("Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"));
    }

    #[test]
    fn test_p2_in() {
        assert_eq!(267_265_166_222_235, p2(IN));
        // 258_962_108_549_019
        // 267_265_166_222_235
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







/*

(2, 4) bst b = A % 8 -> b <= 7

(1, 7) bxl b ˆ= 7 -> b <= 7

(7, 5) cdv c = A / 2 ** b

(0, 3) adv A = A / 2 ** 3 => A = A / 8

(4, 4) bxc b ˆ= c -> b can be greater than 7 now

(1, 7) bxl b ˆ= 7

(5, 5) prints b % 8 = 2



Based on the formulas below, that happen in order, what might be the value of A? Divisions are rounded down.

b = A % 8

b = b ^ 7

c = A / (2 ** b)

A = A / 8

b = b ^ c

b = b ^ 7

b % 8  = 2

256 satisfy the first output!

The value of A that satisfies all the steps and the condition b%8=4 is 512!

The value of A that satisfies all the steps and the condition b%8=1 is 128!

The value of A that satisfies all the steps and the condition b%8=7 is 896!

The value of A that satisfies all the steps and the condition b%8=5 is 640!

The value of AA that satisfies all the steps and the condition b%8=0b%8=0 is 1024!

The value of AA that satisfies all the steps and the condition b%8=3b%8=3 is 384!

maybe try powers of two














Let's try to do the reverse operation:

- It always prints register b % 8
- Last printed number is 0 -> b is a multiple of 8, or b is 0
- A is zero, because program finished
- bxl b ˆ= 7, b was 7 or any multiple of 8 - 1
- bxc b ˆ= c, 
- adv a /= 2.pow(3) => a /= 8, A can be any value lower than 8 ...
- cdv c = a / 2.pow(b) => 0 = a / 2 ** 7 => a: any value lower than 8
- bxl b ˆ= 7, it means b was 0
- bst b = a % 8, a was already 0


*/
