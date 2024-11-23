use std::collections::HashMap;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn set(s: &str) -> HashSet<char> {
    let mut set = HashSet::new();
    s.chars().for_each(|c| {
        set.insert(c);
    });
    set
}

#[allow(dead_code)]
pub fn sorted(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}

#[allow(dead_code)]
pub fn freq(s: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    s.chars().for_each(|c| {
        map.entry(c).and_modify(|qt| *qt += 1).or_insert(1);
    });
    map
}

#[allow(dead_code)]
pub fn left_pad<T: ToString>(data: T, len: usize, c: char) -> String {
    let mut ret = String::with_capacity(len);
    let s = data.to_string();
    if len < s.len() {
        eprintln!(
            "WARNING: [left_pad] - data length {} is greater than desired length {len}.",
            s.len()
        );
        return s;
    }
    let padding_len = len - s.len();

    ret.extend(std::iter::repeat(c).take(padding_len));
    ret.push_str(&s);

    ret
}

use std::ops::Range;

pub trait PermutationsExt<I> {
    fn permutations(self) -> Vec<Vec<I>>
    where
        I: Clone + Copy + Ord;
}

impl<I> PermutationsExt<I> for Range<I>
where
    I: Clone + Copy + Ord + std::iter::Step,
{
    fn permutations(self) -> Vec<Vec<I>> {
        let mut numbers: Vec<I> = self.collect();
        let mut permutations = Vec::new();
        generate_permutations(&mut numbers, 0, &mut permutations);
        permutations
    }
}

fn generate_permutations<I>(arr: &mut [I], start: usize, permutations: &mut Vec<Vec<I>>)
where
    I: Clone + Copy + Ord,
{
    if start == arr.len() - 1 {
        permutations.push(arr.to_vec());
        return;
    }

    for i in start..arr.len() {
        arr.swap(start, i);
        generate_permutations(arr, start + 1, permutations);
        arr.swap(start, i); // Backtrack
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_pad() {
        assert_eq!("0001".to_string(), left_pad(1, 4, '0'));
        assert_eq!("0011".to_string(), left_pad(11, 4, '0'));
        assert_eq!("0111".to_string(), left_pad(111, 4, '0'));
        assert_eq!("1111".to_string(), left_pad(1111, 4, '0'));

        // data length bigger than desired length
        assert_eq!("21111".to_string(), left_pad(21111, 4, '0'));
    }

    #[test]
    fn test_permutations() {
        let perms: Vec<Vec<i32>> = (0..5).permutations();
        assert_eq!(120, perms.len());
        assert_eq!(vec![0,1,2,3,4], perms[0]);
    }
}
