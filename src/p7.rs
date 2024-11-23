use crate::intcode::*;
use crate::util::*;

pub fn p1(input: &str) -> i32 {
    let mut mem: Vec<i32> = input
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let mut max = 0;

    for perms in (0..5).permutations() {
        let mut amplifiers = vec![Program::new(mem.clone()); 5];
        let mut v = 0;
        for (i, a) in perms.into_iter().enumerate() {
            println!("----- perm {a}");
            v = match amplifiers[i].run(vec![a, v]) {
                RunStatus::Output(v) => v,
                RunStatus::NoOutput => {
                    println!("Iteration produced no output?!");
                    v = -1;
                    break;
                }
                _ => panic!("Invalid return for part 1?!!"),
            };
        }
        max = max.max(v);
    }

    println!("{}", max);
    max
}

pub fn p2(input: &str) -> i32 {
    let mut mem: Vec<i32> = input
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let mut max = 0;

    for perms in (5..10).permutations() {
        let mut amplifiers = vec![Program::new(mem.clone()); 5];
        for i in 0..5 {
            amplifiers[i].run(vec![perms[i]]);
        }

        let mut v = 0;

        'l: loop {
            for i in 0..5 {
                let loc = amplifiers[i].output.len();
                let resp = amplifiers[i].run(vec![v]);
                if loc == amplifiers[i].output.len() {
                    assert!(resp == RunStatus::NoOutput);
                    assert_eq!(0, i);
                    break 'l;
                }
                assert_eq!(loc + 1, amplifiers[i].output.len());
                v = amplifiers[i].output[loc];
            }
        }
        max = max.max(v);
    }

    println!("{}", max);

    max
}

pub static IN: &'static str = "3,8,1001,8,10,8,105,1,0,0,21,30,55,76,97,114,195,276,357,438,99999,3,9,102,3,9,9,4,9,99,3,9,1002,9,3,9,1001,9,5,9,1002,9,2,9,1001,9,2,9,102,2,9,9,4,9,99,3,9,1002,9,5,9,1001,9,2,9,102,5,9,9,1001,9,4,9,4,9,99,3,9,1001,9,4,9,102,5,9,9,101,4,9,9,1002,9,4,9,4,9,99,3,9,101,2,9,9,102,4,9,9,1001,9,5,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,99,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,99";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(359142, p1(IN));
    }

    #[test]
    fn test_p2() {
        assert_eq!(4374895, p2(IN));
    }
}
