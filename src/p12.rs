use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    vx: i32,
    vy: i32,
    vz: i32,
}

impl Moon {
    fn new(t: (i32, i32, i32)) -> Self {
        Self {
            x: t.0,
            y: t.1,
            z: t.2,
            vx: 0,
            vy: 0,
            vz: 0,
        }
    }
    
    fn potential_energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
    
    fn kinetic_energy(&self) -> i32 {
        self.vx.abs() + self.vy.abs() + self.vz.abs()
    }

    fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }
}

fn get_moons(input: &str) -> Vec<Moon> {
    input
        .lines()
        .map(|line| {
            let t1: Vec<&str> = line[1..line.len() - 1].split(", ").collect();
            let a: i32 = t1[0].split_once('=').unwrap().1.parse().unwrap();
            let b: i32 = t1[1].split_once('=').unwrap().1.parse().unwrap();
            let c: i32 = t1[2].split_once('=').unwrap().1.parse().unwrap();
            Moon::new((a, b, c))
        })
    .collect()
}

pub fn p1(input: &str) -> i32 {
    let mut total = 0;
    let mut moons = get_moons(input);
    let len = moons.len();
    // dbg!(moons);

    for step in 0..1000 {

        // calc gravity
        for i in 0..len {
            for j in 0..len {
                if i == j { continue; }

                moons[i].vx += apply_gravity(moons[i].x, moons[j].x);
                moons[i].vy += apply_gravity(moons[i].y, moons[j].y);
                moons[i].vz += apply_gravity(moons[i].z, moons[j].z);
            }
        }

        // apply velocity
        for i in 0..len {
            moons[i].x += moons[i].vx;
            moons[i].y += moons[i].vy;
            moons[i].z += moons[i].vz;
        }
    }

    moons.into_iter().map(|m| m.total_energy()).sum()
}

fn apply_gravity(a: i32, b: i32) -> i32 {
    if a < b {
        1
    } else {
        -1
    }
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
