use std::{cmp::Ordering, collections::{BTreeSet, HashMap, HashSet, VecDeque}, f64::consts::PI};

pub fn p1(input: &str) -> usize {
    let map = process_input(input);
    let asteroids = find_asteroids(&map);
    let center = find_best_monitoring_station(&asteroids);
    println!("Best station: {:?}", center.0);

    center.1
}

pub fn p2(input: &str) -> usize {
    let map = process_input(input);
    let asteroids = find_asteroids(&map);
    let (center, _) = find_best_monitoring_station(&asteroids);
    let ret = vaporize_asteroids(center, &asteroids);
    dbg!(ret);
    ret
}

// Convert input string to 2D vector of characters
fn process_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

// Find all asteroid positions
fn find_asteroids(map: &[Vec<char>]) -> HashSet<(usize, usize)> {
    map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == '#')
                .map(move |(j, _)| (i, j))
        })
        .collect()
}

// Greatest Common Divisor function (Euclidean algorithm)
fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

// Reduce function to simplify vector directions
fn red(x: isize, y: isize) -> (isize, isize) {
    assert!(x != 0 || y != 0);
    let g = gcd(x, y);
    (x / g, y / g)
}

// Find the asteroid with the most visible asteroids
fn find_best_monitoring_station0(asteroids: &HashSet<(usize, usize)>) -> (usize, (usize, usize)) {
    let mut max = 0;
    let best_location = asteroids
        .iter()
        .max_by_key(|&&a| {
            let visible_asteroids = asteroids
                .iter()
                .filter(|&&b| b != a)
                .map(|&b| {
                    let (dx, dy) = (b.0 as isize - a.0 as isize, b.1 as isize - a.1 as isize);
                    red(dx, dy)
                })
                .collect::<HashSet<_>>();

            max = max.max(visible_asteroids.len());
            visible_asteroids.len()
        });

    dbg!(max);
    (max, best_location.cloned().unwrap())
}

// Find the asteroid with the most visible asteroids, return both the asteroid and count
fn find_best_monitoring_station(asteroids: &HashSet<(usize, usize)>) -> ((usize, usize), usize) {
    asteroids
        .iter()
        .map(|&a| {
            let visible = asteroids
                .iter()
                .filter(|&&b| b != a)
                .map(|&b| {
                    let (dx, dy) = (b.0 as isize - a.0 as isize, b.1 as isize - a.1 as isize);
                    red(dx, dy)
                })
                .collect::<HashSet<_>>()
                .len();
            (a, visible)
        })
        .max_by_key(|&(_, count)| count)
        .unwrap()
}

fn vaporize_asteroids(station: (usize, usize), asteroids: &HashSet<(usize, usize)>) -> usize {
    let mut dirs: HashMap<(isize, isize), Vec<(isize, (usize, usize))>> = HashMap::new();
    let mut dirs_sorted: HashMap<(isize, isize), VecDeque<(isize, (usize, usize))>> = HashMap::new();

    for &b in asteroids {
        if b == station {
            continue;
        }
        let (dx, dy) = (b.0 as isize - station.0 as isize, b.1 as isize - station.1 as isize);
        let norm = dx.pow(2) + dy.pow(2);
        dirs.entry(red(dx, dy)).or_default().push((norm, b));
    }

    // Sort asteroids by distance for each direction
    for (k, asteroids) in dirs.iter_mut() {
        asteroids.sort();
        dirs_sorted.insert(*k, asteroids.clone().into_iter().collect());
    }

    // Sort directions by clockwise angle from up
    let mut s_dirs: Vec<(isize,isize)> = dirs.keys().cloned().collect();
    s_dirs.sort_by(|a,b| {
        (-(a.1 as f32).atan2(a.0 as f32)).total_cmp(&-(b.1 as f32).atan2(b.0 as f32))
    });

    let mut res = Vec::new();
    while res.len() < 200 {
        for &i in &s_dirs {
            // if let Some(asteroid) = dirs.get_mut(&i).and_then(|v| v.pop()) {
            if let Some(asteroid) = dirs_sorted.get_mut(&i).and_then(|v| v.pop_front()) {
                res.push(asteroid);
                if res.len() == 200 {
                    break;
                }
            }
        }
    }

    let (_, (x, y)) = res[199];
    100 * y + x
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(340, p1(IN));
    }

    #[test]
    fn test_p2() {
        assert_eq!(2628, p2(IN));
    }
}





// -------------------------- INPUT



pub static IN: &str = ".#....#.###.........#..##.###.#.....##...
...........##.......#.#...#...#..#....#..
...#....##..##.......#..........###..#...
....#....####......#..#.#........#.......
...............##..#....#...##..#...#..#.
..#....#....#..#.....#.#......#..#...#...
.....#.#....#.#...##.........#...#.......
#...##.#.#...#.......#....#........#.....
....##........#....#..........#.......#..
..##..........##.....#....#.........#....
...#..##......#..#.#.#...#...............
..#.##.........#...#.#.....#........#....
#.#.#.#......#.#...##...#.........##....#
.#....#..#.....#.#......##.##...#.......#
..#..##.....#..#.........#...##.....#..#.
##.#...#.#.#.#.#.#.........#..#...#.##...
.#.....#......##..#.#..#....#....#####...
........#...##...#.....#.......#....#.#.#
#......#..#..#.#.#....##..#......###.....
............#..#.#.#....#.....##..#......
...#.#.....#..#.......#..#.#............#
.#.#.....#..##.....#..#..............#...
.#.#....##.....#......##..#...#......#...
.......#..........#.###....#.#...##.#....
.....##.#..#.....#.#.#......#...##..#.#..
.#....#...#.#.#.......##.#.........#.#...
##.........#............#.#......#....#..
.#......#.............#.#......#.........
.......#...##........#...##......#....#..
#..#.....#.#...##.#.#......##...#.#..#...
#....##...#.#........#..........##.......
..#.#.....#.....###.#..#.........#......#
......##.#...#.#..#..#.##..............#.
.......##.#..#.#.............#..#.#......
...#....##.##..#..#..#.....#...##.#......
#....#..#.#....#...###...#.#.......#.....
.#..#...#......##.#..#..#........#....#..
..#.##.#...#......###.....#.#........##..
#.##.###.........#...##.....#..#....#.#..
..........#...#..##..#..##....#.........#
..#..#....###..........##..#...#...#..#..";
