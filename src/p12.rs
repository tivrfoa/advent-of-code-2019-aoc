use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Moon {
    pos: [i64; 3],
    vel: [i64; 3],
}

impl Moon {
    fn new(t: (i64, i64, i64)) -> Self {
        Self {
            pos: [t.0, t.1, t.2],
            vel: [0, 0, 0],
        }
    }
    
    fn potential_energy(&self) -> i64 {
        self.pos.iter().map(|p| p.abs()).sum()
    }
    
    fn kinetic_energy(&self) -> i64 {
        self.vel.iter().map(|p| p.abs()).sum()
    }

    fn total_energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }
}

fn get_moons(input: &str) -> Vec<Moon> {
    input
        .lines()
        .map(|line| {
            let t1: Vec<&str> = line[1..line.len() - 1].split(", ").collect();
            let a: i64 = t1[0].split_once('=').unwrap().1.parse().unwrap();
            let b: i64 = t1[1].split_once('=').unwrap().1.parse().unwrap();
            let c: i64 = t1[2].split_once('=').unwrap().1.parse().unwrap();
            Moon::new((a, b, c))
        })
    .collect()
}

pub fn p1(input: &str, num_steps: usize) -> i64 {
    let mut moons = get_moons(input);
    let len = moons.len();
    dbg!(&moons);

    for step in 0..num_steps {
        // println!("{step}");

        // calc gravity
        for i in 0..len {
            for j in 0..len {
                if i == j { continue; }
                for d in 0..3 {
                    moons[i].vel[d] += apply_gravity(moons[i].pos[d], moons[j].pos[d]);
                }
            }
        }

        // apply velocity
        for i in 0..len {
            for d in 0..3 {
                moons[i].pos[d] += moons[i].vel[d];
            }
        }
    }

    moons.into_iter().map(|m| m.total_energy()).sum()
}

// x, y and z are independent
// https://www.reddit.com/r/adventofcode/comments/e9j0ve/comment/faja0lj/
pub fn p2(input: &str) -> i64 {
    let mut moons = get_moons(input);
    let len = moons.len();
    let mut rep: [i64; 3] = [0; 3];

    for r in 0..3 {
        let mut set: HashSet<Vec<(i64, i64)>> = HashSet::new();
        set.insert(moons.iter().map(|m| (m.pos[r], m.vel[r])).collect());
        for steps in 1.. {
            // calc gravity
            for i in 0..len {
                for j in 0..len {
                    if i == j { continue; }
                    moons[i].vel[r] += apply_gravity(moons[i].pos[r], moons[j].pos[r]);
                }
            }

            // apply velocity
            for i in 0..len {
                moons[i].pos[r] += moons[i].vel[r];
            }

            if !set.insert(moons.iter().map(|m| (m.pos[r], m.vel[r])).collect()) {
                rep[r] = steps;
                break;
            }
        }
    }

    crate::util::lcm_of_array(&rep)

}

fn apply_gravity(a: i64, b: i64) -> i64 {
    if a < b {
        1
    } else if a > b {
        -1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_sample10() {
        assert_eq!(179, p1(SAMPLE1, 10));
    }

    #[test]
    fn test_p1() {
        assert_eq!(8310, p1(IN, 1000));
    }

    #[test]
    fn p2_sample10() {
        assert_eq!(2772, p2(SAMPLE1));
    }

    #[test]
    fn test_p2() {
        assert_eq!(319290382980408, p2(IN));
    }
}





// -------------------------- INPUT



pub static SAMPLE1: &str = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";




pub static IN: &str = "<x=1, y=3, z=-11>
<x=17, y=-10, z=-8>
<x=-1, y=-15, z=2>
<x=12, y=-4, z=-4>";
