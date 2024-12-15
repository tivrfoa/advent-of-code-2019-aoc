use std::collections::*;
use crate::util::*;

const EMPTY: char = '.';
const BOX: char = 'O';
const BL: char = '[';
const BR: char = ']';
const WALL: char = '#';
const ROBOT: char = '@';

fn get_robot_pos(g: &Vec<Vec<char>>) -> (usize, usize) {
    for (r, row) in g.it() {
        for (c, v) in row.it() {
            if *v == ROBOT {
                return (r, c);
            }
        }
    }
    panic!("Failed to find robot.");
}

pub fn p1(input: &str) -> usize {
    let mut sum_box_positions = 0; // answer
    let mut g = vec![];
    let mut lines = input.lines();

    while let Some(l) = lines.next() {
        if l.is_empty() { break; }

        g.push(l.chars().collect::<Vec<char>>());
    }
    let (mut r, mut c) = get_robot_pos(&g);
    // dbg!(g);

    let mut mvs: Vec<char> = vec![];
    while let Some(l) = lines.next() {
        mvs.append(&mut l.chars().collect::<Vec<char>>());
    }

    for m in mvs {
        match m {
            '^' => {
                let (mut cr, mut cc) = (r, c);
                while g[cr - 1][cc] == BOX {
                    cr -= 1;
                }
                if g[cr - 1][c] == EMPTY {
                    for i in cr - 1..r {
                        g[i][c] = g[i + 1][c];
                    }
                    g[r][c] = EMPTY;
                    r -= 1;
                } else if g[cr - 1][cc] == WALL {
                    // don't move
                } else {
                    panic!("{}", g[cr - 1][cc]);
                }
            }
            'v' => {
                let (mut cr, mut cc) = (r, c);
                while g[cr + 1][cc] == BOX {
                    cr += 1;
                }
                if g[cr + 1][c] == EMPTY {
                    for i in (r + 1..=cr + 1).rev() {
                        g[i][c] = g[i - 1][c];
                    }
                    g[r][c] = EMPTY;
                    r += 1;
                } else if g[cr + 1][cc] == WALL {
                    // don't move
                } else {
                    panic!("{}", g[cr - 1][cc]);
                }
            }
            '>' => {
                let (mut cr, mut cc) = (r, c);
                while g[cr][cc + 1] == BOX {
                    cc += 1;
                }
                if g[cr][cc + 1] == EMPTY {
                    for i in (c + 1..=cc + 1).rev() {
                        g[r][i] = g[r][i - 1];
                    }
                    g[r][c] = EMPTY;
                    c += 1;
                } else if g[cr][cc + 1] == WALL {
                    // don't move
                } else {
                    panic!("{}", g[cr][cc + 1]);
                }
            }
            '<' => {
                let (mut cr, mut cc) = (r, c);
                while g[cr][cc - 1] == BOX {
                    cc -= 1;
                }
                if g[cr][cc - 1] == EMPTY {
                    for i in cc - 1..c {
                        g[r][i] = g[r][i + 1];
                    }
                    g[r][c] = EMPTY;
                    c -= 1;
                } else if g[cr][cc - 1] == WALL {
                    // don't move
                } else {
                    panic!("{}", g[cr][cc - 1]);
                }
            }
            _ => panic!("..."),
        }
    }

    for (r, row) in g.it() {
        for (c, v) in row.it() {
            if *v == BOX {
                sum_box_positions += r * 100 + c;
            }
        }
    }

    sum_box_positions
}

pub fn p2(input: &str) -> usize {
    let mut sum_box_positions = 0;
    let mut g = vec![];
    let mut lines = input.lines();

    while let Some(l) = lines.next() {
        if l.is_empty() { break; }

        g.push(l.chars().collect::<Vec<char>>());
    }
    // make it wider
    let mut ng = vec![vec!['.'; g[0].len() * 2]; g.len()];
    for (r, row) in g.it() {
        let mut col = 0;
        for (_, v) in row.it() {
            if *v == ROBOT {
                ng[r][col] = *v;
            } else if *v == BOX {
                ng[r][col] = '[';
                ng[r][col + 1] = ']';
            } else { // WALL or EMPTY
                ng[r][col] = *v;
                ng[r][col + 1] = *v;
            }
            col += 2;
        }
    }
    g = ng;
    let (mut r, mut c) = get_robot_pos(&g);
    dbg!(r, c);

    let mut mvs: Vec<char> = vec![];
    while let Some(l) = lines.next() {
        mvs.append(&mut l.chars().collect::<Vec<char>>());
    }

    for (midx, m) in mvs.it() {
        assert_eq!(ROBOT, g[r][c]);
        // println!("{} {} {} {}", midx, m, r, c);
        // draw_grid(&g);
        // wait_input();
        match m {
            '^' | 'v' => {
                let d: i64 = if *m == '^' { -1 } else { 1 };
                let mut to_move = vec![(r, c)];
                let mut move_set = HashSet::from([(r, c)]);
                let mut to_test = HashSet::from([(ad(r, d), c)]);
                let mut stop = false;
                loop {
                    let mut new_test = HashSet::new();
                    for (nr, nc) in to_test {
                        if g[nr][nc] == WALL {
                            stop = true;
                            break;
                        }
                        if g[nr][nc] == BL {
                            if move_set.insert((nr, nc)) { to_move.push((nr, nc)); }
                            if move_set.insert((nr, nc + 1)) { to_move.push((nr, nc + 1)); }
                            new_test.insert((ad(nr, d), nc));
                            new_test.insert((ad(nr, d), nc + 1));
                        } else if g[nr][nc] == BR {
                            if move_set.insert((nr, nc - 1)) { to_move.push((nr, nc - 1)); }
                            if move_set.insert((nr, nc))  { to_move.push((nr, nc)); }
                            new_test.insert((ad(nr, d), nc - 1));
                            new_test.insert((ad(nr, d), nc));
                        }
                    }
                    if stop || new_test.is_empty() { break; }
                    to_test = new_test;
                }

                if !stop {
                    dbg!(&to_move);
                    for (nr, nc) in to_move.into_iter().rev() {
                        g[ad(nr, d)][nc] = g[nr][nc];
                        g[nr][nc] = EMPTY;
                    }
                    r = ad(r, d);
                }
            }
            '>' => {
                let (mut cr, mut cc) = (r, c);
                while g[cr][cc + 1] == BL {
                    cc += 2;
                }
                if g[cr][cc + 1] == EMPTY {
                    for i in (c + 1..=cc + 1).rev() {
                        g[r][i] = g[r][i - 1];
                    }
                    g[r][c] = EMPTY;
                    c += 1;
                } else if g[cr][cc + 1] == WALL {
                    // don't move
                } else {
                    panic!("{}", g[cr][cc + 1]);
                }
            }
            '<' => {
                let (mut cr, mut cc) = (r, c);
                while g[cr][cc - 1] == BR {
                    cc -= 2;
                }
                if g[cr][cc - 1] == EMPTY {
                    for i in cc - 1..c {
                        g[r][i] = g[r][i + 1];
                    }
                    g[r][c] = EMPTY;
                    c -= 1;
                } else if g[cr][cc - 1] == WALL {
                    // don't move
                } else {
                    panic!("{}", g[cr][cc - 1]);
                }
            }
            _ => panic!("..."),
        }
    }

    for (r, row) in g.it() {
        for (c, v) in row.it() {
            if *v == BL {
                sum_box_positions += r * 100 + c;
            }
        }
    }

    sum_box_positions
}

