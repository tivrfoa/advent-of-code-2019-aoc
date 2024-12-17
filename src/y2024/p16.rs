use std::cmp::{Ordering, Reverse};
use std::collections::*;
use crate::util::*;

const N: usize = 0;
const E: usize = 1;
const S: usize = 2;
const W: usize = 3;

const DIRS: [(i64, i64); 4] = [
    (-1, 0),
    (0, 1),
    (1, 0),
    (0, -1),
];

pub fn p1_0(input: &str) -> usize {
    let mut min = usize::MAX;
    let d: usize = E;
    const M: usize = 1;
    const R: usize = 1000;
    let g = input_to_char_grid(input);
    let rows = g.len();
    let cols = g[0].len();
    let (mut r, mut c) = (rows - 2, 1);

    let mut mem: HashMap<(usize, usize, usize), usize> = HashMap::new();

    // state: (points, action, direction, row, col)
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, R, d, r, c)));

    while let Some(Reverse((p, a, d, r, c))) = pq.pop() {
        if let Some(v) = mem.get(&(d, r, c)) {
            if p >= *v { continue; }
        }
        mem.insert((d, r, c), p);
        if g[r][c] == 'E' {
            return p;
        }

        // 1) try to move
        let (dr, dc) = DIRS[d];
        let (nr, nc) = (ad(r, dr), ad(c, dc));
        if g[nr][nc] != '#' {
            pq.push(Reverse((p + M, M, d, nr, nc)));
        }

        // 2) rotate
        pq.push(Reverse((p + R, R, rotate(d, true), r, c)));
        pq.push(Reverse((p + R, R, rotate(d, false), r, c)));
    }

    panic!("Mission Failed");
}

fn rotate(d: usize, is_clockwise: bool) -> usize {
    if is_clockwise {
        match d {
            N => E,
            E => S,
            S => W,
            W => N,
            _ => panic!("{}", d),
        }
    } else {
        match d {
            N => W,
            W => S,
            S => E,
            E => N,
            _ => panic!("{}", d),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    points: usize,
    d: usize,
    r: usize,
    c: usize,
    tiles: HashSet<(usize, usize)>,
}

impl State {
    fn new(points: usize, d: usize, r: usize, c: usize) -> Self {
        State {
            points,
            d,
            r,
            c,
            tiles: HashSet::new(),
        }
    }

    fn move0(&self, nr: usize, nc: usize) -> Self {
        Self {
            points: self.points + M,
            d: self.d,
            r: nr,
            c: nc,
            tiles: self.tiles.clone(),
        }
    }

    fn rotate(&self, is_clockwise: bool) -> Self {
        Self {
            points: self.points + R,
            d: rotate(self.d, is_clockwise),
            r: self.r,
            c: self.c,
            tiles: self.tiles.clone(),
        }
    }
}

// Implement PartialOrd for State
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Implement Ord for State
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.points.cmp(&self.points)
    }
}

pub fn p1(input: &str) -> usize {
    let mut min = usize::MAX;
    let d: usize = E;
    let g = input_to_char_grid(input);
    let rows = g.len();
    let cols = g[0].len();
    let (mut r, mut c) = (rows - 2, 1);
    let mut mem: HashMap<(usize, usize, usize), usize> = HashMap::new();
    let mut pq: BinaryHeap<State> = BinaryHeap::new();
    pq.push(State::new(0, d, r, c));

    while let Some(mut s) = pq.pop() {
        if let Some(v) = mem.get(&(s.d, s.r, s.c)) {
            if s.points >= *v { continue; }
        }
        mem.insert((s.d, s.r, s.c), s.points);
        s.tiles.insert((s.r, s.c));
        if g[s.r][s.c] == 'E' {
            return s.points;
        }

        // 1) try to move
        let (dr, dc) = DIRS[s.d];
        let (nr, nc) = (ad(s.r, dr), ad(s.c, dc));
        if g[nr][nc] != '#' {
            pq.push(s.move0(nr, nc));
        }

        // 2) rotate
        pq.push(s.rotate(true));
        pq.push(s.rotate(false));
    }

    panic!("Mission Failed");
}

const M: usize = 1;
const R: usize = 1000;

