use std::collections::*;
use crate::util::*;

#[allow(dead_code)]
fn parse(input: &str) -> usize {


    0
}

pub const M: [(i32, i32); 4] = [
    (-1, 0), // N
    (0, 1),  // E
    (1, 0),  // S
    (0, -1), // W
];

pub const N: usize = 0;
pub const E: usize = 1;
pub const S: usize = 2;
pub const W: usize = 3;

const T: [usize; 4] = [E, S, W, N];

fn get_start_pos(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, row) in grid.it() {
        for (j, v) in row.it() {
            if *v == '^' {
                return (i, j);
            }
        }
    }
    panic!("didnt find start position");
}

pub fn p1(input: &str) -> usize {
    let mut visited = HashSet::new();
    let grid = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let (mut r, mut c) = get_start_pos(&grid);
    let mut dir = N;

    loop {
        visited.insert((r, c));

        loop {
            let ir = r as i32 + M[dir].0;
            let ic = c as i32 + M[dir].1;
    
            if ir < 0 || ir == rows || ic < 0 || ic == cols {
                return visited.len();
            }

            if grid[ir as usize][ic as usize] == '#' {
                dir = T[dir];
            } else {
                (r, c) = (ir as usize, ic as usize);
                break;
            }
        }
    }
}

fn is_loop(start_pos: (usize, usize), grid: &Vec<Vec<char>>) -> bool {
    let mut visited = HashSet::new();
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let (mut r, mut c) = start_pos;
    let mut dir = N;

    loop {
        if !visited.insert((r, c, dir)) {
            return true;
        }

        loop {
            let ir = r as i32 + M[dir].0;
            let ic = c as i32 + M[dir].1;
    
            if ir < 0 || ir == rows || ic < 0 || ic == cols {
                return false;
            }

            if grid[ir as usize][ic as usize] == '#' {
                dir = T[dir];
            } else {
                (r, c) = (ir as usize, ic as usize);
                break;
            }
        }
    }
}

fn get_path(grid: &Vec<Vec<char>>) -> ((usize, usize), HashSet<(usize, usize)>) {
    let mut visited = HashSet::new();
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let start_pos = get_start_pos(&grid);
    let (mut r, mut c) = start_pos;
    let mut dir = N;

    loop {
        visited.insert((r, c));

        loop {
            let ir = r as i32 + M[dir].0;
            let ic = c as i32 + M[dir].1;
    
            if ir < 0 || ir == rows || ic < 0 || ic == cols {
                return (start_pos, visited);
            }

            if grid[ir as usize][ic as usize] == '#' {
                dir = T[dir];
            } else {
                (r, c) = (ir as usize, ic as usize);
                break;
            }
        }
    }
}

/// Strategy:
///     - find the p1 path
///     - need to check for every poosition in that path (excluding start position)
///     - count how many of them become a loop
pub fn p2(input: &str) -> usize {
    let mut qt = 0;
    let mut grid = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
    let (start_pos, mut path) = get_path(&grid);
    path.remove(&start_pos);

    for p in path {
        grid[p.0][p.1] = '#';
        if is_loop(start_pos, &grid) { qt += 1; }
        grid[p.0][p.1] = '.';
    }

    qt
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sample() {
        assert_eq!(41, p1(SAMPLE));
    }

    #[test]
    fn test_p1() {
        assert_eq!(5534, p1(IN));
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(6, p2(SAMPLE));
    }

    #[test]
    fn test_p2() {
        assert_eq!(2262, p2(IN));
    }
}


// -------------------------- INPUT



pub static SAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

