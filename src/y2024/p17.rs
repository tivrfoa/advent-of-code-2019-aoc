use std::cmp::Reverse;
use std::collections::*;
use crate::util::*;

#[allow(dead_code)]
fn parse(input: &str) -> ([i32; 3], Vec<i32>) {
    let mut lines = input.lines();
    let a = lines.next().unwrap();
    let b = lines.next().unwrap();
    let c = lines.next().unwrap();
    let _ = lines.next().unwrap();
    let program = lines.next().unwrap();

    let a = a.split_once(": ").unwrap().1.to_i();
    let b = b.split_once(": ").unwrap().1.to_i();
    let c = c.split_once(": ").unwrap().1.to_i();

    let program: Vec<i32> = program.split_once(": ").unwrap().1.split_to_digits(',');

    ([a, b, c], program)
}

const A: i32 = 8;
const B: i32 = 9;
const C: i32 = 10;

fn process_operand(operand: i32) -> i32 {
    match operand {
        0..=3 => operand,
        4 => A,
        5 => B,
        6 => C,
        7 => todo!(),
        _ => panic!("{}", operand),
    }
}

const adv: i32 = 0;
const bxl: i32 = 1;
const bst: i32 = 2;
const jnz: i32 = 3;
const bxc: i32 = 4;
const out: i32 = 5;

fn process_opcode(opcode: i32) {
    match opcode {
        adv => {
            // division
            // A /= 2i32.pow(process_operand(operand))
        }
        bxl => {
            // B ^= operand
        }
        bst => {
            // B %= 8
        }
        jnz => {
            if A != 0 {
                // ip = literal operandd
                // do not increase ip by 2 after this
            } else {
                // do nothing
            }
        }
        bxc => {
            // B ^= C
        }
        out => {
            // outputs.push(process_operand(operand) % 8)
        }
        bdv => {
            // division
            // B /= 2i32.pow(process_operand(operand))
        }
        cdv => {
            // division
            // C /= 2i32.pow(process_operand(operand))
        }
    }
}

/// 3 bit computer -> max 111 -> 7 decimal
/// Registers: A, B and C can hold any integer
///
/// opcode is followed by an operand (3 bit)
pub fn p1(input: &str) -> String {
    let mut comma_joined_output = String::new();

    let (mut registers, program) = parse(input);
    dbg!(registers, program);

    // instruction pointer
    let mut ip = 0;


    comma_joined_output
}

pub fn p2(input: &str) -> usize {


    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sample() {
        assert_eq!("171".to_string(), p1(SAMPLE));
    }

    #[test]
    #[ignore]
    fn test_p1_in() {
        assert_eq!("171".to_string(), p1(IN));
    }

    #[test]
    #[ignore]
    fn test_p2_sample() {
        assert_eq!(171, p2(SAMPLE));
    }

    #[test]
    #[ignore]
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
