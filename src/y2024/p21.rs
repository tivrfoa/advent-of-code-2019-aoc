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
        // println!("WARNING: from {from:?} is target:  {to:?}");
        return vec!["".into()];
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

fn get_ways(code: &str, 
        map: &HashMap<char, usize>, dist: &[Vec<Vec<String>>]) -> Vec<String> {
    let mut ways = vec![String::new()];
    let mut curr_pos = 'A';

    for dest in code.chars() {
        let from_idx = map[&curr_pos];
        let to_idx   = map[&dest];
        let mut new_ways = vec![];
        for d in &dist[from_idx][to_idx] {
            // println!("{from_idx} -> {to_idx}: {d:?}");
            for w in &ways {
                let mut s = w.clone();
                s.push_str(d);
                s.push('A');
                new_ways.push(s);
            }
        }
        ways = new_ways;
        curr_pos = dest;
    }
    ways
}

pub fn p1(input: &str) -> usize {
    let mut sum = 0;
    let (nmap, ndist) = paths(&nkeypad);
    let (dmap, ddist) = paths(&dkeypad);

    for code in input.lines() {
        let num_ways = get_ways(code, &nmap, &ndist);
     
        let mut ways = vec![];
        for w in num_ways {
            ways.append(&mut get_ways(&w, &dmap, &ddist));
        }

        let mut you_ways = vec![];
        for w in ways {
            let min = get_ways(&w, &dmap, &ddist).into_iter().min_by_key(|s| s.len()).unwrap();
            you_ways.push(min);
        }

        // dbg!(you_ways);
        let min = you_ways.iter().min_by_key(|s| s.len()).unwrap();
        dbg!(min);
        let n = (&code[..3]).parse::<usize>().unwrap();
        sum += min.len() * n;
    }

    sum
}

pub fn p2(input: &str) -> usize {


    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sample() {
        assert_eq!(126384, p1(SAMPLE));
    }

    #[test]
    fn test_p1_in() {
        assert_eq!(248108, p1(IN));
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