pub fn p2(input: &str) -> usize {
    let mut min = usize::MAX;
    let d: usize = E;
    let g = input_to_char_grid(input);
    let rows = g.len();
    let cols = g[0].len();
    let (mut r, mut c) = (rows - 2, 1);
    let mut mem: HashMap<(usize, usize, usize), usize> = HashMap::new();
    let mut pq: BinaryHeap<State> = BinaryHeap::new();
    pq.push(State::new(0, d, r, c));

    let mut best_seats: HashSet<(usize, usize)> = HashSet::new();

    while let Some(mut s) = pq.pop() {
        if let Some(v) = mem.get(&(s.d, s.r, s.c)) {
            if s.points > *v { continue; }
        }
        if s.points > min {
            return best_seats.len();
        }
        mem.insert((s.d, s.r, s.c), s.points);
        s.tiles.insert((s.r, s.c));
        if g[s.r][s.c] == 'E' {
            assert!(min == usize::MAX || min == s.points);
            min = s.points;
            best_seats.extend(s.tiles);
            continue;
        }

        // 1) try to move
        let (dr, dc) = DIRS[s.d];
        let (nr, nc) = (ad(s.r, dr), ad(s.c, dc));
        if g[nr][nc] != '#' {
            pq.push(s.move0(nr, nc));
        }

        // 2) rotate
        pq.push(s.rotate(true));
        pq.push(s.rotate(false));
    }

    panic!("Mission Failed");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sample() {
        assert_eq!(7036, p1(SAMPLE));
    }

    #[test]
    fn test_p1_in() {
        assert_eq!(114476, p1(IN));
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(45, p2(SAMPLE));
    }

    #[test]
    fn test_p2_in() {
        assert_eq!(508, p2(IN));
    }
}


// -------------------------- INPUT

pub static SAMPLE: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

