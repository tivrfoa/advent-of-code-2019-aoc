use std::collections::HashSet;

pub fn p1(input: &str) -> usize {
    let map = process_input(input);
    let asteroids = find_asteroids(&map);
    let center = find_best_monitoring_station(&asteroids);
    println!("Best station: {:?}", center.1);

    center.0
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
fn find_best_monitoring_station(asteroids: &HashSet<(usize, usize)>) -> (usize, (usize, usize)) {
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
