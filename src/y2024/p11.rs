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

pub fn solve(n: usize, times: u8, mem: &mut HashMap<(usize, u8), usize>) -> usize {
    if times == 0 { return 1; }
    if let Some(qt) = mem.get(&(n, times)) {
        return *qt;
    }

    let mut qt = 0;
    let (l, r) = change(n);
    if l != n {
        qt += solve(l, times - 1, mem);
    }
    qt += solve(r, times - 1, mem);

    mem.insert((n, times), qt);
    qt
}

pub fn p1_0(input: &str) -> usize {
    let mut ret = 0;
    let mut tmp1: Vec<usize> = input.split(' ').map(|n| n.to_usize()).collect();
    let mut nums = Vec::with_capacity(tmp1.len() * 100);
    nums.append(&mut tmp1);

    for _ in 0..25 {
        let len = nums.len();
        for i in 0..len {
            let n = nums[i];
            let (l, r) = change(n);
            if l == n {
                nums[i] = r;
            } else {
                nums[i] = l;
                nums.push(r);
            }
        }
    }

    nums.len()
}

pub fn p1(input: &str) -> usize {
    let mut ret = 0;
    let mut nums: Vec<usize> = input.split(' ').map(|n| n.to_usize()).collect();
    let mut mem = HashMap::new();

    for n in nums {
        ret += solve(n, 25, &mut mem);
    }

    ret
}

pub fn p2(input: &str) -> usize {
    let mut ret = 0;
    let mut nums: Vec<usize> = input.split(' ').map(|n| n.to_usize()).collect();
    let mut mem = HashMap::new();

    for n in nums {
        ret += solve(n, 75, &mut mem);
    }

    ret
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
        assert_eq!(65601038650482, p2(SAMPLE));
    }

    #[test]
    fn test_p2_in() {
        assert_eq!(237994815702032, p2(IN));
    }
}


// -------------------------- INPUT

pub static SAMPLE: &str = "125 17";

pub static IN: &str = "872027 227 18 9760 0 4 67716 9245696";
