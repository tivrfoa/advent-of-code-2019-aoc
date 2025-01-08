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

fn simple_ways(from: char, to: char,
        map: &HashMap<char, usize>, dist: &[Vec<Vec<String>>]) -> Vec<String> {
    let mut ways = vec![];
    let from_idx = map[&from];
    let to_idx   = map[&to];
    for d in &dist[from_idx][to_idx] {
        let mut s = String::from(d);
        s.push('A');
        ways.push(s);
    }
    ways
}

/// example of a first call: mem, 'A', '0', 25, map, dist
fn get_ways_depth(mem_depth: &mut HashMap<(char, char, u8), Vec<String>>, 
        from: char, to: char, depth: u8,
        map: &HashMap<char, usize>, dist: &[Vec<Vec<String>>]) -> Vec<String> {
    if let Some(w) = mem_depth.get(&(curr_pos, dest, depth)) {
        // println!("Found in cache");
        return w.clone();
    }

    if depth == 0 {
        let ways = simple_ways(from, to, map, dist);
        mem_depth.insert((from, to, depth), ways.clone());
        return ways;
    }

    let mut input = get_ways_depth(mem_depth, from, to, depth - 1, map, dist);
    let mut ways = vec![];
    for s in input {
        let mut prev = from;
        for c in s.chars() {
            let ways = simple_ways(prev, c, map, dist);
        }
    }

    // If remainder is empty, return current paths
    if remainder.is_empty() {
        mem_ways.insert((curr_pos, dest, remainder.into()), ways.clone());
        return ways;
    }

    let mut final_ways = vec![];
    let next_dest = remainder.chars().next().unwrap();
    for rem_path in get_ways2(mem_ways, dest, next_dest, &remainder[1..], map, dist) {
        for w in &ways {
            let mut s = w.clone();
            s.push_str(&rem_path);
            final_ways.push(s);
        }
    }
    mem_ways.insert((curr_pos, dest, remainder.into()), final_ways.clone());
    final_ways
}

fn get_ways2(mem_ways: &mut HashMap<(char, char, String), Vec<String>>, curr_pos: char, dest: char, remainder: &str, 
        map: &HashMap<char, usize>, dist: &[Vec<Vec<String>>]) -> Vec<String> {
    if let Some(w) = mem_ways.get(&(curr_pos, dest, remainder.to_string())) {
        // println!("Found in cache");
        return w.clone();
    }

    let from_idx = map[&curr_pos];
    let to_idx   = map[&dest];
    let mut ways = vec![];
    for d in &dist[from_idx][to_idx] {
        // println!("{from_idx} -> {to_idx}: {d:?}");
        let mut s = String::from(d);
        s.push('A');
        ways.push(s);
    }

    // If remainder is empty, return current paths
    if remainder.is_empty() {
        mem_ways.insert((curr_pos, dest, remainder.into()), ways.clone());
        return ways;
    }

    let mut final_ways = vec![];
    let next_dest = remainder.chars().next().unwrap();
    for rem_path in get_ways2(mem_ways, dest, next_dest, &remainder[1..], map, dist) {
        for w in &ways {
            let mut s = w.clone();
            s.push_str(&rem_path);
            final_ways.push(s);
        }
    }
    mem_ways.insert((curr_pos, dest, remainder.into()), final_ways.clone());
    final_ways
}

fn get_ways(mem_ways: &mut HashMap<String, Vec<String>>, code: &str, 
        map: &HashMap<char, usize>, dist: &[Vec<Vec<String>>]) -> Vec<String> {
    if let Some(w) = mem_ways.get(code) {
        // println!("Found in cache");
        return w.clone();
    }
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
    mem_ways.insert(code.into(), ways.clone());
    ways
}

