use std::collections::*;
use crate::util::*;

#[allow(dead_code)]
fn parse(input: &str) -> usize {


    0
}

fn change(n: usize) -> Vec<usize> {
    let sn: String = n.to_string();
    if n == 0 {
        vec![1]
    } else if sn.len() % 2 == 0 {
        let m = sn.len() / 2;
        let l = sn[0..m].parse::<usize>().unwrap();
        let r = sn[m..].parse::<usize>().unwrap();
        vec![l, r]
    } else {
        vec![n * 2024]
    }
}

pub fn p1(input: &str) -> usize {
    let mut ret = 0;
    let mut nums: Vec<usize> = input.split(' ').map(|n| n.to_usize()).collect();

    for _ in 0..25 {
        let mut new = vec![];
        for n in nums {
            new.append(&mut change(n));
        }
        nums = new;
    }

    nums.len()
}

pub fn p2(input: &str) -> usize {


    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sample() {
        assert_eq!(55312, p1(SAMPLE));
    }

    #[test]
    fn test_p1_in() {
        assert_eq!(199946, p1(IN));
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

pub static SAMPLE: &str = "125 17";

pub static IN: &str = "872027 227 18 9760 0 4 67716 9245696";