fn wait_input() {
    use std::io::Write;
    let mut input = String::new();
    print!("Please enter some input: ");
    std::io::stdout().flush().unwrap(); // Ensure the prompt is displayed
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sample0() {
        assert_eq!(2028, p1(SAMPLE0));
    }

    #[test]
    fn test_p1_sample1() {
        assert_eq!(10092, p1(SAMPLE1));
    }

    #[test]
    fn test_p1_in() {
        assert_eq!(1577255, p1(IN));
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(9021, p2(SAMPLE1));
    }

    #[test]
    #[ignore]
    fn test_p2_in() {
        assert_eq!(1597035, p2(IN));
    }
}


// -------------------------- INPUT

pub static SAMPLE0: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

pub static SAMPLE1: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

pub static IN: &str = "##################################################
#.O.O.O.#..O.....O..O#O.........OO...OO..O......O#
#..OO#...OO....O#O..O....O...#...O...#...O.O.O##.#
#.....#..O...OO.O.O.O.O#O..O..O.O...O.O.O......O.#
#..............O.#O.....#.#O.#....OO....O...#..OO#
#O.O....#.OO.O.OOO..O..O.OO.#..O....O...#..O..O..#
#.O...O..OO..#O......OO.O.O.....O.O#O.........O..#
#.O#..O.O..#.O.O.O..#.O.O.O#..O..O.O.OO...O.O....#
#......O.....O.O.OO.O..O..#O.........OO.........##
#......O.......#.......O......O..#OO...........#.#
#...O#O..O....OOOOOOOO....O...OO.#..OO...O......O#
#.O.#.O.#O....O.OO#..O..............OO..#....OOO.#
#..OO.##..#..O........O#.O..#.#.............O#...#
#.OOO#.OO.......OO.#O.O.O..O..#.O.O.O....##.OO...#
#..#O.OOOOOOOO..O......O...O...O#.#..OO..OO#....O#
#.O..O....#..O......#.#.O...O.O#.O.O#.O.....O##..#
#......OO.OOOO..#O#.#..O....O...O..O....O..#O..O.#
#..O.O..#......O....OOO.O#.OO.#..O........#....O.#
#...O................#.OO....#.O..O.#.O.#OO..##.O#
#.O.O.O.#.O.O.......O..........O...##OO.O.O......#
#.#.O...OO.O#OO.O...#...OO.OO...#......#.O#O.O...#
#.........O.OOO......O...#.#...OO#.O....OO.O.....#
#.....OOOO.....O.O..O.O.OO......OO.OOO#....O..OOO#
#...O..........O.O.O....O.O.......#...O..OO....O.#
#.O...#......O....O.#...@..OO.#O.O.#.O.O...#O.O###
#.#...OO...O#OOO.....OO..O...#O......O...O.OO..O.#
#O#O.OOOO.O.....O......O.....O...O.O.......OO..O##
#O.OOO..OO.OO..O..O...##......OO.OO..#O....#....O#
#.O...O.OO....O...O.O.OOO.O...O#..O.O.O..O..OOOOO#
#..#.OO.#O#....O.#O.#..#...##.......OO.#.#..O.OO.#
#.OO....O#......O.O..O....O..#O.O..OO..O#.O...OOO#
#...O...#OO.........O#O..#..O..O.#O...O.O#..OO#.O#
#OO....#...O#....O#.O#..O...O.....#.....O#O.OO.#.#
#..OO...#O.O.O.........O...O...O...O#.....OO.OO..#
#O...O..O.#....OOO...O..O...OO....O...O.....OO...#
#......O....OOOO..#........O..#O.OOO#....O.OOO..O#
#O.O...O..#..#..OO#...##O..OO##O.OO.OO.O..O..O..O#
#..O.O.O..O....O#....#..O.O.#O...OOO#OO.O.......O#
#.......OO.#.#.#.......O.##....O...O.OO.O#.O#....#
#.....O...O.#..#..O...#O.#..O............O.O#..#.#
#.#.O........O#.....O.O..O..O........#...O.O.O.OO#
##.O..OO..O..O..O..OO.#..OO...O.#O..#....#..O.#.##
#O..O..#.O..O.OOO..O..O..OO.O......O.......O..O..#
#...O###.OOOO................O...O..OO..#...O.#OO#
#..O...O..O....O.O.O...O.O.OO..OOOO.#...O#.OO...O#
#O.O.O......O.#..O.O#.O.OO..O#......O.#..........#
#O..O......#OO.......OOO.....O.OO....OOO.#.O..O..#
#OO.#..O.....O..........OO.OO..O.O..#.....#OO..O.#
#......O..O..O...OOO..OO....#....O.....O.###O....#
##################################################

