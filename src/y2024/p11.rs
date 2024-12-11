use std::collections::*;
use crate::util::*;

fn count_digits(mut num: usize) -> u32 {
    if num == 0 {
        return 1;
    }
    
    let mut count = 0;
    while num != 0 {
        num /= 10;
        count += 1;
    }
    count
}

fn split_even_digits(num: usize, digits: u32) -> (usize, usize) {
    let divisor = 10usize.pow(digits / 2);
    let left = num / divisor;
    let right = num % divisor;
    (left, right)
}

fn change(n: usize) -> (usize, usize) {
    let sn: String = n.to_string();
    if n == 0 {
        (n, 1)
    } else {
        let digits = count_digits(n);
        if digits % 2 == 0 {
            split_even_digits(n, digits)
        } else {
            (n, n * 2024)
        }
    }
}

pub fn p1(input: &str) -> usize {
    let mut ret = 0;
    let mut nums: Vec<usize> = input.split(' ').map(|n| n.to_usize()).collect();

    for _ in 0..25 {
        let mut new = Vec::with_capacity(nums.len() * 2);
        for n in nums {
            let (l, r) = change(n);
            if l != n {
                new.push(l);
            }
            new.push(r);
        }
        nums = new;
    }

    nums.len()
}

pub fn p2(input: &str) -> usize {
    let mut ret = 0;
    let mut nums: Vec<usize> = input.split(' ').map(|n| n.to_usize()).collect();

    for _ in 0..75 {
        let mut new = Vec::with_capacity(nums.len() * 2);
        for n in nums {
            let (l, r) = change(n);
            if l != n {
                new.push(l);
            }
            new.push(r);
        }
        nums = new;
    }

    nums.len()
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
    fn test_p2_sample() {
        assert_eq!(171, p2(SAMPLE));
    }

    #[test]
    fn test_p2_in() {
        assert_eq!(171, p2(IN));
    }
}


// -------------------------- INPUT

pub static SAMPLE: &str = "125 17";

pub static IN: &str = "872027 227 18 9760 0 4 67716 9245696";
