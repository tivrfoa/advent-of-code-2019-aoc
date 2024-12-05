use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::{Div, Mul, Rem};

pub trait ParseToInt {
    fn to_i(&self) -> i32;
}

impl ParseToInt for str {
    fn to_i(&self) -> i32 {
        self.parse::<i32>().unwrap()
    }
}

#[allow(dead_code)]
pub fn input_to_char_grid(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

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

#[allow(dead_code)]
pub fn get_grid(panels: &HashMap<(i64, i64), i64>, convertion: &HashMap<i64, char>) -> Vec<Vec<char>> {
    // Find the bounds
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    for (coord, _) in panels.iter() {
        if coord.0 < min_x { min_x = coord.0; }
        if coord.0 > max_x { max_x = coord.0; }
        if coord.1 < min_y { min_y = coord.1; }
        if coord.1 > max_y { max_y = coord.1; }
    }

    // Create the grid with dimensions (max_x - min_x + 1) x (max_y - min_y + 1)
    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    let mut grid = vec![vec![' '; width]; height];

    // Populate the grid
    for (&(x, y), &value) in panels.iter() {
        let grid_x = (x - min_x) as usize;
        let grid_y = (y - min_y) as usize;
        
        grid[grid_y][grid_x] = convertion[&value];
    }

    grid
}

#[allow(dead_code)]
pub fn draw_grid(panels: &HashMap<(i64, i64), i64>, convertion: &HashMap<i64, char>) {
    let grid = get_grid(panels, convertion);

    // Print the grid (flipped vertically)
    for row in grid.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

#[allow(dead_code)]
pub fn draw_grid_rev(panels: &HashMap<(i64, i64), i64>, convertion: &HashMap<i64, char>) {
    let grid = get_grid(panels, convertion);

    // Print the grid (flipped vertically)
    for row in grid.iter().rev() { // reverse the iterator
        println!("{}", row.iter().collect::<String>());
    }
}

// Function to calculate GCD using Euclidean algorithm
#[allow(dead_code)]
pub fn gcd<T>(a: T, b: T) -> T
where
    T: Copy + Rem<Output = T> + PartialOrd + Div<Output = T> + From<u8>,
{
    if b == T::from(0) {
        a
    } else {
        gcd(b, a % b)
    }
}

// Function to calculate LCM of two numbers
#[allow(dead_code)]
pub fn lcm<T>(a: T, b: T) -> T
where
    T: Copy + Mul<Output = T> + Rem<Output = T> + PartialOrd + Div<Output = T> + From<u8>,
{
    (a * b) / gcd(a, b)
}

// Function to calculate LCM of an array
#[allow(dead_code)]
pub fn lcm_of_array<T>(arr: &[T]) -> T
where
    T: Copy + Mul<Output = T> + Rem<Output = T> + PartialOrd + From<u8> + Div<Output = T>,
{
    arr.iter().fold(T::from(1), |acc, &x| lcm(acc, x))
}

// Reduce function to simplify vector directions
#[allow(dead_code)]
pub fn red(x: isize, y: isize) -> (isize, isize) {
    assert!(x != 0 || y != 0);
    let g = gcd(x, y);
    (x / g, y / g)
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
