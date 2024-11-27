use std::collections::{HashMap, HashSet};

fn get_moons(input: &str) -> [(i32, i32, i32); 4] {
    let mut moons: [(i32, i32, i32); 4] = [(0, 0, 0); 4];

    for (i, line) in input.lines().enumerate() {
        let t1: Vec<&str> = line[1..line.len() - 1].split(", ").collect();
        let a: i32 = t1[0].split_once('=').unwrap().1.parse().unwrap();
        let b: i32 = t1[1].split_once('=').unwrap().1.parse().unwrap();
        let c: i32 = t1[2].split_once('=').unwrap().1.parse().unwrap();
        moons[i] = (a, b, c);
    }

    moons
}

pub fn p1(input: &str) -> i32 {
    let mut total = 0;
    let mut moons = get_moons(input);
    dbg!(moons);

    for i in 0..1000 {
    }

    total
}

fn apply_gravity(a: i32, b: i32) -> i32 {
    if a < b {
        b - a
    } else {
        a - b
    }
}

fn process_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
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



pub static IN: &str = "<x=1, y=3, z=-11>
<x=17, y=-10, z=-8>
<x=-1, y=-15, z=2>
<x=12, y=-4, z=-4>";
