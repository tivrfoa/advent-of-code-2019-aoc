use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::iter::Iterator;
use std::ops::{Div, Mul, Rem};



pub const N: i8 = 0;
pub const E: i8 = 1;
pub const S: i8 = 2;
pub const W: i8 = 3;

pub struct DirsIterator {
    directions: [(bool, usize, usize, i8); 4],
    index: usize,
}

impl DirsIterator {
    fn new(r: usize, c: usize, rows: usize, cols: usize) -> Self {
        let directions = [
            (r > 0, if r > 0 { r - 1 } else { 0 }, c, N),
            (r + 1 < rows, r + 1, c, S),
            (c + 1 < cols, r, c + 1, E),
            (c > 0, r, if c > 0 { c - 1 } else { 0 }, W),
        ];
        DirsIterator { directions, index: 0 }
    }
}

impl Iterator for DirsIterator {
    type Item = (usize, usize, i8);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.directions.len() {
            let (valid, r, c, dir) = self.directions[self.index];
            self.index += 1;
            if valid {
                return Some((r, c, dir));
            }
        }
        None
    }
}

pub fn dirs(r: usize, c: usize, rows: usize, cols: usize) -> impl Iterator<Item = (usize, usize, i8)> {
    DirsIterator::new(r, c, rows, cols)
}

pub trait ParseToInt {
    fn to_i(&self) -> i32;
    fn to_i64(&self) -> i64;
    fn to_usize(&self) -> usize;
    fn to_digits<T>(&self) -> Vec<T>
    where
        T: From<u8> + Display;
    fn to_digits_grid<T>(&self) -> Vec<Vec<T>>
    where
        T: From<u8> + Display;
}

impl ParseToInt for str {
    fn to_i(&self) -> i32 {
        self.parse::<i32>().unwrap()
    }
    fn to_i64(&self) -> i64 {
        self.parse::<i64>().unwrap()
    }
    fn to_usize(&self) -> usize {
        self.parse::<usize>().unwrap()
    }
    fn to_digits<T>(&self) -> Vec<T> 
    where
        T: From<u8> + Display,
    {
        self.chars()
            .map(|c| (c as u8 - b'0').into())
            .collect()
    }
    fn to_digits_grid<T>(&self) -> Vec<Vec<T>> 
    where
        T: From<u8> + Display,
    {
        self.lines()
            .map(|line| line.chars()
                .map(|c| (c as u8 - b'0').into())
                .collect()
            )
            .collect()
    }
}

pub trait GridIterator<T> {
    fn it(&self) -> std::iter::Enumerate<std::slice::Iter<'_, T>>;
}

impl<T> GridIterator<T> for Vec<T> {
    fn it(&self) -> std::iter::Enumerate<std::slice::Iter<'_, T>> {
        self.iter().enumerate()
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
