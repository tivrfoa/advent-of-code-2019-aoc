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
const NUM_ROBOTS: usize = 4;
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

fn get_min_movement<const R: usize, const C: usize>(from: (usize, usize), to: (usize, usize), g: &[[char; C]; R]) -> String {
    if g[from.0][from.1] == GAP || g[to.0][to.1] == GAP {
        return "PANIC".into();
    }
    if from == to {
        // println!("WARNING: from {from:?} is target:  {to:?}");
        return "".into();
    }

    let rows = g.len();
    let cols = g[0].len();
    let mut pq = VecDeque::new();
    let mut vis: [[bool; C]; R] = [[false; C]; R];
    pq.push_front(("".to_string(), from));

    while let Some((mut m, pos)) = pq.pop_front() {
        let (r, c) = pos;
        if vis[r][c] { continue; }
        vis[r][c] = true;
        if pos == to {
            return m;
        }
        for (nr, nc, d) in dirs(r, c, rows, cols) {
            if g[nr][nc] == GAP { continue; }
            let mut nm = m.clone();
            nm.push(get_dir_char(d));
            pq.push_back((nm, (nr, nc)));
        }
    }
    panic!("failed: from {from:?}, to: {to:?}");
}


fn get_path<const R: usize, const C: usize>(g: &[[char; C]; R])
        -> (HashMap<char, usize>, Vec<Vec<String>>) {
    let mut map: HashMap<char, usize> = HashMap::new();
    let mut v: Vec<Vec<String>> = vec![];
    let mut idx = 0;
    for r in 0..R {
        for c in 0..C {
            map.insert(g[r][c], idx);
            idx += 1;
            let mut iv: Vec<String> = vec![];
            for nr in 0..R {
                for nc in 0..C {
                    iv.push(get_min_movement((r, c), (nr, nc), g));
                }
            }
            v.push(iv);
        }
    }
    (map, v)
}


fn solve_directional_keypad(dest: char, robot_idx: usize, rp: &mut [char; NUM_ROBOTS],
        dmap: &HashMap<char, usize>, ddist: &[Vec<String>],
        robots_path: &mut Vec<String>) {
    let from_idx = dmap[&rp[robot_idx]];
    let to_idx = dmap[&dest];
    let p = &ddist[from_idx][to_idx];
    if robot_idx > 0 {
        for c in p.chars() {
            solve_directional_keypad(c, robot_idx - 1, rp, dmap, ddist, robots_path);
        }
    }
    robots_path[robot_idx].push_str(p);
    robots_path[robot_idx].push('A');
    rp[robot_idx] = dest;
}

pub fn p1(input: &str) -> usize {
    let (nmap, ndist) = get_path(&nkeypad);
    let (dmap, ddist) = get_path(&dkeypad);

    // make ddist A to v be equals problem description for debugging
    // ddist[2][4] = "<v".into();

    // dbg!(nmap, ndist);
    // dbg!(dmap, ddist);

    let mut rp = ['A'; NUM_ROBOTS]; // robots position
    let mut robots_path: Vec<String> = vec![String::new(); NUM_ROBOTS];
    let nidx = rp.len() - 1;

    let mut solve = |code: &str| -> usize {
        println!("Solving code: {code}");
        for c in code.chars() {
            let from_idx = nmap[&rp[nidx]];
            let to_idx = nmap[&c];
            let p = &ndist[from_idx][to_idx];
            robots_path[nidx].push_str(p);
            robots_path[nidx].push('A');
            // dbg!(nk_r, nk_c, from_idx, to_idx, p);
            for c2 in p.chars() {
                solve_directional_keypad(c2, nidx - 1, &mut rp, &dmap, &ddist, &mut robots_path);
            }
            solve_directional_keypad('A', nidx - 1, &mut rp, &dmap, &ddist, &mut robots_path);
            rp[nidx] = c;
        }
        let s = &robots_path[0];
        let n = (&code[..3]).parse::<usize>().unwrap();
        let v = s.len() * n;
        dbg!(&s, s.len(), n, v);
        v
    };

    // let sum = input.lines()
    //     .map(|l| solve(l, &mut rp))
    //     .sum();
    let sum = solve("029A");

    dbg!(robots_path);

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
        assert_eq!(171, p1(SAMPLE));
    }

    #[test]
    #[ignore]
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
