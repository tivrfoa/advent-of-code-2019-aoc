#[derive(Debug)]
enum Opcode {
    Add,
    Multiply,
    Halt,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            99 => Opcode::Halt,
            _ => panic!("Invalid opcode: {}", value),
        }
    }
}

fn p1(input: &str) -> usize {
    let mut ops: Vec<usize> = input.split(",").map(|v| v.parse::<usize>().unwrap()).collect();
    ops[1] = 12;
    ops[2] = 2;
    let ret = solve(ops);
    println!("{}", ret);
    ret
}

fn solve(mut ops: Vec<usize>) -> usize {
    let len = ops.len();

    for i in (0..len).step_by(4) {
        if ops[i] == 99 { break; }
        let a = ops[ops[i + 1]];
        let b = ops[ops[i + 2]];
        let dest = ops[i + 3];

        match ops[i] {
            1 => ops[dest] = a + b,
            2 => ops[dest] = a * b,
            _ => panic!("Invalid opcode: {}", ops[i]),
        }
    }

    ops[0]
}

fn p2(input: &str) {
    let ops: Vec<usize> = input.split(",").map(|v| v.parse::<usize>().unwrap()).collect();

    for n in 0..=99 {
        for v in 0..=99 {
            let mut copy = ops.clone();
            copy[1] = n;
            copy[2] = v;
            if solve(copy) == 19690720 {
                println!("{}", 100 * n + v);
                return;
            }
        }
    }
    panic!("Failed to find an answer.");
}

fn main() {
    // p1(SAMPLE1);
    // p1(SAMPLE2);
    // p1(SAMPLE3);
    // p1(SAMPLE4);
    assert_eq!(5534943, p1(IN));
    p2(IN);
}


static SAMPLE1: &'static str = "1,0,0,0,99";
static SAMPLE2: &'static str = "2,3,0,3,99";
static SAMPLE3: &'static str = "2,4,4,5,99,0";
static SAMPLE4: &'static str = "1,1,1,4,99,5,6,0,99";

static IN: &'static str = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,19,9,23,1,23,9,27,1,10,27,31,1,13,31,35,1,35,10,39,2,39,9,43,1,43,13,47,1,5,47,51,1,6,51,55,1,13,55,59,1,59,6,63,1,63,10,67,2,67,6,71,1,71,5,75,2,75,10,79,1,79,6,83,1,83,5,87,1,87,6,91,1,91,13,95,1,95,6,99,2,99,10,103,1,103,6,107,2,6,107,111,1,13,111,115,2,115,10,119,1,119,5,123,2,10,123,127,2,127,9,131,1,5,131,135,2,10,135,139,2,139,9,143,1,143,2,147,1,5,147,0,99,2,0,14,0";