pub static IN: &str = "#############################################################################################################################################
#...#.#.....#.......#.......#...#.........#...#...........#.....#.#.......#.......#.....#...#.......#...#.#...............#...#.....#...#..E#
#.#.#.#.#.#.#.#.###.#.#.#####.#.#.#.#####.#.#.#.###.#####.###.#.#.#.#.#.#.#.###.#.#.###.#.###.#####.#.#.#.#.#.#.#########.#.#.#.###.#.###.#.#
#.#...#...#...#...#.#.#.......#...#.....#...#.#.#.#.#.........#...#.#...#.......#.#...#.#.#...#.....#.#...#...#.#.#.....#.................#.#
#.#.#.###.#######.###.#################.#.#####.#.#.#.###########.#.###.#.#######.###.#.#.#.#.#.#.###.#.#####.#.#.#.#.#######.#.#.#####.###.#
#.#.#...#...#...#.....#...............#.#.#...#...#.#.#.........#.#.#...#.......#...#.#.#.#.#.......#.#.#.....#.#.#...................#.#...#
#.#.#.###.#.#.#.#######.#######.###.#.#.###.#.###.#.###.#######.###.#.#.###.###.###.#.#.#.#.#####.#.#.###.###.#.#.#.#######.#####.#.#.#.#.#.#
#.#...#.....#.#.........#.....#...#.#.#...#.#.....#.....#.....#.....#.......#...#.#...#.#.#...#.....#.............#.#.#...#.....#.#.#.......#
#.#.###.#####.#####.###.#.###.#####.#.###.#.#################.#.#######.#####.###.#####.#.###.###.#.#.#.###.#.#.#.#.#.#.#.#####.#.#.#.#.#.###
#.#.....#.........#.....#.#.#.......#...#...#.......................#.....#...#.......#.#...#.....#.#.......#.#.#.....#.#...#...#...#.#.#...#
#.#.#########.###.#.###.#.#.###########.#####.###.#################.###.#.#.###.#######.###.#####.#.###.#####.#.#######.###.#.#######.#####.#
#.#.#...#...#...#...#...#.#...#.......#...#...#.....#...#.........#...#.#.#.#...#.............#...#...#.....................#.......#.......#
#.#.#.#.#.#.#########.#.#.#.#.#.###.#####.###.#.###.#.#.#.#######.###.#.#.#.#.#.#.#############.#.###.#.#.###.#.#.#########.#.###.#.#######.#
#.#.#.....#...#.......#...#.#...#...#.....#...#.#.....#...#...#.....#.....#.#.#.#.#.....#.......#...#.#.............#...#...#.#.#.#.#.....#.#
#.#.#.#######.#.#####.#.#####.###.#.#.#####.###.#.#########.#.#.#######.###.#.#.#.###.#.#.#######.###.#.#.#######.#.#.###.###.#.#.#.###.#.#.#
#...#.......#...#.....#.....#...#.#.#.....#...#.#.#...#...#.#.#.........#...#.#.#.#...#.#.....#.#...#.#.#.........#...#...#.................#
#.#########.#####.#.#.#.###.#####.#######.###.#.###.#.#.#.#.#.#.#########.###.###.#.###.#####.#.#.#.#.#.#########.###.#.###.###.#.#######.###
#.#...#.....#...#.#.#...#.#.#...#.......#...........#...#...#.#...#.....#.#.....#.#.#.....#...#.#.#.......#.....#.......#...#.............#.#
#.#.#.#.#####.###.#.#####.#.#.#.#######.#######.#.###########.###.#.###.#.#####.#.#.#.#####.###.#.#######.###.#########.#.###.#####.#.###.#.#
#...#...#...#.....#.#.....#...#.......#.....#...#...........#...#...#.#.#.....#...#.#.#.....#...#.#.....#.#.............#.....#...#.#...#...#
#.#######.#.#.#####.#.#.#####.#.#####.#####.#.###.#########.###.#####.#.#####.#.###.#.#.#####.###.#.###.#.#.#.#.#############.#.#.#.#.#.#.#.#
#.#.......#.#.....#.#.#.#.....#...#.#.......#...#.#.......#...#.#.....#...#.#.#...#.#.#...#...#.......#.#...#.#.......#...#...#.#...#.#...#.#
###.#######.#######.#.#.#.#####.#.#.#######.###.#.#.#.###.###.#.#.###.###.#.#.###.#.#####.#.###.###.#.#.#############.###.#.###.###.#.###.#.#
#...#.....#.....#...#.#.#.....#.#.#...#...#...#.#.#.#...#.#...#...#.#...#.....#.....#.....#...#.....#.#.#...#...#.....#...#.#...#...#...#...#
#.#####.#######.#.#.#.#######.#.#.#.#.#.#.#.###.#.###.#.###.#.#.###.#.###.###.#######.#####.#.###.#.#.#.#.#.#.#.#.#####.###.#.###.#####.###.#
#.#...#.....#...#...#.......#.#.#.#.#.#.#...#...#...#.#...#.#...#...#.....#...........#.....#.#...#.#.#...#...#.#...#...#...#...#.......#...#
#.#.#.###.###.###.#.#.#####.#.#.#.#.#.#.#####.#.###.#.###.#.#.###.#.#######.###.#############.#.#####.#########.#.#.#.#.#.#####.#####.###.###
#.#.#...#...#...#.#.#.#...#...#.#.#.#.#...........#.#...#...#...#.#...#...#...#...#.....#...#.#...#...#.#.....#.#.#.#.#...#...#...#...#...#.#
#.#.###.#.#.###.#.#.###.#.#.#.#.#.###.###.###.#####.###.#######.#.#####.#.###.###.#.###.#.#.#.###.#.###.#.#.#.#.###.#.#######.###.#####.###.#
#.#.#...#.#...#.#.#...#.#.#.....#...#.....#...#.....#.#...#...#.#.#.....#.....#...#.#.#...#...#...#.#.....#.#.#...#...#.....#...#.......#...#
#.#.#.#####.#.#.#####.#.#.#.#######.#######.###.###.#.###.###.#.#.#.#.#########.#.#.#.#######.#.###.###.###.#####.#.#.#.###.###.#########.#.#
#...#.#.....#.#.....#...#.#.#.....#.......#.#...#.......#...#.....#.#.#.........#...#.......#.#...#...#...#.......#.#...#.......#...#.....#.#
#.###.###.#.###.###.#.#.#.#.#.#.#.#######.#.#.#######.#.###.#######.#.#.#########.#.#.#####.#.###.###.###.###.#####.#.#########.#.#.#####.#.#
#...#.#...#.#.......#.#.#.#.#.#.#.......#.#.#...#...#.#.#...........#.#.....#...#.#.#.#...#.....#...#.......#.......#.#...#.#...#.#.#.....#.#
#####.#.#####.#######.#.#.#.###.#########.#.#.#.#.#.#.#.#.###########.#.#.#.#.#.#.#.###.#.#########.###.###.#.###.#.#.#.#.#.#.###.#.#.#####.#
#...#.#.....#.....#.....#.#...#.........#.......#.#...#.#.#...#.....#.....#.#.#...#.#...#.#.......#...#...#.#...#.#.#.#.#.#.....#.#...#...#.#
#.#.#.#.###.#####.#######.###.#.#.#####.#####.###.###.###.#.#.###.#####.###.#.#####.#.###.#.#####.###.###.#.###.#.#.###.#.#####.#.#####.###.#
#.#...#...#...#...#.......#.#.#...#...#.....#...#...#.#...#.#.#...#.........#.....#.....#.......#...#.#...#.#...#.#.....#...#...#...#.#.....#
#.#########.#.#.###.#####.#.#.###.###.###.#.###.###.###.###.#.#.###.#####.#########.###.###########.#.#.#.#.#.###.#########.#.#####.#.#.#####
#.............#...#.....#...#...#...#...#.....#.........#...#.#.....#.........#...#.......#.......#.#.#...#.#...#...#.......#...............#
#.#.###.#########.###.#.###.###.###.###.#.###.###.#####.#.###.#.#######.#.###.#.#.#########.#####.#.#.#####.###.#####.#.#############.###.#.#
#.#...#.........#...#.#...#...#.#.......#...#...#.........#.#.#...#.....#.....#.#...........#.#...#.#.....#.#...#.......#.......#.....#.#...#
#####.#.#######.###.#.###.#####.###.###.###.###############.#.###.###.###.#####.#############.#.###.#####.#.#.###.###.###.#####.###.#.#.###.#
#.....#...#...#...#.#...#.....#.......#.#...#.........#.....#.......#...#.....................#.#.....#.#...#.#...#...#...#...#.#...#.....#.#
#.#######.#.#.###.#.###.#####.#######.###.###.###.###.#####.#.#####.###.#############.#####.###.#.#.#.#.#####.#.#.#.###.###.#.#.#.#######.#.#
#.#...#...#.#...#...#...#.#...#.....#.#...#...#...#...#.....#.....#.....#...#.........#...#.#...#.#.#...#.#...#.#.#...#.....#.#.#.....#...#.#
#.#.###.###.###.#####.###.#.#.#.#####.#.###.###.###.#.#.###.#####.#####.#.###.#.#####.#.#.#.#.###.#.###.#.#.###.#.#########.###.#.###.#.###.#
#.#...#...#...#.....#.....#.#...#.............#.#...#...#...#.#...#...#...#...#.#...#...#...#.#.#.#...#...#...#.#...........#...#...#.......#
#.#.#.###.###.#.#######.###.###.#.#########.#.#.#.#.#.###.#.#.#.###.#####.#.###.#.#.#########.#.#.###.#######.#.#############.#####.###.#####
#.#.#...#...#.#.........#...#...#.#.......#...#...#.#...#.#.#.#.#.....#...#...#...#.#.......#.#.....#...#...#.#.........#.....#.....#.......#
#.#.#.#####.#.#######.###.#######.#.#####.###.###.#.###.#.#.#.#.#####.#.#####.#####.#.#####.#.#.###.###.#.#.#.###########.#####.#####.#.###.#
#.#.............#...#.#...........#.#.....#...#...#...#.#...#.#.....#.#.#...#...#.#...#...#...#...#.#.#.#.#.#.#.......#...#...#.#.........#.#
#.###.#.#####.###.#.###.###.#######.#.#######.#.#####.#.#.###.#####.#.#.#.#####.#.###.###.#######.#.#.#.#.#.#.#.#####.#.###.#.#.#.###.#.#.#.#
#...#...#...#.#...#...#.#...#.......#...#...#.#.....#.#.#.........#.......#.....#.#...........#...#...#...#...#...#.#...#...#.#.#...#...#.#.#
#.#.#.###.#.###.###.#.#.#.###.#########.#.#.###.###.#.#.###########.#####.#.###.#.#.#.#######.#.#####.###########.#.#######.#.#.#######.#.#.#
#.#.............#.....#.#.....#.......#...#...#.#...#...#...#.....#.....#.#.#...#...#.....#...#.#.....#...#...............#.#.#.......#.#...#
#.#.#.#.###.#.###.#####.#.#.###.#####.#######.###.#.#####.#.#.###.#####.###.###.#.#######.#.###.#.#####.#.#.###########.#.###.#######.#.###.#
#.#.#...#...#.#.#.#.......#.#...#.......#...#.....#.#...#.#...#.#.#...#.#...#...#.......#.#.....#.......#.#...#.......#.#.....#.....#.......#
#.#.#.###.###.#.#.#####.###.#.###.#.###.#.#######.###.#.#.#####.#.#.#.#.#.###.###.#####.#.#############.#.#.#.#.#.###.#.#.#####.#.#####.#.#.#
#.#...#.......#.#.#...#.....#.#...#...#.#.........#...#...#.....#...#...#.#.......#...#.#...#...........#.#.#.#.#.#...#.#.#.................#
#.###.#########.#.#.#.#####.#.#.###.###.#.#########.###########.#.#####.#.#.#######.#.#.###.#.#######.###.###.#.#.#.###.###.#######.#.#.#####
#.......#.......#...#.....#.#.#.#...#...#...#...#...#...........#.....#.#.#.#.......#...#.#...#.....#...#.....#.#.#...#...#.................#
#.#.#.#.###.#.#######.###.###.#.#.###.#####.#.###.###.#########.#.#####.#.#.#.#.#######.#.#####.#.#.###.#########.#.#.###.#.###.#.#########.#
#.#...#.....#.......#...#.....#.#.....#.......#...#...#...........#.....#.#.#.#.#.......#.........#...#...........#.#.....#.#...#.......#...#
#.###.###.#.#####.#.#.#.#######.#######.#######.###.###.###########.#####.#.###.#####.###.#########.###########.###.#####.#.#.#######.#.#.###
#...#.#...#.#...#.#.#.#.......#...#.....#.....#.#...#...........#.......#.#...#.#...#.....#.......#.#...........#.......#.#.#...#...#.#.#.#.#
###.#.#.###.#.###.#.#####.#######.#######.###.#.#.#.#########.###.#######.###.#.#.#.#####.#.#####.#.#.###############.###.#.###.###.#.###.#.#
#.#...............#.#...#.......#.......#...#...#.#...#.....#.....#.#...#...#...#.#...#.#.#...#.#.#.#.#.....#.......#.#...#...#...#.#...#...#
#.#.#.###.#.###.###.#.#.#.###.#.#######.###.###.#.###.#.###.#.#####.#.#.###.#####.###.#.#.###.#.#.###.#.#.#.#####.#.#.#.###.#.#.#.#.###.###.#
#.....#...#.#...#.....#.#.#...#.......#.#...#.....#...#...#.....#...#.#...#.#.....#...#...#...#.#.#...#.........#.#...#...#.#...#...#.#.....#
#######.#####.#########.###.#########.#.#.###.#####.###.#.#####.#.###.###.#.#.#####.###.###.###.#.#.#####.#####.#.###.###.#.###.###.#.#######
#.....#.#.....#.....#.#.....#.....#...#...#...#.....#...#.....#...#...#...#.......#.#.....#.#.....#.#.....#...#.#...#...#...#.#.#...#...#...#
#.#.#.#.#.#####.###.#.###.#.#.###.#########.###.###.#.###.###.###.#.#######.###.###.#.#####.#.#####.#.#####.#.#.###.###.#####.#.#.#####.#.#.#
#...#...#.#...#.#.#.#.....#.#.#.#...#.....#...#.#...#...#.#...#...#.#.........#.#...#.#.....#...#...#.#...#.#.......#...#.....#.#.#...#...#.#
###.#.#.#.#.#.#.#.#.###.###.#.#.###.#.#.###.###.#.###.###.#.###.###.#.#.#.#.#.###.#####.#######.###.#.#.#.#.#.#########.#.#####.#.#.#.#####.#
#...........#.....#...#.....#...#...#.#.....#.....#.......#.#...#...#.#.#.#.#.#...#.....#.......#...#...#.#.#.........#.#.....#.#.#.#.......#
#.#.#.#.#########.###.#.#######.#.###.#######.###.#.###.###.###.#.###.#.###.#.#.#####.###.#######.###.###.#.#####.###.#.###.#.#.#.#.#.#####.#
#.#...#.....#.....#.#.#.#.....#.#.#...#.......#...#.#.#.#.#...#.#...#...#...#...#.....#...#.....#.#.#...#.#.#.......#.#...#.#.#.#...#.......#
#.#.#.#.#.#.###.#.#.#.###.###.#.#.#.###.###.#.#.###.#.#.#.###.#.###.#.#.#.#########.###.###.#.#.#.#.#.#.#.#.#####.#.#.#.#.###.#.#####.#.#####
#.#.......#.....#...#.......#.#.#...#...#.....#...#...#...#.#.......#.#...#.........#...#...#.#...#...#...#...#...#.#.#.#.....#.#.....#.....#
#.#.#.#.###.#######.#########.#.#####.#.#######.###.#####.#.#########.###########.###.#######.#####.###.#####.#.###.#.#.###.###.#.###.#.#.#.#
#.....#.....#.....#...#.....#.....#.#.#.............#.....#.......#.......#.......#.#.#.......#...#.#.#.....#.#.#...#.#...#.#...#...#.#.#.#.#
#.#.#.#.#####.###.#####.###.#####.#.#.#.#.###########.#####.#####.#####.#.#.#######.#.#.#######.#.#.#.#####.#.#.#####.#.###.#.#####.#.#.#.#.#
#...#.#.....#.#.#.#.....#.....#.....#.#.#...#.......#.....#.....#.....#.#.#...#...#.#.#.#.......#.#.#.....#.#...#.....#.#...#.....#...#...#.#
#.#.#.#.###.#.#.#.#.#########.#######.#####.#.#.#.#######.#.#.#.#####.###.#.#.#.#.#.#.#.#.#########.#.###.#.###.#.#####.#.#######.#.#.#####.#
#...#.#.....#.#...#.#.......#.......#.....#...#...#.........#.#.....#...#.#.#...#.#.#.#.#.........#...#.#.#...#.#.#.............#.#.#...#...#
#####.#.#####.#.###.#.#####.#######.#####.#####.###.#########.#.###.###.#.#.#####.#.#.#.#.#######.#####.#.###.#.#.###.###########.#.#.###.###
#...#.#.#...#.#.#.......#...#.#...#...........#...#.......#...#.#...#...#.#...#...#.#...#.#.....#.......#.#.#...#...#.#...........#.#...#.#.#
#.#.#.#.#.###.#.#########.###.#.#.#.#############.###.###.#.###.#.###.###.###.#.###.#####.#.#####.#.#.###.#.#######.###.###########.#.#.#.#.#
#.#...#.#.#...#.#.........#.#...#...#...........#.#...#...#.#...#.........#...#.#...#...#.#...#...#.#...#.#.#.......#...#...........#.#.#.#.#
#.###.#.#.#.###.#.#########.#.#######.#####.###.#.#.#.#.###.#######.#######.###.#.#.#.#.#.###.#.#####.#.#.#.#.#######.#.#.#####.#.#.#.#.#.#.#
#...#.....#.#.....#.....#.............#...#.#...#...#.#.#.#.........#.......#...#.#...#.#.#...#...#...#.....#...#...#.#...#...#...#...#.#...#
#.#.#.#.###.#######.###.#####.#####.###.#.#.#.###.#####.#.#.#######.#######.#.#########.#.#.#####.#.###########.#.#.#.#.#.#.#####.###.#.###.#
#.#.......#.........#.#...#...#.....#...#...#.....#.....#.#.#.....#.#.....#.#.......#...#.#.#.....#.#.........#...#.#.#.#.#...#.....#.#...#.#
#.#######.#.#########.###.#.#.#.#####.###.#######.#.#####.#.#.###.#.#.#.#.#.#######.#.###.#.#.###.#.#.#######.###.#.#.#.#.#.#.#.#####.#.###.#
#.......#.#.......#...#.#.....#.......#...#.....#.#.#.#.....#.#...#.....#.#.#.....#.#.......#...#.#.#.#.....#...#.#.#.#...#.#.#...#...#.....#
###.#####.#######.#.#.#.#####.#.###########.###.###.#.#.#####.###.#######.#.#.###.#.#######.###.###.#.#.#######.###.#.#.#.#.#.#.###.#####.#.#
#...#.....#.......#.#.....#...#.#...........#.#.#...#...#...#...#.........#.#...#.#.#...#...#...#...#.#.......#.....#.#.#.#.#...#...#...#.#.#
#.###.#####.#.#####.#####.#.###.#.###.#######.#.#.#######.#.###.###########.###.#.#.#.#.###.#.#.#.###.#######.#.#####.#.#.#####.#.#####.#.#.#
#...#.#.....#...#...#...#.#.#...#.....#.......#.#.#.......#.....#.#.......#...#.#.#...#...#.#.#.#...#.#.....#...#.....#.#.#...#.#.......#.#.#
###.#.#.###.###.#.#.#.#.###.#.#######.#####.#.#.#.#.#.#.#########.#.###.#####.###.#######.#.#.#.#.###.#.###.#####.#######.#.#.#####.#####.#.#
#.#.#.#.#...#...#.#...#...#.....#...#.#.....#.#.....#.#.#.....#.......#...........#.....#.#...#...#...#.#.#...#...#.....#.#.#.....#.#.....#.#
#.#.#.#.#.###.###########.#####.###.#.#.#####.#######.###.###.#.#################.###.###.###.#####.###.#.###.#.###.###.#.#.#####.#.#.#.###.#
#...#.#.#...#...#.....#...#...#...#.#.#...#.#.......#.#.....#.#.#...#.......#...#...#...#...#.....#...#.#.......#...#...#...#...#...#.#.#...#
#####.#.###.###.#.###.#.###.#.###.#.#.###.#.#######.#.#.#####.#.#.#.#.#####.#.#.###.###.###.#####.###.#.###.#####.#.###.#.###.#.#####.###.###
#.....#...#...#.#...#...#...#.#.#.#.#.....#.......#.#...#.#...#...#.#.#.....#.#...........#.#...#...#.#...#.......#...#.#.#...#.....#...#.#.#
#.#######.#.###.###.#####.###.#.#.#.###########.#.#.#.###.#.#######.#.#.#################.#.#.#.#####.###.#.#########.#.#.#.#####.#####.#.#.#
#.......#.#.........#.....#...#.......#.......#.#.#.#.....#.....#...#.#.#.............#...#...#.....#.....#.#.....#...#.#...#.....#.....#.#.#
#.#####.#.###########.#.#.#.#####.#####.#.###.#.#.#.#######.###.#####.#.#.###########.#.#######.###.#.#####.#.###.#.###.#####.#####.#####.#.#
#.#...#.#.......#.....#...#.....#.#.....#...#.#.#.#...#...#...#.#.....#.#.#.......#...#...#...#.#...#...#...#.#...#...#...#.......#...#.....#
#.#.#.#.#######.#.#############.###.#######.#.#.#.###.#.#.#####.#.#####.#.#.#.###.#.###.###.#.###.###.#.#####.#.#####.###.###########.#.###.#
#...#.#...#.....#.....#.......#.....#.......#.......#...#.#.....#.#...#.....#...#.#...#.#...#.....#.....#.....#.#...#.#.........#.....#...#.#
#####.###.###.#######.#.###.#.#######.###.#######.#.#####.#.#####.#.###.#######.#.###.#.#.###########.###.#####.#.#.#.#########.#.#.#.###.#.#
#...#...#...#.#.....#.#.#...#...#.....#...#...#...#.#...#.#.#.....#.............#...#.#.#...........#.....#...#.....#.....#.....#...#.#.#...#
#.#.###.###.###.###.#.###.#####.#.#####.#.#.#.#.#.###.#.#.#.###.###.#####.###########.#############.#########.#####.#.#.#.#.#.#.###.#.#.###.#
#.#...#.#.#.......#.#.....#.....#.....#.#.#.#...#...#.#.#...#...#.....#.#.#.......#...#.......#.....#.............#.#...#.#.#...#...#.....#.#
#.#####.#.#######.#.#######.#####.###.#.#.#.#######.#.#.#####.#######.#.#.#.#####.#.###.#####.#.#####.#####.#######.#.#.#.#.#.#.#.#.#####.#.#
#.......#...#...#.#.......#.....#.#.....#.#...........#.............#...#.....#.#...#.#...#...#.....#.....#.#...#...#...#.#.#.#.#.#.......#.#
#.#######.#.#.#.#####.#.#######.#.#.#.#.#########.#####.#.#####.###.#.#######.#.#####.#.#.#.###.###.#.###.###.#.#.#######.#.#.#.#.#.#######.#
#.#.......#.#.#.....#.......#.....#...#.........#.#.#...#.#.........#.......#.#.....#...#.#.#.....#.#...#.....#.#.#.......#.#.....#.#...#...#
#.#.#######.#.#####.#######.#.###.#.#########.#.#.#.#.###.#.###########.###.#.#.###.#####.#.#######.#.#########.#.#.#######.#####.#.#.###.###
#.#.......#.#.#.#...#...#...#...#.#.....#.....#.#...#.#.....#...........#.#.#...#.......#.#...#.....#.#.......#.#...#...#...#.....#...#...#.#
#.#########.#.#.#.###.#.#.#####.###.###.#.#.#.#####.#.#.#####.#########.#.#.###########.#.###.#.#####.#.#####.#.#####.###.###.#####.###.###.#
#.#.........#.#.#...#.#.#.#...#...#.....#.#...#.....#.#.....#.#...........#.........#.#.#.#.#...#...#.#.....#...#...........#.#...#.....#...#
#.#.#######.#.#.###.#.#.#.#.#.###.###.#.#.#####.#####.#.###.#.#.#########.#########.#.#.#.#.#####.###.#####.###.#.###.#####.#.#.#.#.#####.#.#
#...#...#...#.#.......#.#.#.#.....#...#.#.....#...#...#.#.#.#.#.........#...#.......#...#.#.#.........#...#...#...#...#...#...#.#...#.....#.#
#####.#.#.###.#.#######.#.#.#######.###.#####.#.#.#.###.#.#.#.#####.#######.#.#######.###.#.#.#########.#.###.###.#.###.#.#.#.#.###.#.#####.#
#.#...#.#.#...#.#...#.#...#.#...#.....#.....#.#.#.......................#...#.#...#...#...#...#.........#...#.....#.#...#.#...#...#...#...#.#
#.#.###.#.#.#####.#.#.###.#.#.#.#####.###.#.#.#.#.#.#.###.#####.#.###.#.#.###.#.###.###.#####.#.###.#######.#.###.#.#.###.#######.#.###.###.#
#.#...#...#.#.....#.#...#.#...#...#...#.#.....................#.....#.#.#...#.#.#...#...#...#...#...#...#...#.......#.#.#.........#...#.#...#
#.###.#.###.#.#####.#.###.#######.#.###.#.###.###.#.###.#.###.#######.#.###.#.#.#.###.###.#.#####.###.#.#.#######.#.#.#.#######.###.#.#.#.###
#...#...#...#.#.#...#...#.......#.#.....#.......#.#...#...#...................#.....#...#.#.............................#.........#...#.#...#
###.#.#.#.###.#.#.#####.#######.#.#########.###.#.###.#####.###.###.#####.#.#.###.#.###.#.#########.#.#####.#####.#.###.###.#####.#.###.###.#
#...#.#.#.....#.#.....#.....#...#.....#...#.#...#.#.......#...#.#...#...#.#.#.....#.....#.#.....#.....................#...#.#...#...#.....#.#
#.###.#.#######.#####.#####.#.#######.#.#.#.#####.#######.###.#.###.#.#.#.#.#####.#######.#.###.#.#.###.#####.#.#.###.###.#.###.#####.###.#.#
#.#...#...........#.#.....#.#.#...#...#.#.#...#...#.....#...#.#...#...#.#...#.....................#...#...#...#.....#.#...#...#.....#.#.#...#
#.#.###.#####.###.#.#####.#.#.#.#.#.###.#.###.#.###.###.#####.###.#.###.###.#.###.#.#.#.#.###.#.#.###.###.###.###.###.#.###.#.#.###.#.#.#####
#.#.....#.......#.....#.#.....#.#.#.#...#.....#.#...#.#.....#...#.#.#.#.#...#.....#...#.#...#.#.#.#.........................#.#...#.#...#...#
#.#######.#.#.#######.#.#.#####.#.#.###.#####.#.#.###.#####.###.#.#.#.#.#.#####.#.#####.###.#.#.#.#.###.###.#####.#######.###.#.#.#####.###.#
#S........#.............#.......#.......#.....#...........#.......#...#.........#.....#.......#...#.......#...................#.#...........#
#############################################################################################################################################";