use std::collections::*;
use crate::util::*;

#[allow(dead_code)]
fn parse(input: &str) -> usize {


    0
}

pub fn p1(input: &str) -> usize {
    let mut uniq = HashSet::new();
    let mut qt = 0;
    let grid = input_to_char_grid(input);
    let rows = grid.len();
    let cols = grid[0].len();
    
    for (r, row) in grid.it() {
        for (c, l) in row.it() {
            let l = *l;
            if l == '.' { continue; }

            // check right
            for j in c + 2..cols {
                if l != grid[r][j] { continue; }

                let distance = j - c;

                // left antinode
                if c >= distance {
                    uniq.insert((r, c - distance));
                    qt += 1;
                }

                // right antinode
                if j + distance < cols {
                    uniq.insert((r, j + distance));
                    qt += 1;
                }
            }

            // check remaining cells down
            for i in r + 1..rows {
                for j in 0..cols {
                    if l != grid[i][j] { continue; }
                    let r_dist = i - r; // always positive
                    let c_dist = if c >= j { c - j } else { j - c };
                    if r_dist == 1 && c_dist == 0 { continue; }

                    if c == j {
                        // up antinode
                        if r >= r_dist {
                            uniq.insert((r - r_dist, c));
                            qt += 1;
                        }

                        // down antinode
                        if i + r_dist < rows {
                            uniq.insert((i + r_dist, c));
                            qt += 1;
                        }
                    } else if j < c {
                        // other letter is on bottom left

                        // up right antinode
                        if r >= r_dist && c + c_dist < cols {
                            uniq.insert((r - r_dist, c + c_dist));
                            qt += 1;
                        }

                        // down left antinode
                        if i + r_dist < rows && j >= c_dist {
                            uniq.insert((i + r_dist, j - c_dist));
                            qt += 1;
                        }
                    } else {
                        // other letter is on bottom right

                        // up left antinode
                        if r >= r_dist && c >= c_dist {
                            uniq.insert((r - r_dist, c - c_dist));
                            qt += 1;
                        }

                        // down right antinode
                        if i + r_dist < rows && j + c_dist < cols {
                            uniq.insert((i + r_dist, j + c_dist));
                            qt += 1;
                        }
                    }
                }
            }
        }
    }

    dbg!(qt, uniq.len());
    uniq.len()
}

struct Antenna {
    positions: HashSet<(usize, usize)>,
    antinodes: HashSet<(usize, usize)>,
}

impl Antenna {
    fn new(positions: HashSet<(usize, usize)>) -> Self {
        Self { positions, antinodes: HashSet::new() }
    }
}

