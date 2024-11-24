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
    let vaporization_order = vaporize_asteroids(center, &asteroids);
    
    // For part two, find the 200th asteroid vaporized
    let two_hundredth = vaporization_order.get(199).unwrap_or(&(0, 0));
    let result = two_hundredth.0 * 100 + two_hundredth.1;
    println!("The 200th asteroid vaporized is at {:?}, result: {}", two_hundredth, result);

    result
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
    let mut asteroid_angles: Vec<((f64, f64), (usize, usize))> = asteroids
        .iter()
        .filter(|&&asteroid| asteroid != station)
        .map(|&asteroid| {
            let (dx, dy) = (asteroid.0 as f64 - station.0 as f64, asteroid.1 as f64 - station.1 as f64);
            let angle = (dx.atan2(dy) + 2.0 * PI) % (2.0 * PI); // Angle in radians, adjusted for clockwise rotation
            let distance = dx.hypot(dy);
            ((angle, distance), asteroid)
        })
        .collect();

    // Sort by angle first, then by distance
    asteroid_angles.sort_by(|a, b| {
        let angle_cmp = if (a.0.0 - b.0.0).abs() > PI {
            b.0.0.partial_cmp(&a.0.0).unwrap_or(Ordering::Equal)
        } else {
            a.0.0.partial_cmp(&b.0.0).unwrap_or(Ordering::Equal)
        };
        if angle_cmp == Ordering::Equal {
            a.0.1.partial_cmp(&b.0.1).unwrap_or(Ordering::Equal) // If angles are close enough, sort by distance
        } else {
            angle_cmp
        }
    });

    let mut vaporization_order = Vec::new();
    let mut rotation_count = 0;
    let mut current_index = 0;

    while !asteroid_angles.is_empty() {
        if let Some((_, asteroid)) = asteroid_angles.get(current_index) {
            vaporization_order.push(*asteroid);
            rotation_count += 1;
            if rotation_count == 200 {
                return vaporization_order;
            }
            asteroid_angles.remove(current_index);
            current_index %= asteroid_angles.len(); // Cycle back to the start if we've gone through all angles
        } else {
            current_index += 1;
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
