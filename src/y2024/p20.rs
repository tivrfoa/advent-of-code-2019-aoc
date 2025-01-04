use crate::util::*;

pub fn get_min_distances((start_row, start_col): (usize, usize), g: &[Vec<char>]) -> Vec<Vec<usize>> {
    let rows = g.len();
    let cols = g[0].len();
    let mut dists = vec![vec![INF; cols]; rows];
    // dummy first value that will be skipped
    let mut pq = vec![(0, 0, 0), (0, start_row, start_col)];
    let mut i = 0;
    while i + 1 < pq.len() {
        i += 1;
        let (steps, r, c) = pq[i];
        if dists[r][c] != INF { continue; }
        dists[r][c] = steps;
        if g[r][c] == '#' { continue; }
        for (nr, nc, _) in dirs(r, c, rows, cols) {
            pq.push((steps + 1, nr, nc));
        }
    }
    dists
}

pub fn p1(input: &str) -> usize {
    let mut answer = 0;
    let g = input_to_char_grid(input);
    let rows = g.len();
    let cols = g[0].len();
    let (start_pos, end_pos) = {
        let (mut start_pos, mut end_pos) = ((0, 0), (0, 0));
        for (r, row) in g.it() {
            for (c, v) in row.it() {
                if *v == 'S' {
                    start_pos = (r, c);
                }
                if *v == 'E' {
                    end_pos = (r, c);
                }
            }
        }
        (start_pos, end_pos)
    };
    let A = get_min_distances(start_pos, &g);
    let B = get_min_distances(end_pos, &g);

    let normal = A[end_pos.0][end_pos.1];
    for r in 0..rows {
        for c in 0..cols {
            if g[r][c] == '#' {
                for (nr, nc, _) in dirs(r, c, rows, cols) {
                    if g[nr][nc] != '#' && B[nr][nc] != INF {
                        let here = A[r][c] + B[nr][nc] + 1;
                        if here <= normal - 100 {
                            answer += 1;
                        }
                    }
                }
            }
        }
    }

    answer
}