pub static IN: &str = "......##...#...#....#.......#....................##............#.#..#.......#.........................................#...........
..................................#.............#................................#..........##..................#.................
....#......................#................#...........................................#....................................#....
............#..............#...#...#...............#..........#.#....#..........................#....##...........................
....#.............#.....#.....................................................................#...#..........................#....
.................#............#......................#.................#.............#..................#.........................
................#...............#....................#...#..#...#.#.....................................................#.........
..............#.#...............................................................#....................#.......#....................
.............#...#..#............................#..............#..#.........................................#....................
.........................#.........#..#.......#...#..............#..#..............................#.........#............#.......
#...............................................................................................................#.................
..........................#.....#.........................#...................#.......#.........................#.................
...........#...........#........#........................#..........#.#.................#........#............#...............#...
..#..#................#...................................................#..................................................#....
.....#............................#..........................................................................#....................
................#.....#...........................#..........................................#.........#........................#.
..#...........................................##............#.........#..........................#.......................#.....#..
.....................#.............#.....#.......#.##.....#......................................#.#..#.#.........................
.......#...##.....#..........#...............#....#..........................#..............................................##....
..#.......#..................##..............#..........................#.#....#....#....................#.#.#...............#....
....................#.......#.......#............#..............#.......#.........#...#.....#........#.#..........................
...............................................#.....#........................................#..#.........#..........#...........
............#...#.............................................................................#.......##..........................
##..........................#...........................#..............................#..........................................
..............................................#.............................................#.................................#...
.............#..............#................##...........#...............#.......................................................
......#............#.......#..#....#.............................................................................#.#..............
..#..##..#..................#...............................................##......#........................................#....
..#...............................#.#........#................................................#.....#........#.#..................
.........#.......#.....................................#............................#..............................#..............
..........................................................................................................#.....................#.
....................#......................................#..........................#.........................##................
........#..............................................................................#.#..................#..................#..
........................................................^..#...................#....#..............##.............................
..#.#........................................................#..#..................#.....................................#....#...
.........................#....#............................#......................................................................
.#......#.............................................................................................#....................###....
...........#...#.#............#........................................................................#..........................
......#..........#.............................#.#.................................#...#.............#.#........................#.
...............................................#..........#.......#.........#........................................#.#..........
.................#.....................#.......#..................................................................................
.#...............#.........#...................#..............#........................#.....#....................................
...........#..............#..................................................................#...................................#
.................#.............#...........#.......................................#......................#......#..........#.....
........#..#.................#..........#....................................................#.....#...........#.................#
.....................#..#.................#......#......#..#..#..........#........................#...........................##..
.................#........#....#.................................................#.#.........................#............#..#....
.....................#...............##...................#.....#..#............................#.#.............#.................
.#..#......##...............#.......#..........................................................................#......#...........
#..#...................................#.#................#.....#.................................#...........................#...
..............#............................#............#...#...#.............#..........#............#...#............#..........
..........##.................#................................#...........#.#....#...............................................#
......#...........#.............#..........#.............#............#...................#.......................#...............
..............#.......#...........................................................................................................
...#................................................................#...............................#....#...............#........
...#..##................#..............................................................................#..........##.#............
.#........#..........#..................#......................#...........#......................................................
....#.......................#...#...........................................#..................#...............................#..
.............................#...........#..........#.................#.....................................................#.....
...........#.........#............#...............................#................#..............#.........................#.....
..........#........#.....#.....................................#.................#.....................................#..#.......
...#.......#...................................................................#.................#................................
.#....#........#..............................#........#.....#........#..#...#.#......................#..........................#
.....................................................................#....................#.......................................
.............................#..............................#.#....................................................#............#.
..#.....#.......#.......#......................#.#.#.........................................................#.....#........#.....
...............#..........#..#....#..................#..........................................#.................................
............#...................#................#.............................................................................#..
........#...#......................................##....................#..................#..........##............#...#........
......................#......#......#.#.........................................................................#.............#...
........#......................................................................................#.................#................
.................................................#....................#..............##.......................................#..#
.............#..........................................................#.................#..........................#....#......#
......................#.....................#............................#..................#...........#.........................
#..............#...........................#...#.#................................................................................
.......#............................#...........................................................#...............#.................
........................#..................................................................................#......................
...................#..........................#.........#........#...........................#....................................
.............#............#.......................................................................................##....#.........
................#.........#.....................................................................................................#.
........#.............................#.................................#.......##..............#............................#....
#........#.....#.....#.......#...............................#....................................................#..............#
........................#.............................#.....#...........#........#............#...................................
..................#...........#...#.............................................................................#...............#.
...................#..................#...................#..............................#....#...........#..............#........
...............#................#.#....................#.........#.........#.............#.........#......#..................#....
..................#.............................#...........................#....#....#......#..#.#....................#..........
...#.......................................#...............................................................#................#.....
...................#.........................................................#....................................................
.................#..............................#.................................................................................
.............#.......#...#....................................................#........#....................................#...#.
..............#...................#.#..........#....................................................#....#...#.##....#............
........#..#...........................................#.........#...#...............#.........................#..................
#.....#...........................................#.................................................#.............................
#...........................................#..........#..........................................#...............#...............
............................#.................................................#...#...............................................
.#...................#.......................................................................................#......#...##..#.....
.....#..#...............#........#....#.........#........................#.............#..........................................
...................#................#........................................#.#..................................................
......#.#...........................#...................#..........#..........#......#............................................
..........................#.#..............#............#.#.......................................................................
...#..#........................#..............................................#..#...#.....#........#.............................
..........................#.....................#.........................................................#............#.#........
............................#.....................#.............#.................................................................
..........................................#..#.................................................................#......#.#.#.......
..........##......................................#................................##....................#..#.....................
..............#.......................................................#...........................................................
..........#........#........................................................................................#........#...#........
......................#.#...................#...............#.....#......................#.....#.......#.......#..........#.......
......#..#....#..................#.........................................#......................................................
...............#.....#..#..........................................##.........................................##.............#....
.....................#..................#..................................#...................#............................#.#...
..#................#..#................................................#...........#..........#.............#...................#.
.......#.............#.......................................................................#...#.........................##..#..
...#...................#..................#.............#..#.........................................#........##..................
.................................#...........#........#.....................#.........................#..#........................
.......#..................#..................................#......................................#.#....................#......
..................................................................................#...........#....#.......#......#..........#....
........##.........................................................#.......................#........#............#................
...#...............##.#..................#....................................#.............#.................#...................
..................##.........#...................#......#................#......................................................#.
.#....................#............#..............#.............#.........#........................................#......#..#....
...............#...........................................................#........................#.............#...............
#............................#...................#...............................##..#...............#..........................#.
................#.............#..........#.........#..........................#....#..............................#..............#
...................................#.........................................#....................................................
....#................................................................#.#..#..........#....#.............................#.........
...........#.............#..........#........#...#...#........#.............#.......................................##............
.........#.......................#..........#....#.......................................#.....#...#.#..........#.................
.......................#...........#.......#......#.#...............................................#.................#.#.........";