pub fn p1(input: &str) -> usize {
    let mut sum = 0;
    let (nmap, ndist) = paths(&nkeypad);
    let (dmap, ddist) = paths(&dkeypad);
    let mut mem_ways_useless: HashMap<String, Vec<String>> = HashMap::new();
    let mut mem_ways: HashMap<(char, char, String), Vec<String>> = HashMap::new();

    for code in input.lines() {
        let num_ways = get_ways(&mut mem_ways_useless, code, &nmap, &ndist);
     
        let mut ways = vec![];
        let mut min_len = usize::MAX;
        for w in num_ways {
            // ways.append(&mut get_ways(&mut mem_ways, &w, &dmap, &ddist));
            let dest = w.chars().next().unwrap();
            let mut rem_ways = get_ways2(&mut mem_ways, 'A', dest, &w[1..], &dmap, &ddist);
            let len = rem_ways.iter().map(|s| s.len()).min().unwrap();
            if len <= min_len {
                min_len = len;
                for rw in rem_ways.into_iter().filter(|s| s.len() == min_len) {
                    ways.push(rw);
                }
            }
        }

        let mut you_ways = vec![];
        for w in ways {
            let min = get_ways(&mut mem_ways_useless, &w, &dmap, &ddist).into_iter().min_by_key(|s| s.len()).unwrap();
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

pub fn p2_depth(input: &str) -> usize {
    println!("========= PART 2 ===============");
    let mut sum = 0;
    let (nmap, ndist) = paths(&nkeypad);
    let (dmap, ddist) = paths(&dkeypad);
    let mut mem_ways_useless: HashMap<String, Vec<String>> = HashMap::new();
    let mut mem_ways: HashMap<(char, char, String), Vec<String>> = HashMap::new();

    for code in input.lines() {
        println!("==== Code: {code}");
        let num_ways = get_ways(&mut mem_ways_useless, code, &nmap, &ndist);
     
        let mut ways = num_ways;
        for robot in 0..25 {
            println!("===== ROBOT {robot}");
            let mut new_ways = vec![];
            let mut min_len = usize::MAX;
            for w in ways {
                // new_ways.append(&mut get_ways(&mut mem_ways, &w, &dmap, &ddist));
                let dest = w.chars().next().unwrap();
                let mut rem_ways = get_ways2(&mut mem_ways, 'A', dest, &w[1..], &dmap, &ddist);
                let len = rem_ways.iter().map(|s| s.len()).min().unwrap();
                if len <= min_len {
                    min_len = len;
                    for rw in rem_ways.into_iter().filter(|s| s.len() == min_len) {
                        new_ways.push(rw);
                    }
                }
            }
            ways = new_ways;
        }

        println!("==== You Ways");
        let mut you_ways = vec![];
        for w in ways {
            let dest = w.chars().next().unwrap();
            let min = get_ways2(&mut mem_ways, 'A', dest, &w[1..], &dmap, &ddist).into_iter().min_by_key(|s| s.len()).unwrap();
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
    println!("========= PART 2 ===============");
    let mut sum = 0;
    let (nmap, ndist) = paths(&nkeypad);
    let (dmap, ddist) = paths(&dkeypad);
    let mut mem_ways_useless: HashMap<String, Vec<String>> = HashMap::new();
    let mut mem_ways: HashMap<(char, char, String), Vec<String>> = HashMap::new();


    for code in input.lines() {
        println!("==== Code: {code}");
        let num_ways = get_ways(&mut mem_ways_useless, code, &nmap, &ndist);
     
        let mut ways = num_ways;
        for robot in 0..25 {
            println!("===== ROBOT {robot}");
            let mut new_ways = vec![];
            let mut min_len = usize::MAX;
            for w in ways {
                // new_ways.append(&mut get_ways(&mut mem_ways, &w, &dmap, &ddist));
                let dest = w.chars().next().unwrap();
                let mut rem_ways = get_ways2(&mut mem_ways, 'A', dest, &w[1..], &dmap, &ddist);
                let len = rem_ways.iter().map(|s| s.len()).min().unwrap();
                if len <= min_len {
                    min_len = len;
                    for rw in rem_ways.into_iter().filter(|s| s.len() == min_len) {
                        new_ways.push(rw);
                    }
                }
            }
            ways = new_ways;
        }

        println!("==== You Ways");
        let mut you_ways = vec![];
        for w in ways {
            let dest = w.chars().next().unwrap();
            let min = get_ways2(&mut mem_ways, 'A', dest, &w[1..], &dmap, &ddist).into_iter().min_by_key(|s| s.len()).unwrap();
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