pub fn p2(input: &str) -> usize {
    let mut answer = 0;
    let g = input_to_char_grid(input);
    let rows = g.len();
    let cols = g[0].len();
    let (start_pos, end_pos) = {
        let (mut start_pos, mut end_pos) = ((0, 0), (0, 0));
        for (r, row) in g.it() {
            for (c, v) in row.it() {
                if *v == 'S' {
                    start_pos = (r, c);
                }
                if *v == 'E' {
                    end_pos = (r, c);
                }
            }
        }
        (start_pos, end_pos)
    };
    let A = get_min_distances(start_pos, &g);
    let B = get_min_distances(end_pos, &g);
    let M = 20;

    let normal = A[end_pos.0][end_pos.1];
    for r in 0..rows {
        for c in 0..cols {
            if g[r][c] == '#' { continue; }
            for nr in if r < M { 0 } else { r - M }..=(rows - 1).min(r + M) {
                for nc in if c < M { 0 } else { c - M }..=(cols - 1).min(c + M) {
                    let abs_diff = r.abs_diff(nr) + c.abs_diff(nc);
                    if abs_diff <= M {
                        if g[nr][nc] != '#' && B[nr][nc] != INF {
                            let here = A[r][c] + B[nr][nc] + abs_diff;
                            if here <= normal - 100 {
                                answer += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    answer
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_p1_sample() {
        assert_eq!(171, p1(SAMPLE));
    }

    #[test]
    fn test_p1_in() {
        assert_eq!(1293, p1(IN));
    }

    #[test]
    #[ignore]
    fn test_p2_sample() {
        assert_eq!(171, p2(SAMPLE));
    }

    #[test]
    fn test_p2_in() {
        assert_eq!(977747, p2(IN));
    }
}


// -------------------------- INPUT

pub static SAMPLE: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

pub static IN: &str = "#############################################################################################################################################
###...#.......#.....#...#...#...#.......#.......#.......###...#...###...#...#...#...#.......#...###...#...#...#.....#...#.............#...###
###.#.#.#####.#.###.#.#.#.#.#.#.#.#####.#.#####.#.#####.###.#.#.#.###.#.#.#.#.#.#.#.#.#####.#.#.###.#.#.#.#.#.#.###.#.#.#.###########.#.#.###
#...#.#.....#.#...#...#...#...#.#...#...#.....#.#.#.....#...#...#.#...#...#...#.#.#.#.....#.#.#.....#.#.#.#.#.#.#...#.#...#...#.....#...#...#
#.###.#####.#.###.#############.###.#.#######.#.#.#.#####.#######.#.###########.#.#.#####.#.#.#######.#.#.#.#.#.#.###.#####.#.#.###.#######.#
#...#.###...#.#...#...#...#...#...#.#.#...###.#.#.#.....#...#.....#...#.....#...#.#.....#.#...#.......#.#.#.#.#.#...#...#...#...###.........#
###.#.###.###.#.###.#.#.#.#.#.###.#.#.#.#.###.#.#.#####.###.#.#######.#.###.#.###.#####.#.#####.#######.#.#.#.#.###.###.#.###################
#...#...#...#.#.....#...#...#...#...#...#.#...#.#.#...#.#...#...#...#...###.#...#...#...#.#.....#...#...#.#.#...#...#...#...................#
#.#####.###.#.#################.#########.#.###.#.#.#.#.#.#####.#.#.#######.###.###.#.###.#.#####.#.#.###.#.#####.###.#####################.#
#.....#.#...#...#.........#...#.....#.....#...#.#.#.#...#...#...#.#...###...#...#...#...#.#.#...#.#.#...#...#...#.#...#.....#...#...........#
#####.#.#.#####.#.#######.#.#.#####.#.#######.#.#.#.#######.#.###.###.###.###.###.#####.#.#.#.#.#.#.###.#####.#.#.#.###.###.#.#.#.###########
#...#.#.#.#.....#.......#.#.#.......#...#...#.#...#...#...#.#...#...#...#...#...#...#...#.#.#.#...#...#.#.....#...#...#.###...#...#.........#
#.#.#.#.#.#.###########.#.#.###########.#.#.#.#######.#.#.#.###.###.###.###.###.###.#.###.#.#.#######.#.#.###########.#.###########.#######.#
#.#.#.#...#.#...#.....#.#.#.......#.....#.#.#...#.....#.#...#...###...#.....#...#...#...#.#.#.#.......#.#.#...#...#...#.......#...#.#.....#.#
#.#.#.#####.#.#.#.###.#.#.#######.#.#####.#.###.#.#####.#####.#######.#######.###.#####.#.#.#.#.#######.#.#.#.#.#.#.#########.#.#.#.#.###.#.#
#.#.#.....#.#.#.#...#.#.#.#.....#.#.###...#.....#.#.....#.....#...###.......#...#...#...#.#.#.#.#.....#.#.#.#.#.#.#.#.......#.#.#...#.###...#
#.#.#####.#.#.#.###.#.#.#.#.###.#.#.###.#########.#.#####.#####.#.#########.###.###.#.###.#.#.#.#.###.#.#.#.#.#.#.#.#.#####.#.#.#####.#######
#.#.......#...#.....#...#.#...#.#.#...#.........#.#.#.....###...#...#...#...#...#...#.....#...#...#...#.#.#.#...#...#.....#...#.#.....#.....#
#.#######################.###.#.#.###.#########.#.#.#.#######.#####.#.#.#.###.###.#################.###.#.#.#############.#####.#.#####.###.#
#.#...#...#...#.........#...#.#.#...#.#...#.....#...#...#...#.#.....#.#.#...#.###.......#...........#...#.#...#...........###...#.....#.#...#
#.#.#.#.#.#.#.#.#######.###.#.#.###.#.#.#.#.###########.#.#.#.#.#####.#.###.#.#########.#.###########.###.###.#.#############.#######.#.#.###
#...#...#...#...#.......#...#.#...#.#.#.#.#.....#.......#.#.#.#.#...#.#.#...#...#.......#.....#...###...#.###.#.#...#.......#...#.....#.#...#
#################.#######.###.###.#.#.#.#.#####.#.#######.#.#.#.#.#.#.#.#.#####.#.###########.#.#.#####.#.###.#.#.#.#.#####.###.#.#####.###.#
#.....#...#...###.......#...#...#.#.#.#.#.#...#.#.#.....#.#...#...#.#.#.#...#...#...#.........#.#.....#.#.#...#...#.#.#.....#...#.#...#.#...#
#.###.#.#.#.#.#########.###.###.#.#.#.#.#.#.#.#.#.#.###.#.#########.#.#.###.#.#####.#.#########.#####.#.#.#.#######.#.#.#####.###.#.#.#.#.###
#...#...#.#.#.#...#...#.#...#...#.#.#.#.#.#.#...#.#.#...#...#.....#.#.#.#...#.#...#.#...#...###.#.....#.#.#...#.....#.#.#...#.#...#.#.#.#...#
###.#####.#.#.#.#.#.#.#.#.###.###.#.#.#.#.#.#####.#.#.#####.#.###.#.#.#.#.###.#.#.#.###.#.#.###.#.#####.#.###.#.#####.#.#.#.#.#.###.#.#.###.#
###.#...#...#...#...#...#.###...#.#.#.#.#.#...#...#.#.#.....#...#...#.#.#...#.#.#...#...#.#...#.#...#...#.#...#.#...#.#.#.#...#.#...#...#...#
###.#.#.#################.#####.#.#.#.#.#.###.#.###.#.#.#######.#####.#.###.#.#.#####.###.###.#.###.#.###.#.###.#.#.#.#.#.#####.#.#######.###
#...#.#.............#...#.....#.#.#.#.#.#.###.#.###.#.#...#.....#...#.#.#...#.#...#...###.#...#.#...#.#...#.#...#.#...#.#.#.....#.#.......###
#.###.#############.#.#.#####.#.#.#.#.#.#.###.#.###.#.###.#.#####.#.#.#.#.###.###.#.#####.#.###.#.###.#.###.#.###.#####.#.#.#####.#.#########
#...#.#...........#...#.....#.#.#.#.#.#.#...#.#...#.#...#.#...#...#.#.#...#...###.#.#...#.#...#.#...#.#...#.#.###.#.....#.#.#...#.#.#.......#
###.#.#.#########.#########.#.#.#.#.#.#.###.#.###.#.###.#.###.#.###.#.#####.#####.#.#.#.#.###.#.###.#.###.#.#.###.#.#####.#.#.#.#.#.#.#####.#
###...#.........#.###.......#...#...#.#...#.#.#...#...#.#.#...#.#...#.....#.#...#.#.#.#.#...#.#.#...#.#...#.#.#...#...###.#.#.#.#.#.#.#.....#
###############.#.###.###############.###.#.#.#.#####.#.#.#.###.#.#######.#.#.#.#.#.#.#.###.#.#.#.###.#.###.#.#.#####.###.#.#.#.#.#.#.#.#####
#...#...#.......#.....#.............#.....#.#.#.#...#.#.#.#...#.#.....#...#...#.#.#...#.....#.#.#...#.#...#.#...#...#.....#...#...#...#.#...#
#.#.#.#.#.#############.###########.#######.#.#.#.#.#.#.#.###.#.#####.#.#######.#.###########.#.###.#.###.#.#####.#.###################.#.#.#
#.#...#...#...#...#.....###...#...#.#.....#.#.#.#.#.#.#.#.###.#.....#...#.......#...........#...#...#.#...#.#.....#.........#.........#...#.#
#.#########.#.#.#.#.#######.#.#.#.#.#.###.#.#.#.#.#.#.#.#.###.#####.#####.#################.#####.###.#.###.#.#############.#.#######.#####.#
#.....#...#.#.#.#.#.#.......#...#...#.#...#...#.#.#.#.#.#.#...#...#...###.#.....#.....#...#...###.....#.....#...#.......#...#.......#.#...#.#
#####.#.#.#.#.#.#.#.#.###############.#.#######.#.#.#.#.#.#.###.#.###.###.#.###.#.###.#.#.###.#################.#.#####.#.#########.#.#.#.#.#
#...#...#.#.#...#...#.................#.#...###.#.#.#.#...#...#.#.#...#...#.#...#...#.#.#.#...#...#...........#...#...#.#.#.......#.#...#...#
#.#.#####.#.###########################.#.#.###.#.#.#.#######.#.#.#.###.###.#.#####.#.#.#.#.###.#.#.#########.#####.#.#.#.#.#####.#.#########
#.#.#.....#.#.........................#...#...#.#.#.#.###.....#.#.#.###...#.#.#...#.#...#...#...#...#.......#.......#...#...#.....#...#...###
#.#.#.#####.#.#######################.#######.#.#.#.#.###.#####.#.#.#####.#.#.#.#.#.#########.#######.#####.#################.#######.#.#.###
#.#...#.....#.......................#.#.....#.#.#.#.#.#...#...#.#.#...###...#.#.#.#...#...###.#.......###...###...#...#...###...#...#...#...#
#.#####.###########################.#.#.###.#.#.#.#.#.#.###.#.#.#.###.#######.#.#.###.#.#.###.#.#########.#####.#.#.#.#.#.#####.#.#.#######.#
#.#.....#.....#...........#...#.....#.#...#.#.#.#.#...#.....#.#.#...#...#.....#.#...#...#...#...#...#...#.......#...#...#.....#.#.#...#...#.#
#.#.#####.###.#.#########.#.#.#.#####.###.#.#.#.#.###########.#.###.###.#.#####.###.#######.#####.#.#.#.#####################.#.#.###.#.#.#.#
#...###...#...#.........#...#.#.....#.....#...#.#...###.......#.###.#...#...#...#...#.....#.#.....#...#.................#.....#...#...#.#.#.#
#######.###.###########.#####.#####.###########.###.###.#######.###.#.#####.#.###.###.###.#.#.#########################.#.#########.###.#.#.#
###...#...#.#...#...###.....#.....#.........###...#...#.#.....#.#...#...###.#.###...#...#...#...............#.........#...#.......#.#...#...#
###.#.###.#.#.#.#.#.#######.#####.#########.#####.###.#.#.###.#.#.#####.###.#.#####.###.###################.#.#######.#####.#####.#.#.#######
#...#.....#...#.#.#.#...#...#...#...........#...#.#...#...###...#.#...#...#.#.....#...#.....#.....#.........#.#.....#.#.....#.....#...###...#
#.#############.#.#.#.#.#.###.#.#############.#.#.#.#############.#.#.###.#.#####.###.#####.#.###.#.#########.#.###.#.#.#####.###########.#.#
#.............#...#...#.#.....#...........#...#.#...###..E#.......#.#...#.#...#...###...#...#...#.#...........#...#...#.....#.#...#.....#.#.#
#############.#########.#################.#.###.#######.###.#######.###.#.###.#.#######.#.#####.#.###############.#########.#.#.#.#.###.#.#.#
#...........#.........#...................#.#...#.......###.....#...#...#.###.#.#.....#...#...#.#.......#.......#...###...#.#...#...#...#.#.#
#.#########.#########.#####################.#.###.#############.#.###.###.###.#.#.###.#####.#.#.#######.#.#####.###.###.#.#.#########.###.#.#
#.........#...........#.................###.#.###.###########...#...#...#...#.#.#...#.......#...#.....#...#...#...#...#.#.#.#...#.....###.#.#
#########.#############.###############.###.#.###.###########.#####.###.###.#.#.###.#############.###.#####.#.###.###.#.#.#.#.#.#.#######.#.#
#.........#.....#.......#.........#...#.....#.....###########...#...#...#...#...#...#...#.....#...###.#.....#...#.....#.#.#.#.#...###.....#.#
#.#########.###.#.#######.#######.#.#.#########################.#.###.###.#######.###.#.#.###.#.#####.#.#######.#######.#.#.#.#######.#####.#
#.........#...#.#...#.....#.....#...#S#########################...###...#.....###.....#...###...#...#...#.......###.....#...#.......#.#.....#
#########.###.#.###.#.#####.###.#######################################.#####.###################.#.#####.#########.###############.#.#.#####
#.........#...#.....#.....#.###.............###########################...#...#...#...###.........#.....#...........#.........#.....#.#.#...#
#.#########.#############.#.###############.#############################.#.###.#.#.#.###.#############.#############.#######.#.#####.#.#.#.#
#...#...#...###...#...###...#...............#############################.#...#.#.#.#...#.............#.....#.......#.......#.#...#...#...#.#
###.#.#.#.#####.#.#.#.#######.###########################################.###.#.#.#.###.#############.#####.#.#####.#######.#.###.#.#######.#
###...#...#.....#...#.........#...........###############################.....#.#...#...###...#.....#.....#...#...#.....#...#.....#.#.....#.#
###########.###################.#########.#####################################.#####.#####.#.#.###.#####.#####.#.#####.#.#########.#.###.#.#
#.........#.#...................#.........#...###.......#######################.#...#.#...#.#.#...#.#...#.......#.....#...#####...#.#...#.#.#
#.#######.#.#.###################.#########.#.###.#####.#######################.#.#.#.#.#.#.#.###.#.#.#.#############.#########.#.#.###.#.#.#
#.......#.#...#...#...#...#...###...........#.....#...#.....#################...#.#.#...#...#.....#...#.....#...#...#.#.....#...#...#...#...#
#######.#.#####.#.#.#.#.#.#.#.#####################.#.#####.#################.###.#.#######################.#.#.#.#.#.#.###.#.#######.#######
#.....#.#...#...#...#...#...#...........#...#...#...#.#.....#.......###...###.....#.....#...#...#.........#...#...#...#...#.#...#.....#...###
#.###.#.###.#.#########################.#.#.#.#.#.###.#.#####.#####.###.#.#############.#.#.#.#.#.#######.###############.#.###.#.#####.#.###
#...#.#...#.#.........#.....#...#...###...#...#...###...#...#.#.....#...#.#.............#.#...#...###...#...........###...#.....#.......#...#
###.#.###.#.#########.#.###.#.#.#.#.#####################.#.#.#.#####.###.#.#############.###########.#.###########.###.###################.#
#...#.....#...........#.###.#.#...#...............#...#...#...#...#...#...#...............#...........#.............#...#...#.........#...#.#
#.#####################.###.#.###################.#.#.#.#########.#.###.###################.#########################.###.#.#.#######.#.#.#.#
#.#.....#...........#...#...#.#.......#...........#.#.#...#.......#...#.#.....#.....#...###...#.....#...............#...#.#.#.......#.#.#.#.#
#.#.###.#.#########.#.###.###.#.#####.#.###########.#.###.#.#########.#.#.###.#.###.#.#.#####.#.###.#.#############.###.#.#.#######.#.#.#.#.#
#...###.#.#.........#...#...#.#.#...#...#.......#...#.#...#...#...#...#.#.#...#...#...#.......#.###.#.#.....#.......###.#.#.........#.#.#.#.#
#######.#.#.###########.###.#.#.#.#.#####.#####.#.###.#.#####.#.#.#.###.#.#.#####.#############.###.#.#.###.#.#########.#.###########.#.#.#.#
#.....#.#.#.........#...###...#...#.......#.....#.#...#.....#...#.#...#.#.#.#...#.........#...#.#...#.#...#...#...#.....#...#.......#...#...#
#.###.#.#.#########.#.#####################.#####.#.#######.#####.###.#.#.#.#.#.#########.#.#.#.#.###.###.#####.#.#.#######.#.#####.#########
#.#...#...#.........#.#.....#.............#.#.....#...#...#.#.....#...#.#.#.#.#...#.....#...#.#.#.....###.......#...#.....#...#...#.........#
#.#.#######.#########.#.###.#.###########.#.#.#######.#.#.#.#.#####.###.#.#.#.###.#.###.#####.#.#####################.###.#####.#.#########.#
#.#...#...#.........#.#.#...#...........#.#...#.......#.#...#.#...#...#.#.#...#...#...#...###...#.............#...#...###.#...#.#...........#
#.###.#.#.#########.#.#.#.#############.#.#####.#######.#####.#.#.###.#.#.#####.#####.###.#######.###########.#.#.#.#####.#.#.#.#############
#...#...#.###...###...#.#.............#.#...#...#.....#.....#...#...#.#.#.#.....#...#...#.#.....#...........#...#.#.....#.#.#...###...#...###
###.#####.###.#.#######.#############.#.###.#.###.###.#####.#######.#.#.#.#.#####.#.###.#.#.###.###########.#####.#####.#.#.#######.#.#.#.###
###.....#.....#.........#...........#.#.###...###...#.#...#.#.....#...#...#.......#.#...#.#.#...#.......#...#...#.....#.#...#.......#...#...#
#######.#################.#########.#.#.###########.#.#.#.#.#.###.#################.#.###.#.#.###.#####.#.###.#.#####.#.#####.#############.#
#.......#...#.....#.......#.........#.#.#.......#...#.#.#.#...###.....#.........#...#...#...#.....#...#.#.....#.....#...#...#.#...........#.#
#.#######.#.#.###.#.#######.#########.#.#.#####.#.###.#.#.###########.#.#######.#.#####.###########.#.#.###########.#####.#.#.#.#########.#.#
#.......#.#.#.###...#.....#.........#...#.#.....#...#.#.#.###.....#...#.....###...#...#.............#.#.....#.....#...#...#...#.....#...#...#
#######.#.#.#.#######.###.#########.#####.#.#######.#.#.#.###.###.#.#######.#######.#.###############.#####.#.###.###.#.###########.#.#.#####
#...###...#...#...###...#.#...#.....###...#.....###.#.#.#.#...#...#...#.....#...###.#.###.....#.....#.....#.#...#...#...#...#.....#...#.....#
#.#.###########.#.#####.#.#.#.#.#######.#######.###.#.#.#.#.###.#####.#.#####.#.###.#.###.###.#.###.#####.#.###.###.#####.#.#.###.#########.#
#.#.............#.#...#.#.#.#.#.......#.#.......#...#...#.#.#...#...#.#.....#.#...#.#.#...#...#...#...#...#.....###.......#...###.#...#...#.#
#.###############.#.#.#.#.#.#.#######.#.#.#######.#######.#.#.###.#.#.#####.#.###.#.#.#.###.#####.###.#.#########################.#.#.#.#.#.#
#.#...#.....#...#...#...#...#.........#.#.###...#.......#.#.#.###.#.#.#.....#.#...#.#.#...#.#...#.#...#...........#.........#.....#.#...#.#.#
#.#.#.#.###.#.#.#######################.#.###.#.#######.#.#.#.###.#.#.#.#####.#.###.#.###.#.#.#.#.#.#############.#.#######.#.#####.#####.#.#
#.#.#...###...#...#.............#.....#.#...#.#.#...#...#.#.#...#.#.#.#.....#.#.#...#...#.#.#.#.#.#.....#...#...#...#...#...#.....#.....#.#.#
#.#.#############.#.###########.#.###.#.###.#.#.#.#.#.###.#.###.#.#.#.#####.#.#.#.#####.#.#.#.#.#.#####.#.#.#.#.#####.#.#.#######.#####.#.#.#
#.#.#.............#.#...#.......#...#.#.#...#.#.#.#.#.#...#.#...#.#.#.#.....#.#.#...#...#.#.#.#.#...###.#.#...#.#...#.#...#.....#.#...#.#.#.#
#.#.#.#############.#.#.#.#########.#.#.#.###.#.#.#.#.#.###.#.###.#.#.#.#####.#.###.#.###.#.#.#.###.###.#.#####.#.#.#.#####.###.#.#.#.#.#.#.#
#...#...............#.#.#.......#...#...#.....#.#.#...#.#...#...#.#.#.#...#...#.#...#.....#.#.#.#...#...#...###...#.#.......#...#.#.#.#.#.#.#
#####################.#.#######.#.#############.#.#####.#.#####.#.#.#.###.#.###.#.#########.#.#.#.###.#####.#######.#########.###.#.#.#.#.#.#
#.................#...#.........#.#.............#...#...#.#...#.#.#...#...#...#.#.#...#.....#.#.#...#.#...#.......#...........###...#.#.#.#.#
#.###############.#.#############.#.###############.#.###.#.#.#.#.#####.#####.#.#.#.#.#.#####.#.###.#.#.#.#######.###################.#.#.#.#
#.........#.....#...#...#.....#...#...#...#.....#...#...#...#.#.#.....#...#...#.#...#.#...#...#...#.#.#.#...#...#...................#...#...#
#########.#.###.#####.#.#.###.#.#####.#.#.#.###.#.#####.#####.#.#####.###.#.###.#####.###.#.#####.#.#.#.###.#.#.###################.#########
#...#...#...###.....#.#...#...#.....#.#.#.#.#...#.#####...#...#.#...#.#...#.###.#.....###.#.#.....#.#...###.#.#.#...#.....#.........###...###
#.#.#.#.###########.#.#####.#######.#.#.#.#.#.###.#######.#.###.#.#.#.#.###.###.#.#######.#.#.#####.#######.#.#.#.#.#.###.#.###########.#.###
#.#...#.............#.#...#.....#...#.#.#.#.#...#...#.....#.#...#.#.#.#...#.#...#.....#...#.#.#...#...###...#.#.#.#...###.#.......#.....#...#
#.###################.#.#.#####.#.###.#.#.#.###.###.#.#####.#.###.#.#.###.#.#.#######.#.###.#.#.#.###.###.###.#.#.#######.#######.#.#######.#
#...........#...#...#...#...#...#...#.#.#...#...#...#.#...#.#.#...#.#.###...#.#.......#.#...#...#...#...#...#.#.#.......#.#...#...#.#.......#
###########.#.#.#.#.#######.#.#####.#.#.#####.###.###.#.#.#.#.#.###.#.#######.#.#######.#.#########.###.###.#.#.#######.#.#.#.#.###.#.#######
#...........#.#...#.#...#...#...#...#.#...#...#...#...#.#.#.#.#...#.#.#.......#.....#...#.........#...#...#.#.#.#...#...#...#.#.....#.....###
#.###########.#####.#.#.#.#####.#.###.###.#.###.###.###.#.#.#.###.#.#.#.###########.#.###########.###.###.#.#.#.#.#.#.#######.###########.###
#.#.....#...#.#.....#.#.#.#...#.#...#...#.#...#.###.....#.#.#.....#...#.....#...#...#.#...#.......###...#.#.#.#...#.#.......#...#...#...#...#
#.#.###.#.#.#.#.#####.#.#.#.#.#.###.###.#.###.#.#########.#.###############.#.#.#.###.#.#.#.###########.#.#.#.#####.#######.###.#.#.#.#.###.#
#...###...#...#.#...#.#.#.#.#...#...###...#...#...###...#.#...#...###.......#.#...###...#...#...###...#.#.#.#.....#.........###.#.#.#.#...#.#
###############.#.#.#.#.#.#.#####.#########.#####.###.#.#.###.#.#.###.#######.###############.#.###.#.#.#.#.#####.#############.#.#.#.###.#.#
#.....#.........#.#.#.#...#.....#.....###...#...#.....#.#.#...#.#.....#.....#.................#...#.#...#.#.#.....#...........#.#.#...###...#
#.###.#.#########.#.#.#########.#####.###.###.#.#######.#.#.###.#######.###.#####################.#.#####.#.#.#####.#########.#.#.###########
#...#.#.........#.#.#.....#.....#...#.#...#...#...#.....#.#...#.#.....#.#...#.....#...#...#.......#.....#.#.#.#.....#.....#...#.#...........#
###.#.#########.#.#.#####.#.#####.#.#.#.###.#####.#.#####.###.#.#.###.#.#.###.###.#.#.#.#.#.###########.#.#.#.#.#####.###.#.###.###########.#
#...#.........#.#.#...#...#.....#.#...#.#...#.....#.....#.....#...#...#.#...#...#.#.#.#.#.#.....#.......#.#.#...#.....###...#...#...#...#...#
#.###########.#.#.###.#.#######.#.#####.#.###.#########.###########.###.###.###.#.#.#.#.#.#####.#.#######.#.#####.###########.###.#.#.#.#.###
#.#.....#...#...#.#...#.#.......#...#...#...#...#.....#...#.....#...#...###...#.#...#.#.#...#...#.....#...#.....#...........#.#...#...#...###
#.#.###.#.#.#####.#.###.#.#########.#.#####.###.#.###.###.#.###.#.###.#######.#.#####.#.###.#.#######.#.#######.###########.#.#.#############
#.#.#...#.#...#...#.#...#.......#...#.#.....###...#...#...#...#...#...#...#...#...#...#.###.#.......#.#.......#.#...........#.#...........###
#.#.#.###.###.#.###.#.#########.#.###.#.###########.###.#####.#####.###.#.#.#####.#.###.###.#######.#.#######.#.#.###########.###########.###
#.#.#...#...#.#...#.#.....#.....#.#...#...#.........#...#.....#...#.....#.#...#...#...#.#...#...#...#.#...#...#.#...........#...#...#...#...#
#.#.###.###.#.###.#.#####.#.#####.#.#####.#.#########.###.#####.#.#######.###.#.#####.#.#.###.#.#.###.#.#.#.###.###########.###.#.#.#.#.###.#
#...###.....#.....#.......#.......#.......#...........###.......#.........###...#####...#.....#...###...#...###.............###...#...#.....#
#############################################################################################################################################";
