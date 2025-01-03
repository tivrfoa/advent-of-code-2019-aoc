use crate::intcode::*;
use crate::util::draw_grid;
use std::collections::HashMap;

pub fn p1(input: &str) -> i64 {
    let mem_vec: Vec<i64> = input
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut mem: HashMap<usize, i64> = HashMap::with_capacity(mem_vec.len() * 2);
    for (i, v) in mem_vec.into_iter().enumerate() {
        mem.insert(i, v);
    }

    let mut panels: HashMap<(i64, i64), i64> = HashMap::with_capacity(mem.len() * 2);
    let mut pos = (0, 0);
    let mut dir = 0; // 0 up, 1 down, 2 left, 3 right
    let mut prog = Program::new(mem);

    let mut panels_painted_at_least_once = 0;
    let mut qt_output = 0;
    loop {
        if let Some(color) = panels.get(&pos) {
            prog.run_input(*color);
        } else {
            prog.run_input(0);
        }

        let olen = prog.output.len();
        if qt_output == olen {
            break;
        }
        qt_output += 2;

        let color_to_paint = prog.output[olen - 2];
        let dir_to_turn = prog.output[olen - 1];

        if !panels.contains_key(&pos) {
            panels_painted_at_least_once += 1;
        }

        panels.insert(pos, color_to_paint);

        match (dir, dir_to_turn) {
            (0, 0) => {
                dir = 2;
                pos = (pos.0 - 1, pos.1);
            }
            (0, 1) => {
                dir = 3;
                pos = (pos.0 + 1, pos.1);
            }
            (1, 0) => {
                dir = 3;
                pos = (pos.0 + 1, pos.1);
            }
            (1, 1) => {
                dir = 2;
                pos = (pos.0 - 1, pos.1);
            }
            (2, 0) => {
                dir = 1;
                pos = (pos.0, pos.1 - 1);
            }
            (2, 1) => {
                dir = 0;
                pos = (pos.0, pos.1 + 1);
            }
            (3, 0) => {
                dir = 0;
                pos = (pos.0, pos.1 + 1);
            }
            (3, 1) => {
                dir = 1;
                pos = (pos.0, pos.1 - 1);
            }
            _ => panic!("{dir} - {dir_to_turn}"),
        }
    }

    panels_painted_at_least_once
}

pub fn p2(input: &str) -> usize {
    let mem_vec: Vec<i64> = input
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut mem: HashMap<usize, i64> = HashMap::with_capacity(mem_vec.len() * 2);
    for (i, v) in mem_vec.into_iter().enumerate() {
        mem.insert(i, v);
    }

    let mut panels: HashMap<(i64, i64), i64> = HashMap::with_capacity(mem.len() * 2);
    panels.insert((0, 0), 1);
    let mut pos = (0, 0);
    let (mut dx, mut dy) = (0, 1);
    let mut prog = Program::new(mem);

    let mut qt_output = 0;
    loop {
        if let Some(color) = panels.get(&pos) {
            prog.run_input(*color);
        } else {
            prog.run_input(0);
        }

        let olen = prog.output.len();
        if qt_output == olen {
            break;
        }
        qt_output += 2;

        let color_to_paint = prog.output[olen - 2];
        let dir_to_turn = prog.output[olen - 1];

        panels.insert(pos, color_to_paint);

        if dir_to_turn == 0 {
            (dx, dy) = (-dy, dx);
        } else {
            (dx, dy) = (dy, -dx);
        }
        pos = (pos.0 + dx, pos.1 + dy);
    }

    draw_grid(&panels);

    prog.output.len()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(2418, p1(IN));
    }

    #[test]
    fn test_p2() {
        // Not testable.
        // Should print GREJALPR
        assert_eq!(498, p2(IN));
    }
}








// ---------------------- INPUT
//
pub static IN: &str = "3,8,1005,8,328,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,101,1,10,10,4,10,108,1,8,10,4,10,101,0,8,28,1006,0,13,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,1002,8,1,54,1,1103,9,10,1006,0,97,2,1003,0,10,1,105,6,10,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,1,10,4,10,1001,8,0,91,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,102,1,8,113,2,109,5,10,1006,0,96,1,2,5,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,0,10,4,10,102,1,8,146,2,103,2,10,1006,0,69,2,9,8,10,1006,0,25,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,0,10,4,10,101,0,8,182,3,8,1002,8,-1,10,101,1,10,10,4,10,108,1,8,10,4,10,1001,8,0,203,2,5,9,10,1006,0,0,2,6,2,10,3,8,102,-1,8,10,101,1,10,10,4,10,108,1,8,10,4,10,1002,8,1,236,2,4,0,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,1002,8,1,263,2,105,9,10,1,103,15,10,1,4,4,10,2,109,7,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,0,10,4,10,1001,8,0,301,1006,0,63,2,105,6,10,101,1,9,9,1007,9,1018,10,1005,10,15,99,109,650,104,0,104,1,21102,387508441116,1,1,21102,1,345,0,1106,0,449,21102,1,387353256852,1,21102,1,356,0,1105,1,449,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21101,179410308315,0,1,21102,1,403,0,1106,0,449,21101,206199495827,0,1,21102,414,1,0,1105,1,449,3,10,104,0,104,0,3,10,104,0,104,0,21102,718086758760,1,1,21102,1,437,0,1105,1,449,21101,838429573908,0,1,21102,448,1,0,1106,0,449,99,109,2,21202,-1,1,1,21102,1,40,2,21102,480,1,3,21101,470,0,0,1105,1,513,109,-2,2105,1,0,0,1,0,0,1,109,2,3,10,204,-1,1001,475,476,491,4,0,1001,475,1,475,108,4,475,10,1006,10,507,1102,0,1,475,109,-2,2106,0,0,0,109,4,2101,0,-1,512,1207,-3,0,10,1006,10,530,21101,0,0,-3,21202,-3,1,1,21201,-2,0,2,21102,1,1,3,21102,549,1,0,1105,1,554,109,-4,2106,0,0,109,5,1207,-3,1,10,1006,10,577,2207,-4,-2,10,1006,10,577,22102,1,-4,-4,1106,0,645,22102,1,-4,1,21201,-3,-1,2,21202,-2,2,3,21101,596,0,0,1106,0,554,22101,0,1,-4,21102,1,1,-1,2207,-4,-2,10,1006,10,615,21101,0,0,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,637,21201,-1,0,1,21101,637,0,0,106,0,512,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2106,0,0";