pub fn p2(input: &str) -> usize {
    let grid = input_to_char_grid(input);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut map: HashMap<char, Antenna> = HashMap::new();
    
    for (r, row) in grid.it() {
        for (c, l) in row.it() {
            let l = *l;
            if l == '.' { continue; }

            let antenna = map.entry(l).or_insert(Antenna::new(HashSet::from([(r, c)])));

            // check right
            for j in c + 1..cols {
                if l != grid[r][j] { continue; }
                antenna.positions.insert((r, j));

                let distance = j - c;

                // left antinode
                let mut cd = distance;
                while c >= cd {
                    antenna.antinodes.insert((r, c - cd));
                    cd += distance;
                }

                // right antinode
                let mut cd = distance;
                while j + cd < cols {
                    antenna.antinodes.insert((r, j + cd));
                    cd += distance;
                }
            }

            // check remaining cells down
            for i in r + 1..rows {
                for j in 0..cols {
                    if l != grid[i][j] { continue; }

                    antenna.positions.insert((i, j));

                    let r_dist = i - r; // always positive
                    let c_dist = if c >= j { c - j } else { j - c };

                    if c == j {
                        // up antinode
                        let (mut rd, mut cd) = (r_dist, c_dist);
                        while r >= rd {
                            antenna.antinodes.insert((r - rd, c));
                            rd += r_dist;
                            cd += c_dist;
                        }

                        // down antinode
                        let (mut rd, mut cd) = (r_dist, c_dist);
                        while i + rd < rows {
                            antenna.antinodes.insert((i + rd, c));
                            rd += r_dist;
                            cd += c_dist;
                        }
                    } else if j < c {
                        // other letter is on bottom left

                        // up right antinode
                        let (mut rd, mut cd) = (r_dist, c_dist);
                        while r >= rd && c + cd < cols {
                            antenna.antinodes.insert((r - rd, c + cd));
                            rd += r_dist;
                            cd += c_dist;
                        }

                        // down left antinode
                        let (mut rd, mut cd) = (r_dist, c_dist);
                        while i + rd < rows && j >= cd {
                            antenna.antinodes.insert((i + rd, j - cd));
                            rd += r_dist;
                            cd += c_dist;
                        }
                    } else {
                        // other letter is on bottom right

                        // up left antinode
                        let (mut rd, mut cd) = (r_dist, c_dist);
                        while r >= rd && c >= cd {
                            antenna.antinodes.insert((r - rd, c - cd));
                            rd += r_dist;
                            cd += c_dist;
                        }

                        // down right antinode
                        let (mut rd, mut cd) = (r_dist, c_dist);
                        while i + rd < rows && j + cd < cols {
                            antenna.antinodes.insert((i + rd, j + cd));
                            rd += r_dist;
                            cd += c_dist;
                        }
                    }
                }
            }
        }
    }

    let mut set = HashSet::new();
    map.into_iter()
        .for_each(|(_, v)| {
            set.extend(v.antinodes.into_iter());
            set.extend(v.positions.into_iter());
        });

    for (i, row) in grid.it() {
        for (j, l) in row.it() {
            if set.contains(&(i, j)) {
                print!("#");
            } else {
                print!("{l}");
            }
        }
        println!();
    }

    set.len()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sample() {
        assert_eq!(14, p1(SAMPLE));
    }

    #[test]
    fn test_p1_in() {
        assert_eq!(354, p1(IN));
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(34, p2(SAMPLE));
    }

    #[test]
    fn test_p2_in() {
        assert_eq!(1263, p2(IN));
    }
}


// -------------------------- INPUT



pub static SAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

pub static IN: &str = "..................................w...............
..................................................
.....................................C............
......................................3...........
.............b.......u.........w...........3......
...........................u......................
...8........K...........u.......y.........I.......
..................................P5........B.....
...U................2.P...............B...C.......
............i......x.P........w......s.C......IB5.
..............t....................y....x.........
.......8............K....1.......w............u...
.................i.............r.........s........
..2.............t..T.K.......r......8..........I..
8..............t..T.....r...................5.....
...2........................1....M.t...N....x.....
...V....U.......................N.S..........I....
.........W...i............O....v............S.....
................L...1...s.fT.....x............3...
....6.......C...N.........Tf.3....................
...6.......F..........V..........k...N......H.....
...................U..f........0......H.y.........
.......................O...P......0...............
.......L..U....m.......R............s.............
6...i.................O....0.2.........H.....B....
........................R......H.........S........
......F.....c..........m..............d...........
................F.L.....m..................7......
.............J.........................S..........
.b....j.k..............V0.........................
.................L.....K..........................
.......F.......J..............r.....M.............
......................m1....a.R...7...............
.......4......Y..6.D..............................
k............9.......D................M...........
.....Xb.................V...h.....................
A.........9.Xl..........D......R..................
4.............c..d........D.............7.........
.A.4.............c.............M.7.v..............
..........n.9........................h............
...j....bd.........f.....p..W.....................
.............k.........p..........................
.......W4.......p......X.....5..J.....v...........
........W.d..c......A........n..v.....o...........
..........l.....n..........o......Y...h...........
.....A............................................
..j..........n....................................
a9.lX..................Y...........o..............
.......a.................Y..........o.............
...a....l.......................p.................";
