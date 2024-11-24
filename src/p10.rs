use std::{cmp::Ordering, collections::{HashMap, HashSet}, f64::consts::PI};

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
    let ret = vaporize_asteroids(&asteroids, center);
    dbg!(ret);

    let ans = ret.0 * 100 + ret.1;
    ans
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

fn vaporize_asteroids(station: (usize, usize), asteroids: &HashSet<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut asteroid_map: HashMap<(f64, f64), (usize, usize)> = HashMap::new();
    let mut angles: Vec<f64> = Vec::new();

    for &asteroid in asteroids {
        if asteroid == station {
            continue; // Skip the station itself
        }
        let (dx, dy) = (asteroid.0 as f64 - station.0 as f64, asteroid.1 as f64 - station.1 as f64);
        let angle = (dx.atan2(dy) + 2.0 * PI) % (2.0 * PI); // Angle in radians, adjusted for clockwise rotation
        let distance = (dx * dx + dy * dy).sqrt(); // Distance for sorting

        if !asteroid_map.contains_key(&(angle, 0.0)) {
            angles.push(angle);
        }
        asteroid_map.entry((angle, distance)).or_insert(asteroid);
    }

    angles.sort_by(|a, b| {
        if (a - b).abs() > PI {
            b.partial_cmp(a).unwrap_or(Ordering::Equal)
        } else {
            a.partial_cmp(b).unwrap_or(Ordering::Equal)
        }
    });

    let mut vaporization_order = Vec::new();
    let mut rotation_count = 0;
    let mut distance_map: HashMap<f64, Vec<((f64, f64), (usize, usize))>> = HashMap::new();

    for (&angle, &asteroid) in &asteroid_map {
        distance_map.entry(angle).or_default().push(((angle, (asteroid.0 as f64 - station.0 as f64).hypot(asteroid.1 as f64 - station.1 as f64)), asteroid));
    }

    while !asteroid_map.is_empty() {
        for &angle in &angles {
            if let Some(asteroid) = distance_map.get_mut(&angle).and_then(|v| v.pop().map(|(_, a)| a)) {
                vaporization_order.push(asteroid);
                rotation_count += 1;
                if rotation_count == 200 {
                    return vaporization_order;
                }
                asteroid_map.remove(&(angle, 0.0)); // Remove this asteroid from the map after vaporization
            }
        }
    }

    vaporization_order
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(340, p1(IN));
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