^><><>^>vv<>^v^v<^<><<<>v<vv>><^<>>^>>>^><<v<><<>>v>>><v>v^vvvv<^v>><^<<><v>^^^^v^^>^v>vv<<v>v^v<<^<v^<^<>^^<^<<^<<<v<<>v^^v>^>v^v^^^^^<v>><<^vv>vvv><>^<v<v>>v><vv^vv^<v^<^^^<^<^<^v^>^<v<>^v<^>>>v>^^<<v<><v<><^>><v<><^>vv>vv^<<^><<<^><v<<v^<^>v>><v<>v^v><<>>v<>^^<vv^<^><vv><^<<v<v>^>v<^<>^<^<^vv<>^>^^vvv<><vvv>v<>^v<^v>v^>v><<vv<v<<^^^>>v>><v>v>>>v>vv<^<^<^vvv><><v^><v<^vv<<v<^>>>v<>^>vv<<vvv>v<^<v>>^v^v^><v><^^^><^<v>v<^v>^v<>vv>>>v^<v^^<^^v<^><><<<v>>><^<vvv^<<v>>^v^<^<>><^<<><>v><v^>^><><>vv^^><^>vv^vv<<<vv^><><^v^<v><v^^v>>^<<<<<<>^^<vv<>v>v^^<^<v^>^vv^>>^>>><>^>^v<>>>vv>vv^^v><>>^v<>v^>^v>^^^<<<>v<<>>>v><^>v<v<<<^^>><v<>^<<>>vvv>>>>^<<>^^>v<v>>^<><^<vv<<<^^<<v^vv>^^v^^v>^<v<^^^v^<^<^v>>^><><>^^^>v<>^^>v<^v^><<^^^<<>v<^><v^v<<<^v<^<>v^^v><v<v^vv^<v>v<vvvv<>vvv>^^vv>^v^<v^^<^^>^<v><>^<v^vv><<<>vvvvv^^<^>v^^><v><v<^vv<v^^^>>v^<v>>><vv^<v<<>v^v><^^v<^<v<<<>^><v<v<v<<v<>><^>vvv<^>^><v>^<^>^>>>v<<^^v><^><^^^>>^>^<v>v><>>^v^^v>^>>>>^^>><<^<<<^>>^<v^v<v^>^vv<>^^>^vv^<>><^><v><<vv^v<<^<<v<
<<<>><^<<^v<<v>vvv>><v><^v<v^^<vvvv>^^v^v^vv>v^v><>^<vv>^vv<><>>>v>>^^v>v^><<v^<v^^vv<^v>^vv><<v<v^^<>^<v<><^<<^^^<^<>><<<v<vv<>v^^<<^^^^^^v>v<vvv<<>>><^<<^vv>^>vvv<v<>>v>^>^^v^<>v^v^>^>v^><^<^<^>v<<>v<<<v>^<^^v<<^^^>^>^^^<<>>v<^<^v>^<^<<^>^^<vv>^^^>^<^^^v>^<<^v><^>>>>>^^<^v<v>>>><<^<>>>>^<^<v<>^^^v<<<vv>>^<v^<v^<<^v><<<<^<v<>vvv>>^v><>v^^>^><>v<>vv><^v>>v^^^^v>^<><><>>^>v^vv<^v^^<^>v^<>>^>^<v>^^>>^>>>>v>^><<vvv^>^vv^v^v<><<<>><>>^<^^^>^vv<<^^>^>vvv<>>>>v<^^v>vvv>><^<>^^vv<^vvv>>>>v^>v<^>v><<>^^v<^>v^v^<vv^<v^^>v<>^>^v><>>>^>^><v><v<<^>>>^v<>v<>>>><<v><v<<<v<v>^^^v>^<v^v<<<<^<vvv^v^v<v>v>v^^^^^>><<<^v<>v<><v<<v<<v<v^>v>^^v<^v^^<>v^v<v^>>^v^>^v^<<>^>v^^v>v>^vv>v<v><<^v^v>vv<v<<^<v^v<<v>><>^v><^v<<><>><>^^>>>v<<v<<^<><<<^<<<v>>v><><<vv>>^v>><>>vvv>^<^><<vv<<v<vv^v<<^v^v<vv<^><^<v^>vv<>v^^<>v>^<<<^<>v<v<v<v>^^vv<v^^v>^>^><v<>>><^<<>^>v<vv^vv<>^vv><v<>>^^<>vv>v><v^^>v><v^v>^>><^^>>>^>>^>^^^v<^>v^<>^>>>^<^v>^<v><>^<vv>vv<^vvv<>>^^><>^^<>><v^^<^>>v>>^vv<>v<<<<v>^<<<><^^^<<<>^<^^v^^^<>v<^v<^<
vvv<vvvv^>><^<v^^>^>v^v<^<>>v>>v<<^^v<<>>vvv^v^^<^v^<<v>^>^^<v<v^^>>>^v^><^v<>^<<^^>v<vv<v>><^><v<><^v^v<v>vv<<>v>^<<<v^<vv<><<^>><<v<v^><vvv^v^v><>><<><<v>><<vv^<vvv>^v^<>^vv>^<>^<v<<<<>vv<>>^^<<>>>v^^>v^<>^<v^^<^>><<>^vv>^>v>>^><>v^>^<vv<<vv>><v^v<<^^^>^^^>^<<><v>^<<><>^^<^<<^>^^v>^>^<><>><^vv<>vv^^v>^<^>vv>><>^>v<^v^^>^v^v<<<<^>v<>vv^<<vvv>v>>^^<><><<>>vvv<<^v^<^>>v^<>^^><^^^v>^<v<<<vvv><<<><^>><v>^>^<^>^><v^^<>>v^^vvvv^^^>>v^>v>>^<<vv^v^><<<^^v^^<<<^<>v<v^v<<^<><<v<<<v><^vvv^v<^<>^^v^>>>v><<^^vvv<<v<<v<v>^><>^v^^v<v^v<>>>>^vv<v^<^>>^^^^^<>v>^<<v^v<>v<>v>><><v^>v<<v<<>v^>^>>>^vv<>v<^><>vv>vv<><>>>>^<vv<>><>v<<^v^^>>>><>>v^^><<<v^<v<^>vv<>>^^^^<vv<<^<^<v^<<<^vv^^>>><>^v<<>^v<vv<^v<v<v^><>^>><^v>>v^v^>>^>>v>>^<>>>v^vv^<^<<<<^>><vvv^^^><<><v<^<<v>^^<vv<><v^>^v<>>^vv<v>^>^<v^>^v<^^v<v>>v^<<<>^vv^>^<v^^v^<v^>>><v<^<>^^^<^<vvv^<^>>><>>>v<>v>^v<v><>^^v<<>>v^><^<<>^^^^v><^v>^>>^v>>^<>vv^^v^^^>>^>>v>>v<<v^<^><>v^<^v<v<>^>v<>^>v<v^<><<v>><^v<<<^<v^><^><>^v<<<^<^>><^v<vv><v><v>v<<v^><>^^>v>>v<
><>>v^vvv^<^v<>^>>^<^>^v>>>vvv>><^<<^<>^^^<^^^>^^v>v<^<><^^v<>^v>><>v^>v^vv<vv<><^><<>v^>^^><>v<v>^^<>^>^^>vv<>>><vv<<v^^<<<<>^>^<<>v>^>^vvv<^<^^^vvv^>>>><<vv>^^<^>v^<v>><^v^<^v><v^^>><>v<vv^<><>v^^vv^<>>><>vv<^v>vvvv<^<v<^>>v^>>^<<v<v>^<vvv<^v>>vv^v<<v^^><<v<>^<<^>^<^vv<>^^v<<^^^<v>^^>^>^^>><<>^vv>>v^>><^v>vv<^v^>^<>v<v<>^><<<^v^>>v><>v<^^><v>v^><^>>v<v^vvv<<v^>^><<<>vv<><>vv^<^^>>^^<^<><<>>^>v^><>^vv<<v<>v^v^<>v<^v<^^<^<^v><<^v<^^v<v^^>^v>^^v^>v^>v><v><><v^<v>^^^vvv<<<v<vv<<>><^<^v<^vv^^>>^^v^>v^v^<><^<^>><>>^<<vvvv^<^^^<<^vv<^^<>v<v<^<<^><^<>vv>v><v>><><>v^^v<v<<^v<>>v><><vv>^^^<v<^<<v^^^>v<v>^><<v>>v^v>v<^vvv<<><>v>v<v><<^<^v^<<<>v<>v^<v>^^^><^>^<><v^>^^v>>^<v<><>^^^><v>v>^<>><>v><v<<<^vv>>^^v^v>>>v^<^<>>><^>v>vv<v>^vv><v<v<v<v>v^v<><^><<v>><vv<<<^>>>^vv<>><^^vv><v<>v^v<<>>^<^<>v^v^vv^>vv^^^vvvv<>^>>^^>v<^>^<^<^>vvvv^>>^v^^<^<>vv<v^<>^^vvv<vvv^>^v<>^>>v><><<>>v^^v<>^<<<>>v<<<><v^>v>v^<>>><v^>><^<v^^v<>v><<><>><<>vv<>>^v<<>>><>>v^^v^v^<>>>^vvv^<^^v>>^v^<^><<^>^v^><vv<<>>v<^^>^^>><<^
<v<<><<v^^^v^^vv<>>vvv<>^v>v<><><<<v^^>^^v^^^^<<<>><>^<^><v>>v>><<<^>^<<>^^><v^>>^><v>v>v^<<v^>v>>^<>>v><v><<^^v<v^>^<v<>>>^>>v><>>v><>v>^vv^^^<v<v<v<>v^^^^>^>vv^><v^^>^<>><<^<<v^^v<^v<^v^<>^>><v^<v^<>^^^<>vv>v<<v<v>^>^>^^>^^^><<>v>>^>v>v>^^<<^<><<>^>^^v^^^^v>vvvvvv<^<<>>v<<<v>^<<><vv<<^><vv<v^v<><^<^><v<^<^<v^<^^<^^>>^><v^><v^<^>v^v<>vvvvv<>v>vv<v>^^>><>>^v^v>^vv><v>v><<><^<>vv>v^<v>^v>vv>^^vv<<^<<^^>v<^vv><<<>v>^^v<><^<^^^^^<<<>^^<^v^^v>>>vvvvv<<<>vv<<^v<^vv<vv>v>^^v<^^>>^vv^v^v^>>v><^vv<>>^v^v<v>^>^v><>^>v<<v<>^v<vv<<<v<<<^>v>v<>v^>^v<>^<>><vv<>^<v<v^^v^^>v><<>^^v<^^<>vvv^v<<^>vv<<<^>^>>vv^><<>>v><v^<vv><<<><><^>><><><v<^v^v^^>>v^v<vv>v^vv><>v^^^<>>^<>^^^<<v^vv^>vv<^v<<v^v<v<<>^v<<><<^^v^^>v^vv<^>vv><v>>>vv^v^^<<^><>v<<<^v^^<><>v><^v<v>>v>^v<>>v>>v>^<^^v<<v><>>v<^v<^>v<^v><v<<^vv><^^<^^<<vv^>^<v<<<^>^>v>><>><v<v<>>vvv<<<vvv^v^>^v>><>><>vvvv<<v^v>v>vvv>^^^><^>>>v<vv^^v^v^<^>^^^<v<v^v><vv<^<vv>>^^v>>v><^v><vvvv>^>>><><<>>><<><<v>>^vv<v>^<v><v<vvv^v>>v><v<<^v^<>v>^v^>vv^^v<v^^^<^^<<vvv
>^<<^v<<><<^>^<<^^^<>vv>vv>vvvv><<<<><vvv<^vvv^<<<^><<<^>v^<^v>^><^v^><>^v<>^v>^v^v^^>>^<<v<>><<v<^><v>^<<>v>><vv>^>v^<v<<^>^<>v>><^v<v^<v^>^>><v^v^>vvv<>>^^>v^<<<v>^v<^vv^v<<<^vv<<^^vv<^<<<vv>>>v^><v^>v><>><><^^^v<^<v^vv><^v^v<v<<^v<>v<>^v^<<<^<<><<>^<^<>vv>v<<^<^v>vv^v<^<>>vv>><>vv><^^>v>>>^^<^vv<^><^^<^>>v<<>vv<v^v<>vvv><><v>^^<>^><^>^>><^<^v>>v<<v<<>^v<<<>v<>^^^<vv<<<^v<>>v^v>^<<>vv<<v<>>^v>^><^>^v^>>^><^>><^^^^vv^<^^><<^^<<<vv>v^^<v^^^<<^<v><^^vv^^<>>v<><<>v^v>v^vv<vv^v^^><>v^>^<>^<v^^><^<<<v>^^^<vv<^>^><<<^<>>^vv^<^<^^>^vv^^v<v^<^<<^>>>v><^v^<<v>^v<>v<vvv<v<v<<><v^^<>^^>v>^>^<><v^v^^v^v>vv>^^v><^<>>^^<^>^><^v<v<vvv^<v<^^<<>^^>v><<v^<<vv<v<^^<>v^<>>><^>vv><^<><^^>vv^>^>>>^v>>v<>><><>>^>>v^^v^^^<<>>v<>^<><v^^^v<^<<v^^^^<^^<^<>>^>><vv>^^v^^>><>>v^>v^>>vv^<>>^><<^>^^v^>>v<><v>v<v^<^>><^^v<^<v^>>>v><<vv^<><^<>^^vv^^^<<^vv^<^vvv<>^<>>v>^^v^>>v^^v<>>^v<<v^>><^^^vvvv^>^<<^<v>^v<<>><<^v<<<^><v<><>>^<v^^>v><^^v>v><>^^<v>v^v<v>>^>^^<>v>><<<v<<>>><>^>^<^v>>vv<<><^^^^>v^><<^^>^<>>>>><<<><vvvv
^v<<^>v>>^vvv<><^>v^>v^<v<v>^<vv<^v<^v^^v><^<^>>^><v>>><><vv^v<vv><<v<v^<<<v^v>^>^v><^>><^<>v>v<>^><<^<^<^v^v>^v>vv<^<<vv^<<><<>><>>><^<<^>^<v><v^>^>^<v>vv^>>^<v<<<<^<^v^^^><<v>v<v><^^><<vv<^<v>^^<>v^>v>vv^><^^vv^><vv><^>^><<^^^<^>^v>^vv>><>^v<^<>v<^^<^>>vv^^^<<>^^v^><>>vvvv>^vv>v<>^<^<v>^>^vvvv^<<<>v>><^><^v^>^vvv<<v<v>^^>>vv<^<^><<^<^>vv>^<^>>^v^>^<><>v<^<>^v>>^<<vv>^>v>v^vvvvv<<>v<v>^^>v>v^vv<><^<>v<^vvv<>>>^^^vvv>>v<v>>>>^v^v<^^^>v^^>^<>>^v<^<<>v>^<>v<vvvvv>^v^v^<><>v^>v<vv^><vv<>v><><<<><v^<<^^v>v<<^^v<<>>v^^^v<>^<^>v<^v<v^^v<^v^<<v<vv<>^>>^>><>^^^^>^>><<><>^^<>vvv>v<>^<vvv>^<^^<^>vv^<^<>^^vv>^>v<<v^vv>^>>>v^v<>>>>vvv^^<^^>><^v<<<^<>>>^^<>>v<^<v^^<<vv<<v^><<v<v>v^^>vvv<<>>^v^^^<v^v^^v>^<<^vv^^v<>v<^^<>v^>^<^^<v>v>^>^<v>>vvv<>v<^v^>>><<^<<^^<^>>>v<^^v^>><>><>v<><^>^><<^>v<<^^<v^>vv<^vvvv>^<vv^>^>^>><^<<>^<v<<>v<vv^<<>^v^<<^^<v<^v><>vvv^^^vvv^<v<<^v^^^^>>vv>v<v<<^^<><^^vvv^<^<v<vv><v>>^v^v<^>^<^^^v>^vv<^<>v>v^<v>>^<>><>><<>>v^>v>v><<<<v>vvvv^>^^vv<<vvv>>>v<<v><<v><^v^v^^>^<<^^^<<>v<
<vv<v^>><>^v<^<<v<>^^v>v><<v^^^^<<<<^<^>><v^^^vvv^v<><v<>^^v^><>v>^^>><v<vv<vv^v>>v^><>^>^<<^v^v>v<<^>^v<<^v>vv<^>>><v>^<>v^>>><^v<v<vvv>v<vv^<v^>^<<vv>><<^^v<<v<^^><<v><>vv>>^<^<>v>v>>>v^^<^<^v>^>vvv^^>^>^><^><v^>v^>^v^^<>vvvv^>>vvvv<^<>><^v<<>^<v^vv^><<><><^><>>^vv^v<^^>v<^>vv<>>v^^^^v<v^>v<<<<^^<>>>v>^v<v^v^v<^v^v<v^><<>vvvvv><<<v^<v^v^v>>v>v>v>v^v><^v<^>^^<>v<<<>^>><<^v<>^<>v<>>>^^v<><<^<>>>v<^v>>>>v^v^><^v>>^^vvv<^v<v>v>^^<>v^v>^<<>^>v<v>^<^>^v>v^v<v^^>>vv<^>><v>vvv^^^<>>^^>^<>><><^v>^<v<v^v<vvv<><<v<><vvvvv>>^^<><^v<<v><^^><vv>>>^<^^<v^<>>v>v^>^v>v>vv^^>v<v^v^^^<^v<^>v>vv<^<>v^^^^>>vv<<<>v^>^vv<v<^><<^v^^<v>v>^<>v<<>v^<v^^>>><<^^^^>^><<<<<v^v^v^vvv<<^><<v<v^><^<vv><^>^^v>>><v<<<<^^<>^vv<>^<>^^>vv<v^^^>vvv>^<>>><vv>>^^>v><>v>^>>^>^<<^^v<^<v<^<^^^v<^<v<^v^<>><^<><v^>^>>v<>>^^vv>>v>><<v^<vv>^^>v^<><<v<v<vv^>>><v<>^v>><<v><>v>^>v<^^<v^<v<^^<>^vv^v>v<>><^^v^^<>^^v^<v<<^^^>>>vv<<v^><v>v^<>>vvv<><>^^><<<v<^vv^>v<>^v<vv^><<<^^>>vv^>^v^>^<v^<^v<>>>^<>v><v>v^<^v<>^v<><^v^><v><v<v<<v^v^^>v<
>>^v><<^<<vv^<>v^v^<>>vv<>^><<<v>>>v^>^^<<v>v<v^v<^^>vv^<v^vvv<v^<^>>^<vv^<>><^vv<<vv^^><v<<^^>v>v>v>^<^v>^>^<^^^^>vv<<v<v^v<v<>^><>^<>^<v^v<v<v<<v>><><>vv<^v<>^^>^>v^<^v^^<^^^<<^<>^<><^vvvv>^<>^v^><<><><^^<>v<>^v<>>>v<v^^>^vvv^^^<><^^^<^^>vvvvv^^vv<>><<>v^^><^<<>>^><><^^<>^>v<<<>^<v><>^><v<>^>^v^^<<^<v^<><v<^^v<<>><v<^><vv^>^<>^<v>><>v^>v^^v<vv>v<<<>^<vv<^v^^v><^^<^<vvv^v^^^>>^<^>><<<>>v^>^<<<>vv><v^^v^<>vv<>^v^v><>><^^v>>>v^><^^^><>v<<v>><v<>>v<^<>>>vv><v><v<<<v<<^<^<^v>v^v>><v<^^^<>>^vv<^>^<>>v^><^<>>>v<^^v>^v^>><^><^<<><^^^>^>vv^>>^>>v<^^^<vv><<<>>^<>>>^^^^><v^vvv>v<^v<^>^vv>>>v^>vv><>v><><>^><v>>>vv^^>^<^^^v<^v>>vvv<v>^^^^><>vv<^>v>v<><>^^vv^v^^<>^^<^<>vv<>v>^<v<^><<>><<vv^>>vv^<v<^^>^<^^<>^^>>^^<><v><vvvv>>><^>v^><>^>v<<v^<^<<^^<^^^>^v<^><^>vv^>^><<^^><v^>>^<^v<<^^v>^>v^>vv^^<v>vv>>><><<><^v^<<<v^^^>^>>v^<vv^^>>>^>^v>>><>^v^^>>v<>vvvv^<^v^<^^^vv<<vvvvv>>^^^v<<^v^><<>^>>>^<v^>v>>vv><v^^v<vv>vv>^^<v^v<^v>^<>>vv^^^<^<<vv<vv>>>^v<>>vvv^^>^v^vv<<^<>^><^^^>><<v<><>^vv<<<<^<>v<^>>^v<^^v
<<>v<vv>^^>>v^^^^v^^>^v<v><^vv^vvv<v^^>^v>^^vv^^^^<^<<>vv<<^>>^^>>^>v>v^><^<<<vv^v^<<>><^>>vv<v>^^<^^v>v<v<v^<vvvv><^>>^^>>^v<^^vv>>>v^^<^<^^<vv^^>v^v><^v^vv^<<v^<^>>v<^>>v^^>>>>vv<^v^v<>v>v^^^^^^^>^^<^<>>^<v^>^^>v>>^^^<<^^^<>^<v^v^<<^v^vvv^>^vvv^^^v^<^>v^<<<^<^v<<<v^^<>^<<vv^<^v><v^<vv^vv<vv^><v^>^<<<^v>v^>v^<vv<<^^<<>>^^>>^<v<^v><^v>>^v^v^>>vv<<vv>v><<<^>><v><^>>v>v<^>vv^^>>vv^v^<vvv^<<v<vv^^v>v<>v><v^^^^v^v^v>v<><<>^vv^^^vvv>^v^v>v<>>^<>^vv>v^>>v^v>v>vv^>^>>>^v<<v><v><>^^<v>^^<v<<><^>^v<^>v>><<>^<^vvvv>v^<>^^v^vv^v^v>>vv^v<v^>>vvvvv^><v^>><v<v^^v>>>^<>>>vv<<<v>^><<<vv^^<<<>vv>vv<v<<>vvv<v^<>v><^<<<><^<^><v<^><^<^<>^><><>^v^<v^>>>v<vv^vv<<^^<>>vv>><<^<<<^v^<<>^<^<>>v^>v<>^^<>>v>>^<^^vv><v^<<<>v<<^v^<>v<>>^>>><vv<>vv^v<<v^><<^>>^>^><v<^>^vv<v<v>^<^><^v^>>^v>vv^v^<v^v^>^^<v^>>><^^^^v^>vvv^vv>v^v^^<^>>>^>v^<<<<^><^v^>v^v>^^<vv<v^^^^>vv>^^v<^<<>^<>v><><<>^v><vvvv<^v>^<^^>v<>^><v<>v<^>><<v^>>v^vv^v>vv^^<<>>v><>^>vv<>>>><>vv^v<>><vvvv<^^v<<v>v<><v^<v>^>^>vv^v>^vvvv>^<^v<<<<^v<^v^v<<^<<v>v>
^>v<^vv^^^>^>>><^>>>v>^>v>>>v<>>v<>^^<>^^^<^>^^<^v<^>>^><^>^^<^>v>>^^>v<<^^<<vv<^vvv<^v^>v<vv^>>><<>><<v^>>^v^^^^>>><>><<>>><^v>^^^^<>^<^<>^><<vv^v><^<^<<>>v<^^<<<<^<^<^<^^<v^<>v<>>>^v<^>><>^vv>^vv^^^<v>^<^vv>>vv<v<^>^<^^^^>^^<<^>v^^>^vv^>^>^v^>>v>vvvv>vvvv>>^><v<^<v>^^><^v^<^^<<^>^<<<<<^<>v^><<>v^<<v<<v>>>v>^<<^<v<<v<vvv^>><vvvvvv^v^vvv<v^><^^vv><>^<>^><>>v^<<^>v>^<vv>>>>v>vv^v<<>^v>^<>^>v^v>^>>>v<<vv<<<^^><>>><^v>^<v^v>v^vvv<v<^v^v<><>v>^v>vv^>><^>v>>>v<^>^<v<vv<<><v>vvvv<^>^<^^<v<><<v>vv<<v>v^^^<<^v<v>^>><^>^<<<<v>v>>><>>^>><<>^><^<<><v^^v^vv^v>v<v<>v>vv^><<>>v>><^<^^v^><<v^>^<><<><>v<<<v^<>>^v>><><<>^>><>v^>v^>vvv<>>^<>vv<^>vv>><>v><<><<<v^^>>v^v^^v<<<<v^vv>^^<>v>>^^^<><<^vv<>vv><^vv<^<v>^v<^>v^^v<vv<^^<^vvv^<^^v<<<><>v<<<^<<>vv><><^<>>^<>>vv<^^<vv>>><>vvv<>>^v><^^v>><<<^><>>>v^vv^^v^^<<<<^>^<^<<>><^>><>v>^^vvv^<^^<^^>^<>^^<>^><^v<^v<<v>v<>v^^v>^v^>^>^><vv^><<^^^<^^<<v<^><^vv>v^<^^>>vvv^><>>v>><^^<^v^v>>><<v<><<v<>^^>^<<v<>v^v^<>^><><><v<^<^vv^v><v>><^^v^<^<>v>><<>^<><^><^>^v<v^^<>
^^^>vv^v>^<v>v<>>>^v<v^<<v>>^^<<>v<><<^vvvv<v><^<>><>^<><><>v^^<<v<<<>^>>><>>vv^^>^>>^<<^v><>v<<v<><^>^^>>><<^>v^^^><v<^^>>><^^<^<^><^><<v^>^><^<^^v>^<<vv^^>^<<v^v^^^^^<<<<^v>>v^<<<>^v^v<v^>>><>v><<<^<vv^^^<^<^^>><>v<^vv<>^v<<v>^v^><vvvv<<>v><vv><<>^<>>^^>^<>>v>vv<<v^^<<^vv<v<<v>><^^>^vv>v>^<<><v>^v>^^><v^>>^>>><<v<^^<^v^vv>^<^^<^^>v<^^><<vvv<><^>>^vv^<>^v^<<v>v<v^vv^<^>v^v<^v^>^<<vv<>vv>v<<^<v<v<^>v><^>^v><<><>^>><<<^^>v><v<v^^>><v^v>>v<>v^v<^^>v^>>>v<<<v>vv>v^^^<v>vvv<v<>v<>>v>>v><<>v>vvv><<<^^<v><>v<^vv<>><^>>>v>><^>>^v><^>vv>>^vv<vvv<v>v<^><<^^>vv^^<>^v^><vv<>>>^<v^>>><vvv>>>>vv>>>v>^<^^<<>^<^v><>v<^^^<^^^v<<<<v>>><<<^v<^^^<^vvv<v>>>>^^^v<^<vv^^vv^><^>><<<^v^><<<v^vv<>^^^<^v^<>><><^^>^<<^v><<<vv<v<v^<^><>v^<<vv>>^<v>^^v^><vv>v<^><^v><><^><<v^>><><>v><^<^v<v<vv>vvv^^>^<<^<<^>^>^<<^<^^^v>>^v<>^><<<<v^v><<^>^vvv<v^^^<<^<v<^^vv<v^v<v^v^v^v<>>>>^v<v>v^><^^v>v^^<<^^<<><^v<><<^>>>>v<^^v<v>><<^v>><^v^<>v<<><>^><>^<<<^>v^^<vv<^^><v>^^v^v<v<<>^<>>><<v^v^v<<>>^<v<<v<^<>><v<v<v^v<vv>^>><>^v^^^
^<^^>>>>v<v>>vv>><<^v>^^v^v^^v^^vv>v<^^v<vv<>>^><<><^vv<^<vv>v<>^><<^v<><<>vv^^<vv>^>v<^>^v>vv^^^<^v^v^<<><v^v<vv>^^>>^^>^^v<^^v<<^<^^v^>>^<v>^><<v>^><>>v<^>v<^>>^vvv>v><<^><^v<v<<<<v<^^>>>v<v>>v>^<>v<^vv^^>>vvv^<v<>>>v<v^<<<>^>v^^><^>^><>>>^v>^^^v>^v<v>>^>>^<<><>>vvv<<^v>v>^<^<>vv^>>>>><^<v<<v>^>^><v^><^^>>>^^><^<<>^>>^>^><^<>^^v>^>^vv><v>><v>v^>vv^^^>^v^^<v>^<vv^<<>v^<>><>^^<^v>>^<v^><>v^v^v>vv><v^v>>^v<v<^<>>>^v^vv^>v<v^>><^<^<v<^<^<^v<^v<<>^v^<v><<<<>v^v>>v><^<vv>^^<<^v<>^v<>^<v<v^<<>><<^<^^vv><><^>v^>><<<>>^<<<v<<v>v^^v^v^^<v>v^^>^vv^>>v<<>vv>^v<vv>^<<vv^vvvvv<v>>^>><<^^<^^<^v>v^v^v^<^v<^v<v>v^<^v>>>^<^>^^^><<<><^^>>>v<>^>>v>>>v>^>v^<>><^<vv^<v><^>^v<v>><v<<^<<v^<^<<>^>vvv<^><v^^>^>><<<<>>v>>>><^^>vv^>>v^v^^v>>^><>v^^v><vv<<^v^><^<>^vvvv<^<>v^^^^<^<>><<<>^><v<^<^><v<<<<v^>^>v^>v^vv>vvv^<v^v><>vv<^<><>v>vv>^v^^vvvv^vv<^>><>^v>>>^<<^^<v>v^v><<><^^>v<^><^<>^>vv^^^^><^v>v><>v^>^vvv>>>>>^<^><v^vvv>>v^^>>>v>>vv^^>>v>^<^v^^>^>vv<<^v<<<vv>v>>v^><v<v<vv^>>vv^^v^v<^^vv>^v^>^^v<v<v>>^<vv<^<<
<<>>^>^>>v>vvv>^vv><^<vvv>>^^v>>^v<v^>>><<v<<v>^^>v>^>><<v><<<^^<><>^^>>^>v<>^<^>v>vv<^>^v<>>v<^>v<>^<^<^^vv^v<^<v<^>>v>>v><v>>><>v<<<^^<v<^>v<^vv>v<^v<vv^><^vv<><^<><<^v>>>^v>v^>^>>>vvvv>^>^^v^vvv>v<>^<^<^^>v<>^v^^v^<^v^^<v<>vv>vvv^^<v<><<<^^>v<>>^<^^^><^<<<^<^<v^>^>^<^^vvv<>v><vvvv>><>^>^^v^^>>v<vv>v>v>v>^vv>^<v<^><v^^<>^>^>v^v<^^v<>^>v>vvv>^<v>>^^>v<><v<^><^v>>>vv>v<^>>v>>^v<><^>vvv^<<^>v<>vv<>^^<v<>^<><>^<<^^>>vv><<v<>>>>>>vvvvvv^^>v<^^^^>>>>^<<vv^<^>v^^<^^vv^<^>v^><^^<<<>>^^><^v>>>><<^v^v<v<v<><<><^^v<v<^><<v^>^v><<>^><v^^^^>^>^<v<<>v<v>vv><<<<<><v^<^vvv><<<<^vv<<<^^vvvv>>>^<v<v^v>^>>>v>><<^v<<<vv><^<v>>vv>^<^<^>^^<^<<><^>^v>>^vv^>^v>vv>vv<<^^>^^<>v<><<>v>v^^><v^<>>v<>v^^^^<^><>>v^^>^<^^^vv^^><<vv>>^^v<><v<>v<v^<>vv>^^^>^^vv^v^v<^^<>^>>>v^^>^^<^><>vv>v>^v<<<^>^<>>>v<v^<v^>v^>vv<vvvv^<<<<v<>^^v<<>><^>vv><^vv>>v<>^^>>^v^^><^^>^^<<v^>^<>>^><v^<>v^<^v^^><^<v>v^v<^>><<v>v<^v^>vv<^>>v^v<^<^vv<^<<^v>v^v>v<^v^v>^^>>>^<><^>v>v^^<<>^<>>v^<><<^^<vv<<<v<v<<>v<><<>v^<v<<<^^>^<^v<v<><<>^vvv>^<<
^<vv^^>v><<v^<^vv<v^>v><>>>>>v>>vv<^<<v<v<>vv^<><<v<^v>^^><v^<><^>v<<^v><<vv^><^<>^^<>v<vv<<<^v>>^>vv>vv<^<>>vv<^<v>^>vv^^<^><><<^v<^><<^<>>vv<<<><>v>>v><><><>^^vvvv>^><vv<v>>^<<>v<<<^<<v<vvv^<>>^>v<<^v^><^><<v<>vv<><v><<^<v>vv>>^>v<<<^<vv>>>><v^<>v<^^v<v^^>v^<>>><>v><<<^<<v<<<^>v>><^vvv^<v<v><^>^v>^<v>><><v<<^><>^v^>v<v<^<>^<vv>^>v>^v>v>^>^v<^^><^v^>>>^<<^<v<v>><><vv^^<>v>^vv>>>>v><^>>^><^^<^^vvv<^v<<><<<<v^>^>^<<v>^<^>><v<v^<^^>^<v^<^<v^><v^^^vv><>^><>v<>>><vvv<<v>v^v>>v^<v>>^v><<<<^v>^vv^v>vvv<^><>>^>v<^v>^>^>vv<<>v^v^^>^>^v<<^v<>vv><^^v^<>^^^^>v<v^<<v>vv>><^v<<<<^v^>^v^v<vv<><^<<<>v^><^>v^^v><<>>v<^<<>>>v>^>^v>v^v<vv<^>^>vvv>>>vvvvv<>><<^^<>vvv<<v>v<<>v^^vv<<^^>v>^^v<>>>v><^><>v^^<><>><^>^>>>v^<><vv^^><><>vv^<v>>>v^v<>>v><<>><^^>v<^^<^v><><><<v<v<^^vvv<<^<><>>><v^>>^><<^<^><><^^>>v<<>^^^<><<<>v<^^v<vvv><<<v^>^^>vv<^^>>^>^^<^<<v>>v<<^v>^>><v^>>^<v>>>v><>>>v^>^v><>>v>><v<^^<^v>^v>v>^>^<<<^<^v<v<<><v^>^vvv>^<v>v^^<v>^v^>><vv^^^v>vv>v>vv>^>v^v^>>>><<^v^^>^v<v<v>^^v^vv>v^<^^><<^v^<>v><v
>^^v><<>><v^>>^>v<vv>v>^>v<<<<>>^>^<v^vv^<v>^<^<^v<<^v<>>v>^>>>^<>><<^^^^>^^<<^^><^<>v>>^<>><^v>><<^^v^^>v^^v^^>v>vv<v><^vv<>vvv<<v><v^v<<v^<>v^>>>v^^^^^v><^vvv<v^^<<v>^>><v^>v^<^v^<vv^>^<<^^^^<^v>v>v<^^>vv>^><>v^<v>v<>v<<v^v^v^^<^v^>v>>>v>>^>^^v<v><^vv>^^<<^>^v>^vv^<>^<v<<v><>v<<^><^v>>^>^v^<>>>^vv^>>^>>^><>>vv^<^><>^><^><<>>^^<<v><v^vv^><^<v^v^v>v><<<<v>>>>^>>v>><<>v<v^>v^><>>^v>v<>vv>>^>^v>>^v^<<^<v^<^><>vv^^v<v><>><><^>v>^^^^vv<>v^<<>>^><>^^v^v^<<>v>>v>><<>>^<^<^<>^^>^v<<<><^v<^<<<<v^<<<<^<v<>vv^<^^<^^v<v^^<^^v<^v<vvv<<^>vv<>><<<^>>^vv^<<>^v<>><<^>^^>><<><><^v<><><>vv>>>>v>^>^<>^<><v<v>>v^>v><v<v>^v><^>>v<<<<>^>v>v^>>^>^^^>v><<>v><v>>^<v>><<v>v^><<v>^>>^v<<<^v>v^^>^^^<v<<><v>^^<<>v^><^<<><v>^^>^><<^<>^<v^>>>^>v^<^^^>v>>><v<^>v>><v<><>>vv<vv>^^^v<>>^v^v><<<^^v<><>^<v><^^v>>>>^^v>v^>>>v>v^<v>vv^>v<^^<v<v^vv^<^vv<v<>>v>>v^>^vv^<v^>>>>>^^>^^v<^>>^^^>vvv<^>><<<vv>^<><<>v>^<^^^v^v>v<<^>v<>>><<<<v^>v>>vv>>^^^v>^v<><^>^<v<><>^^^<><>^>v><vv^v<v>v^^v<><^v^^v<<v<>>>v<v^<^^v^^><vv>^>>vv>>^<^v>
<<^<<><^<^v<<>>v>>v^vv^vvv>^>^v>>>>>>^>>^>vv><v^<><^>><<<^<vv>>>^>>>v^>><>^>v^^^<^<>^>^^vv>v<^<^>><<<v<v><>vvv^v^>v>^>^vv<>>^<>v^v<^>vv^v^^<^^vv>vv^<>^<^<<>^v<v^v><^<>^v^>v>^^><vv^<^>>>><>v^<^^<<<vv^v>>^^^><v^^>>^<v<<<<^><v^><^^><<>v>^v^><vvv^<<v^><^>vv^^^v<>><<<<^<>>>vv^><v>v^vv>v><><><><<<>^v<>v<v<>>^v<><vv>^v>v^^v<^<<><>>v^>>vv^<<<v^<v>>^v<>>>>><><<>^<^^<vvv>^^<<v>>^<^><^vv<><<><v^>>^<v^vv^<<<^>v^v^><^^<>^^^<>v<><<>>vv><<^<<vv>^>^<vv<^<>vv<<<v^v>^>>v>^>v^<vv>vvvvv>^^>vv^<^^<vv<>vv^^^^v^<>v>>>^v^^<>>v<>vv^v>>v<^>vv^v>>v^v<<>v>^v<>>><v>>v>vv^^^^^<<>^<>>><>>v^v<>v<>^^>>v<<^><v>><v^v^<v<<^>><>>v><^^<<><^<>^vv>><^>vv^^vv>v>>^^v>^v<>><^v^>>^<<<>^v^v<><^v^vv^^<>^>vv^^vv<<^v^>>^vv^^v^><<^<><<^^^^^><vv^^>v>v^^<<>><^>v>><><^^v><<<^<^vvvvv>><v<>v^>^<><>v><><>>^^<^^vv<^^vv^^v^v<>><v^^<vvv<>>v^<^<><^><<>>vv>^>>>^><v<v^><<>^^><vv><<>^^vvvvv><v^<><v>v>>vv^>>vv>^v><^v>>v^v^vv<<^^^^<v^<v^>v^<v><><<vv^v><<<v>vvvv>v^v><vv>><>>>>^>><>v>>v^><^^^>v^<<<^>v<<^^<>v<>v><vv><vv^vvv<<>v^v><<<v><<<<v^>v<v^v<^<v
^<<v<<v>^>vv^^^><<v>^<>^<<^<^><^v<<v<><<^^^v<^<^>^><<vvv>^^vvvv^^v<^><^^v>v<>><vv^v>>>>>^^^<<^v<^>^<><^vv>^v^>v^>><<v<^^^>v<>^>^><<<vv>^>v^>v^<v<<<><^v>>v><v^>v^vv^<>^>v>^>vv^^^>>>^>><vv<^v<vvv>>v<<^>>>><vv>><<>><v<>>>^v>>^>^<<v<<>^v>^^>^<<>v>>v<^>v>><><^>^<v<^>^^^v>^^^<^<vv<^v<v^^<v^<<^<v^v<^^vv<v<^>vvv<^>>v<^v<^>v^<^><^v><^vv>^>>^v>v><>^<^vv^<<^<vvv^v><<v<><<<><v<^vv^^<^^v>^^^<^v>^^<v^^><>><>v<v>v<v<v^>^>vvv^v>>><<<v^^>>vv><>>>v><v<^^<<><^^<>>>v<v>^<>>v^v>^v^^v<<vv>>^<<v^v>^^^^^>v^><v<v>^><><v>v^>>v^<^vv<<<>v<>^^>>>>>><<<v^^<<<v>^^<<v<^<^<>v^><^vv<<>><v^><vv^v^><^<>>^^>v^><<^>^<><vv>vvv^<^>v>vv<v<>v<^v>^>v^^>^<<>>>>><<><<>v<^v<^<v<^>v<^^<^>^>><^^^<^v>>^^^vv<^^<v<<^v^v^vv^<<<^^<><^>>>^<<<^v^><^v>v>><<>^^^>><v>>^><^<v^^v<>^^><^<^<v<v^>>>^v^<vv^^>^<^^<^v<>^v<>vv<^^<v><^^v>^v<^<<<>><v>><<><v<^v<<^><>^><><<>^v^<v>^^^vv^v>v^^^^<^vv<v^<<>v^vv<^^<<v<vv^<>v^^><^vvv>^<<><^v^><^v^v^vvvv<^^<>^v<<<vv><<^^<^<<^^<v>^>v><v^<^vv^vv>v><<>^<^<vv<<<v<<<><><^v^^v<>v>vv>><v>><^><v<<>^vv>v<<v^><^><<v><^^>v
<v>v><v>^>>v^^>^><vv><<<^v>^><v^^><v>vv>^v^<>^vvv>v<^<<>>v><<^^<<v>^<>><v^^v<^^vv>>>>vv><<<v>^<^^v>>^v>>v<v><v<><><vv<<^>><^>>^v><vv><v^<<><v^<><>^<v^>^^>v^<>><><^>^^>>^vv>^^^v>v<v><<^v<^>>v<v>>^<<vv^^v<>vv^<^^<^v<^<^><><v><v>>><<>^<><vv<<v^<v>^vv<<>^<>v^>^>^v^^><^>><^^<>^>>><v><^<v^vv^^<^v^^<<<<<vvv<<<>vv<v^v><v<vv<>^>v><^vvv<v<>v><><>><^>vvv^^v^<>v<<<<^>^<v<>v<><v><^^v^>vv^<v>^<v<>v>^<<<<^^vvvv^<^v>^^v^v>>><^^>^vv^<>><v^^^v>^^v>^v<vvv<v<<>^>>><v^>><<v<v^<>v^>^>^^v>^>v><^>v><<^v><v>^>v^<v>vv>^<<^^^^<^v<^v<v>^<v<><v<^><<^><<v^<^^^^<^vv<<<^v^>>><<v>>>^>>>><vv<<<><v^><^v<v<^<<>^>vv>^><<><><>>v<^<><>>>^vv>vv^<v<vv^<v>^<<<>vv^>v<v<vv^^^<^^v><^>>vvv>>^<v<^<v><v>><<<<<^<>^v>><<^<<^v>^>^^^^vvv<vvvv>vv^<v^^>>>vv^<>^v^>vv>vvv^^^v^v<v>^<^>vv^v><>vv>>^^^^v^>^<<<^>^<>v<^<>v^>v^<^v^><^^^>v^<v>^<><<><v^<vv^v<<<^<^^^vv^<>^>v<^^>>vvv>^^v<^><><^^^><<v<<^>><>^v<v^<^^><v^v<<<v<>vv>>^>>v<^^>>v>^<v><v^<v>^^v^^vv<^<>^<^<^^^v^>^>>v<<vv<v>v<><<v><<v<<<v<^^<<v>v<<^^<>>^^^<<^<>>v<v^<v>^^^^>><^^v>><>>>v<<v><v<<^
>><><<^^^v^^<><^>><>^<^^<<^v^v<<^>>><v^><<v><<<^>^<<v^v^^vvv^^><<>v^><>^^^<><<><^>v><^<>^vv^><vv<v^^<v<^vv<><v><vv<^<^><v<v^>^^>^v<>v^<<^^vvv<>>vv^<<v<^^>><><^^>vvvv>^>>>>>^>>>v^><v>^>>v<^^^<<v>^><>>^>><>^<<>vv<v>^<>^<<^<>^^<>vvvv^vv<<<>v><>^<^>>^<vv^vvv>v^v^<>>v^^>>v<v<><^^vv>vv<<vv^v>>>v^^vv><<>^v<><vv^<v>vv>^>v<vvv<vv^><><vv>^<^>^vv^<v>^v>v^^v<vv<><^<v^^v^><>^<<>v>^^<v>v><vv<>v^>>>><^v<<v<<vv>v^<v^v><v>v><vvv<v<^^>><>><<>^>>>^^v>^>v><^^vv>v>>><v>vvvv<v^v^vv<v<^>^^^<v<^>v<>><<>>vv^>^<v^^>v<<^<v<vv^^<^^vv^>v^v<^<vv^>>>><<<^>v><v^^><^<<^v>^v>^><>^^><>^<><>><>^vv>><vvvv<^^>>^^>^^vvv^<v>v^<>v<v<v^^<^^vv<^^><<v^^<^<^v>>v<v<>v<^<^v<vv<<^<<^^v>>v^^^>vvv^^^v<^^<<<^^<^^<>>>v^>^<^^v><>>>v^v><>v^<>vv^<<^>^v<>>vv>vv^vv>^^<>v^v>vv^<>><^^><>><>v<<^^>><>v^^^^<>^>v^^<v>v<^v^<>^>v>^>>>>>>^^><v<>v>>^vv>vv>><^>v><>>>>>v<<v^v>^^<>^v^^^^^v^<v>v<>v>>v>vv^><v<<<>>^vv^<<^>v<^^<^^<v>^v<v<v>>>vvvv>>>>>^<^^<<^v><^^><vv^<v<<>v^^v>^<^>^vvv<>>vv^v>^>>^^^>v>v<>>v^vv<<>^^<^vvv>^>^>^><v>v^<>^^>v<v^<<><>><><v^<v^^vvv";
