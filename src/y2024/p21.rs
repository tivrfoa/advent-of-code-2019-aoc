use std::cmp::Reverse;
use std::collections::*;
use crate::util::*;

#[allow(dead_code)]
fn parse(input: &str) -> usize {


    0
}

/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+

Directional keypad:
  - first change dir
  - then press A to move
*/

fn get_dir_char(d: i8) -> char {
    match d {
        N => '^',
        S => 'v',
        E => '>',
        W => '<',
        _ => panic!("{}", d),
    }
}

fn get_min_movement(from: (usize, usize), target: u8, g: &NKeyPad) -> String {
    if g[from.0][from.1] == target {
        println!("WARNING: from {from:?} is target:  {target}");
        return "".into();
    }
    let mut mov = String::new();
    let rows = g.len();
    let cols = g[0].len();
    let mut pq = VecDeque::new();
    pq.push_front(("".to_string(), from));

    while let Some((mut m, (r, c))) = pq.pop_front() {
        if g[r][c] == target {
            m.push('A');
            return m;
        }
        for (nr, nc, d) in dirs(r, c, rows, cols) {
            let mut nm = m.clone();
            nm.push(get_dir_char(d));
            pq.push_back((nm, (nr, nc)));
        }
    }
    panic!("failed: from {:?}, target:  {}", from, target);
}

type NKeyPad = [[u8; 3]; 4];
const A: u8 = 10;
const nkeypad: [[u8; 3]; 4] = [
    [7, 8, 9],
    [4, 5, 6],
    [1, 2, 3],
    [11,0, A],
];
pub fn p1(input: &str) -> usize {

    let nkeypad_distances: Vec<Vec<String>> = {
        let mut v: Vec<Vec<String>> = vec![];
        for r in 0..4 {
            for c in 0..3 {
                v.push((0..=9)
                    .map(|t| get_min_movement((r, c), t, &nkeypad))
                    .collect());
            }
        }
        v
    };

    dbg!(nkeypad_distances);
    0
}

pub fn p2(input: &str) -> usize {


    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sample() {
        assert_eq!(171, p1(SAMPLE));
    }

    #[test]
    fn test_p1_in() {
        assert_eq!(171, p1(IN));
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

pub static SAMPLE: &str = "029A
980A
179A
456A
379A";

pub static IN: &str = "869A
180A
596A
965A
973A";
