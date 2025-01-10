use std::collections::*;
use crate::util::*;

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
  - the up / down / left / right buttons cause it to move its arm one button in that direction,
  - the A button causes the robot to briefly move forward, pressing the button being aimed at by the robotic arm.

Person directional keypad
  -> 1 robot at directional keypad
    -> 2 robot at directional keypad
      -> 3 robot at numeric keypad

    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+

num_keypad A to 0 -> <A

How does this sequence on robot 2 makes robot 3 type 029A ?

v<<A>>^A<A>AvA<^AA>A<vAAA>^A

v<<A -> down, left, left, activate
  - makes robot 3 move to zero

>>^A -> must go back to A to activate
  - makes robot 3 push the button

*/
const NUM_ROBOTS: usize = 3;
const GAP: char = 'G';
const nkeypad: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [GAP, '0', 'A'],
];
const dkeypad: [[char; 3]; 2] = [
    [GAP, '^', 'A'],
    ['<', 'v', '>'],
];

fn get_dir_char(d: i8) -> char {
    match d {
        N => '^',
        S => 'v',
        E => '>',
        W => '<',
        _ => panic!("{}", d),
    }
}

fn paths<const R: usize, const C: usize>(g: &[[char; C]; R])
        -> (HashMap<char, usize>, Vec<Vec<Vec<String>>>) {
    let mut map: HashMap<char, usize> = HashMap::new();
    let mut v: Vec<Vec<Vec<String>>> = vec![];
    let mut idx = 0;
    for r in 0..R {
        for c in 0..C {
            map.insert(g[r][c], idx);
            idx += 1;
            let mut iv: Vec<Vec<String>> = vec![];
            for nr in 0..R {
                for nc in 0..C {
                    iv.push(get_min_movements((r, c), (nr, nc), g));
                }
            }
            v.push(iv);
        }
    }
    (map, v)
}

fn get_min_movements<const R: usize, const C: usize>(from: (usize, usize), to: (usize, usize), g: &[[char; C]; R]) -> Vec<String> {
    if g[from.0][from.1] == GAP || g[to.0][to.1] == GAP {
        return vec!["PANIC".into()];
    }
    if from == to {
        return vec!["A".into()];
    }

    let mut ret = vec![];
    let mut min = usize::MAX;
    let rows = g.len();
    let cols = g[0].len();
    let mut pq = VecDeque::new();
    pq.push_front(("".to_string(), from, [[false; C]; R]));

    while let Some((mut m, pos, mut vis)) = pq.pop_front() {
        if m.len() > min { return ret; }
        if pos == to {
            min = m.len();
            m.push('A');
            ret.push(m);
            continue;
        }
        let (r, c) = pos;
        if vis[r][c] { continue; }
        vis[r][c] = true;
        for (nr, nc, d) in dirs(r, c, rows, cols) {
            if g[nr][nc] == GAP { continue; }
            let mut nm = m.clone();
            nm.push(get_dir_char(d));
            pq.push_back((nm, (nr, nc), vis.clone()));
        }
    }
    ret
}

fn compute_seqs<const R: usize, const C: usize>(g: &[[char; C]; R], map: &HashMap<char, usize>, dist: &[Vec<Vec<String>>]) 
        -> HashMap<(char, char), Vec<String>> {
    let mut seqs: HashMap<(char, char), Vec<String>> = HashMap::new();
    for r1 in 0..R {
        for c1 in 0..C {
            if g[r1][c1] == GAP { continue; }
            for r2 in 0..R {
                for c2 in 0..C {
                    if g[r2][c2] == GAP { continue; }
                    let l = g[r1][c1];
                    let r = g[r2][c2];
                    let from = map[&l];
                    let to   = map[&r];
                    seqs.insert((l, r), dist[from][to].clone());
                }
            }
        }
    }
    seqs
}

// Helper function for Cartesian product
fn cartesian_product(options: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    let first = &options[0];
    let rest = &options[1..];
    if rest.is_empty() {
        return vec![first.clone()];
    }
    let sub_product = cartesian_product(rest);
    for value in first {
        for sub in &sub_product {
            let mut combined = vec![value.clone()];
            combined.extend(sub.clone());
            result.push(combined);
        }
    }
    result
}

fn product(options: &[Vec<String>]) -> Vec<String> {
    let mut ss = options[0].clone();
    for i in 1..options.len() {
        let mut new = vec![];
        for s in &options[i] {
            for cs in &ss {
                let mut ns = cs.clone();
                ns.push_str(s);
                new.push(ns);
            }
        }
        ss = new;
    }
    ss
}

fn get_dir_lengths(dir_seqs: &HashMap<(char, char), Vec<String>>)
        -> HashMap<(char, char), usize> {
    let mut dir_lengths: HashMap<(char, char), usize> = HashMap::new();
    for (key, value) in dir_seqs {
        dir_lengths.insert(*key, value[0].len());
    }
    dir_lengths
}

fn solve(s: &str, seqs: &HashMap<(char, char), Vec<String>>) -> Vec<String> {
    let mut string = String::from('A');
    string.push_str(s);
    let mut options: Vec<Vec<String>> = vec![];
    for (x, y) in string.chars().zip(s.chars()) {
        options.push(seqs[&(x, y)].clone());
    }
    dbg!(options.len());
    dbg!(&options);
    let ret: Vec<String> = product(&options);
    dbg!(ret.len());
    dbg!(&ret);
    ret
}

fn compute_length(seq: &str, depth: usize,
        dir_seqs: &HashMap<(char, char), Vec<String>>,
        dir_lengths: &HashMap<(char, char), usize>,
        memo: &mut HashMap<(String, usize), usize>) -> usize {
    if let Some(v) = memo.get(&(seq.into(), depth)) {
        return *v;
    }
    let mut string = String::from('A');
    string.push_str(seq);
    if depth == 1 {
        return string.chars().zip(seq.chars())
            .map(|(x, y)| dir_lengths[&(x, y)])
            .sum();
    }
    let mut length = 0;
    for (x, y) in string.chars().zip(seq.chars()) {
        let mut min = usize::MAX;
        for subseq in &dir_seqs[&(x, y)] {
            min = min.min(compute_length(subseq, depth - 1, dir_seqs, dir_lengths, memo));
        }
        length += min;
    }
    memo.insert((seq.into(), depth), length);
    length
}

pub fn p2(input: &str) -> usize {
    let (nmap, ndist) = paths(&nkeypad);
    let (dmap, ddist) = paths(&dkeypad);

    let num_seqs: HashMap<(char, char), Vec<String>> = compute_seqs(&nkeypad, &nmap, &ndist);
    let dir_seqs: HashMap<(char, char), Vec<String>> = compute_seqs(&dkeypad, &dmap, &ddist);
    let dir_lengths = get_dir_lengths(&dir_seqs);
    dbg!(dir_lengths.len());

    let mut memo: HashMap<(String, usize), usize> = HashMap::new();
    let mut total = 0;
    for code in input.lines() {
        let inputs = solve(code, &num_seqs);
        let length = inputs
            .iter()
            .map(|i| compute_length(i, 25, &dir_seqs, &dir_lengths, &mut memo))
            .min().unwrap();
        total += length * (&code[..code.len() - 1]).parse::<usize>().unwrap()
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2_sample() {
        assert_eq!(154115708116294, p2(SAMPLE));
    }

    #[test]
    fn test_p2_in() {
        assert_eq!(303836969158972, p2(IN));
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
