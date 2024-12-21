use std::collections::*;
use crate::util::*;

const INF: usize = usize::MAX;

pub fn p1(input: &str, bf: usize, rows: usize, cols: usize) -> usize {
    let mut g = vec![vec!['.'; cols]; rows];
    let mut lines = input.lines();
    for _ in 0..bf {
        let (x, y) = lines.next().unwrap().split_once_to_num::<usize>(',');
        g[y][x] = '#';
    }
    get_min_distances(&g, |r: usize, c: usize, g: &[Vec<char>]| g[r][c] != '#')[rows - 1][cols - 1]
}

fn get_min_distances(g: &[Vec<char>], allow: impl Fn(usize, usize, &[Vec<char>]) -> bool) -> Vec<Vec<usize>> {
    let rows = g.len();
    let cols = g[0].len();
    let mut dists = vec![vec![INF; cols]; rows];
    let mut pq = VecDeque::new();
    pq.push_back((0, 0, 0));
    while let Some((steps, r, c)) = pq.pop_front() {
        if dists[r][c] != INF { continue; }
        dists[r][c] = steps;
        for (nr, nc, _) in dirs(r, c, rows, cols) {
            if allow(nr, nc, g) {
                pq.push_back((steps + 1, nr, nc));
            }
        }
    }
    dists
}

pub fn p2(input: &str, bf: usize, rows: usize, cols: usize) -> (usize, usize) {
    let mut g = vec![vec!['.'; cols]; rows];
    let bytes: Vec<(usize, usize)> = input.lines()
        .map(|l| l.split_once_to_num::<usize>(','))
        .collect();
    for i in 0..bf {
        let (x, y) = (bytes[i].0, bytes[i].1);
        g[y][x] = '#';
    }

    let mut l = bf;
    let mut h = bytes.len() - 1;
    while l <= h {
        let md = l + (h - l) / 2;
        let mut grid = g.clone();
        for i in bf..=md {
            let (x, y) = (bytes[i].0, bytes[i].1);
            grid[y][x] = '#';
        }
        if get_min_distances(&grid, |r: usize, c: usize, g: &[Vec<char>]| g[r][c] != '#')[rows - 1][cols - 1] != INF {
            l = md + 1;
        } else {
            h = md - 1;
        }
    }

    bytes[l]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sample() {
        assert_eq!(22, p1(SAMPLE, 12, 7, 7));
    }

    #[test]
    fn test_p1_in() {
        assert_eq!(234, p1(IN, 1024, 71, 71));
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!((6, 1), p2(SAMPLE, 12, 7, 7));
    }

    #[test]
    fn test_p2_in() {
        assert_eq!((58, 19), p2(IN, 1024, 71, 71));
    }
}


// -------------------------- INPUT

pub static SAMPLE: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

pub static IN: &str = "35,68
1,30
69,36
37,59
49,54
54,59
14,13
45,3
69,31
3,21
33,12
69,68
55,63
48,23
33,14
59,39
7,3
38,29
37,25
4,1
40,53
67,67
37,19
61,60
70,65
54,55
39,14
31,3
49,7
1,9
7,18
51,63
63,53
9,21
11,5
50,65
8,5
37,7
63,42
13,20
33,68
41,69
21,8
58,43
68,57
35,18
53,52
38,31
17,12
45,6
33,24
35,16
61,64
67,49
65,31
32,21
38,53
69,37
47,53
50,67
31,4
32,67
20,13
31,7
65,37
45,66
39,66
5,8
31,25
59,50
67,62
44,51
18,1
39,55
37,16
49,64
56,67
49,67
13,8
43,2
36,67
11,3
51,59
40,31
43,17
55,61
27,9
19,7
8,23
47,65
47,49
33,62
35,7
41,57
43,53
5,3
43,10
36,55
27,7
9,7
11,8
57,55
35,35
33,18
69,34
33,10
69,50
51,54
70,27
57,41
69,29
3,13
33,25
66,59
67,61
42,57
13,23
45,11
23,6
7,24
21,14
15,18
41,7
17,7
43,67
40,23
30,63
36,5
5,11
36,53
56,35
5,29
65,61
58,59
41,28
41,11
19,13
9,12
48,57
25,1
13,13
45,63
3,28
35,22
63,47
2,11
67,43
29,15
1,4
47,59
31,59
33,19
63,69
8,9
12,13
32,55
45,65
67,41
21,1
69,38
63,45
69,69
29,4
29,11
35,69
8,37
9,26
43,49
61,70
1,31
29,24
45,68
62,49
46,19
33,11
7,33
45,17
10,15
65,57
39,24
28,13
53,65
15,5
29,25
48,1
13,4
44,13
61,55
19,19
1,33
5,19
36,25
35,17
33,16
67,46
15,16
46,51
5,30
57,51
29,5
11,22
33,28
8,1
68,53
29,6
23,8
49,21
15,8
69,67
34,31
28,23
56,51
13,17
22,15
1,25
2,23
45,14
43,8
57,60
53,8
43,5
39,23
51,1
35,15
38,69
59,63
37,63
56,45
4,37
51,46
39,4
37,20
7,11
55,65
53,55
63,40
7,14
47,52
35,32
55,70
66,29
65,30
37,33
59,69
35,59
3,11
42,15
63,48
32,9
31,19
59,55
67,63
17,5
36,61
44,61
7,19
43,66
11,9
39,5
41,19
33,60
16,9
26,5
42,53
37,14
61,45
39,61
7,9
58,45
43,4
60,53
69,65
3,33
25,3
9,19
10,3
55,57
13,19
30,3
58,69
60,45
54,47
57,65
1,15
43,13
39,58
44,19
23,4
26,17
40,5
35,13
46,9
35,2
45,59
29,9
31,9
7,0
69,39
41,63
65,53
67,66
49,5
44,49
37,61
49,48
11,7
39,59
65,64
63,39
48,19
34,21
61,69
69,59
59,54
1,1
9,18
59,47
6,25
49,44
63,60
19,20
60,39
53,60
56,57
40,21
11,1
34,61
4,23
35,54
1,13
32,65
31,11
57,69
40,67
39,11
41,67
49,58
38,1
49,55
68,49
68,61
18,7
31,22
37,3
62,33
35,11
60,59
1,2
11,14
29,13
49,4
30,9
48,61
52,65
17,1
21,17
64,45
65,55
58,65
57,61
5,25
54,69
41,2
59,41
67,51
63,63
30,61
50,59
16,13
3,7
1,3
1,21
33,61
66,41
40,69
70,39
3,6
69,41
31,63
27,64
54,43
68,67
63,58
36,19
11,23
59,49
51,55
7,30
20,23
41,58
57,3
19,3
51,9
61,67
7,6
3,12
35,65
53,61
39,56
35,27
2,25
37,0
53,69
66,65
55,39
29,7
35,1
15,3
45,53
59,52
65,43
51,69
31,30
7,12
6,21
33,20
55,64
51,3
35,21
1,45
67,33
9,1
61,59
68,47
3,19
61,58
59,57
39,31
12,1
37,15
23,21
34,53
13,35
1,17
19,16
35,5
21,19
39,17
28,63
27,12
7,28
45,69
1,26
1,6
48,67
43,70
4,9
65,39
39,65
45,57
53,51
1,29
55,69
59,48
51,60
37,53
64,47
41,64
28,7
63,43
63,7
4,3
43,29
15,1
48,55
30,29
61,63
60,65
47,67
35,9
34,27
41,13
43,69
45,15
51,61
55,40
37,17
47,60
50,63
21,9
15,11
17,10
62,51
59,42
69,44
66,43
43,9
47,69
53,57
55,53
68,51
2,29
5,7
35,10
51,57
30,1
46,17
51,5
29,23
59,66
38,7
39,8
37,23
1,8
26,15
17,11
65,41
28,1
20,1
17,9
43,59
64,41
33,55
15,6
39,29
41,51
11,13
27,62
27,3
65,45
48,5
45,55
55,55
46,1
33,29
37,5
35,3
53,64
39,1
37,28
25,5
35,29
1,7
31,26
54,51
1,14
67,29
7,1
53,53
57,54
37,66
7,17
39,62
51,53
40,1
6,5
49,18
45,49
53,63
49,47
9,13
9,10
1,11
41,29
38,55
47,3
69,55
33,63
4,29
50,19
54,67
67,32
18,19
2,37
46,67
47,5
65,54
7,5
67,57
6,9
15,7
41,1
7,13
44,7
8,15
45,1
55,66
47,57
41,25
61,47
4,17
1,5
39,3
42,65
41,31
56,61
37,55
35,31
3,15
51,62
5,13
44,1
45,16
69,35
45,61
31,23
43,32
35,6
35,25
49,20
35,19
61,68
5,23
17,15
43,23
37,65
45,58
15,17
13,10
15,4
21,4
10,5
41,18
45,54
39,13
45,21
2,3
1,18
67,31
25,2
57,63
68,55
44,63
69,49
9,9
7,21
67,44
63,61
47,9
67,47
69,33
61,42
41,61
37,13
43,55
66,39
49,51
47,62
0,33
47,1
31,5
37,27
41,30
39,21
69,64
5,1
68,41
59,45
29,63
38,5
19,9
33,56
15,0
49,59
19,8
53,67
1,27
57,64
15,23
57,49
51,49
5,5
43,65
33,7
41,60
45,56
63,49
51,2
37,69
51,68
67,37
35,53
49,57
33,3
69,45
9,22
41,10
52,53
33,4
16,3
33,2
46,59
4,21
15,13
27,10
27,63
33,15
36,63
61,56
65,36
50,69
4,13
67,60
49,65
43,28
68,37
36,31
52,5
37,12
37,24
63,51
37,11
45,19
12,17
37,22
49,53
43,11
17,3
33,67
42,7
62,65
45,9
59,56
62,53
37,21
43,7
68,63
33,6
35,61
65,65
51,65
43,16
37,29
41,5
43,63
45,4
40,13
39,63
23,1
3,3
63,55
33,27
31,1
0,13
49,63
47,55
43,68
15,10
63,59
54,57
5,22
47,63
56,53
31,24
62,45
43,31
35,70
32,27
69,58
59,53
24,9
9,5
55,13
65,29
39,18
61,48
39,16
69,56
49,61
42,11
47,66
51,56
67,65
34,23
41,17
55,48
66,69
11,12
69,57
65,63
47,64
57,53
51,41
67,34
64,69
65,59
13,1
26,3
11,15
7,15
33,8
3,5
35,63
59,35
40,7
41,59
66,57
39,69
45,7
31,12
53,62
59,67
1,19
13,9
51,67
59,59
22,1
29,1
25,13
45,51
33,33
41,55
59,68
57,4
39,9
14,1
49,50
5,18
51,51
63,56
2,17
50,51
61,41
12,5
17,13
33,30
69,53
65,67
42,61
4,35
58,61
46,55
6,3
44,53
38,19
66,55
39,64
3,24
11,16
26,7
43,12
70,53
65,47
53,19
35,23
39,27
2,15
67,59
37,34
34,65
50,57
67,39
55,51
67,69
50,3
67,45
21,12
3,20
64,63
47,51
8,3
26,9
29,8
39,57
4,5
69,47
33,5
22,3
33,1
11,11
43,57
67,40
55,59
63,38
66,51
2,41
2,1
38,11
13,3
27,61
48,51
69,27
61,53
37,31
5,15
3,23
53,56
24,5
35,58
69,32
37,67
61,49
45,10
3,1
5,14
59,65
41,53
67,30
33,17
67,53
25,9
1,20
57,67
31,14
10,7
30,11
37,2
69,51
61,65
65,62
43,62
61,43
45,5
65,68
63,65
3,8
41,15
65,52
18,3
43,51
39,53
45,64
19,5
42,3
13,11
55,67
70,47
49,69
67,55
17,6
7,16
37,58
28,3
4,27
68,29
55,9
17,14
33,13
9,3
31,13
21,7
46,47
1,23
3,16
37,1
29,3
47,2
27,11
9,15
61,62
31,17
36,29
66,47
38,61
37,64
18,5
42,55
7,38
63,66
39,60
56,47
31,61
19,2
9,20
55,33
58,63
64,57
12,11
31,6
63,57
67,35
55,62
33,9
63,41
36,11
57,57
39,7
28,11
35,4
3,32
19,15
59,46
51,52
41,65
46,5
35,64
13,7
39,15
47,19
41,3
7,20
24,1
6,11
31,16
39,28
35,8
32,1
61,61
66,35
9,11
55,54
58,51
47,70
57,58
5,26
61,57
37,26
52,67
5,17
54,19
3,25
63,54
35,14
40,55
0,27
43,15
65,38
14,3
3,9
3,31
27,1
25,11
3,17
21,15
65,49
41,68
32,63
3,10
66,49
65,35
33,65
47,61
63,44
11,2
5,49
25,7
69,6
9,43
49,23
63,11
19,28
23,11
25,62
10,35
63,37
5,69
17,59
53,43
55,17
5,44
65,4
59,25
67,12
43,37
25,19
41,40
21,45
5,33
1,50
53,1
55,15
34,49
3,54
27,65
65,20
54,23
69,15
43,35
13,5
35,40
24,27
27,18
19,23
63,25
27,60
14,61
7,39
53,47
55,47
11,61
7,7
60,3
59,28
43,34
26,31
16,45
9,66
15,19
63,33
6,43
24,59
37,40
24,21
53,2
11,28
57,45
55,19
57,43
63,67
63,16
43,39
26,61
31,39
21,57
27,38
49,33
5,51
35,50
7,53
22,45
67,27
16,65
56,15
53,21
29,51
16,41
31,68
69,3
47,11
20,51
69,17
35,33
53,33
62,23
4,55
11,17
28,55
14,35
12,19
65,14
64,1
11,26
14,15
65,11
67,5
63,20
31,29
29,53
11,31
25,15
59,14
9,32
8,63
21,55
3,46
47,15
27,52
13,18
44,37
42,37
42,51
57,18
14,53
25,21
37,51
19,47
9,30
20,39
57,19
55,4
32,47
51,44
13,31
37,39
23,68
7,29
23,5
11,19
59,15
65,8
64,33
21,49
10,61
33,49
22,69
29,32
4,59
9,46
26,27
65,17
29,28
47,22
15,25
61,6
51,25
5,64
63,18
13,53
23,47
59,1
10,33
18,37
27,47
63,35
11,47
15,9
10,49
57,35
29,45
59,6
25,40
56,29
51,37
23,42
19,17
25,57
30,17
53,41
57,17
59,34
17,49
10,53
29,26
3,59
61,40
33,39
31,52
57,15
13,59
7,46
11,37
20,43
23,3
62,17
63,28
15,21
10,9
17,45
13,29
61,7
44,29
59,16
36,43
25,35
59,12
27,57
51,10
45,35
63,27
21,35
45,44
2,57
37,43
51,47
59,7
23,61
13,47
59,33
53,13
0,35
25,24
16,53
43,58
47,47
31,58
57,37
63,26
61,29
26,35
56,17
67,6
25,36
32,39
33,36
49,28
55,12
53,28
19,27
21,11
57,33
33,43
45,37
29,56
19,51
1,65
23,45
61,10
29,39
65,1
43,41
29,57
22,61
58,9
27,53
5,57
29,29
9,51
35,51
46,21
17,65
15,37
2,67
57,13
31,33
67,23
1,61
12,33
41,37
41,47
69,25
54,9
53,17
25,51
30,43
19,35
35,45
3,39
53,29
20,35
11,66
55,43
3,69
12,61
23,54
23,30
65,33
5,21
7,65
3,45
31,50
63,1
4,69
23,7
31,32
27,33
39,48
51,43
61,9
23,13
23,49
50,23
21,27
15,51
38,51
39,67
17,34
9,57
58,21
25,69
54,45
25,67
49,24
69,24
30,69
21,56
53,30
31,43
15,31
19,61
34,37
61,1
51,48
1,48
27,69
21,62
8,35
7,55
21,13
27,32
22,41
47,35
44,23
5,9
56,7
4,43
38,49
65,10
25,55
48,27
11,29
45,30
21,31
21,18
50,31
7,37
11,39
37,48
30,21
59,31
59,11
29,17
26,47
55,1
53,9
34,41
55,23
1,57
11,56
9,70
11,27
23,23
69,5
27,22
49,31
41,49
2,39
55,3
25,45
9,54
15,43
9,35
7,67
59,17
15,66
25,58
43,1
23,39
47,45
11,65
61,11
7,61
49,6
7,35
17,38
47,28
53,7
53,39
23,57
39,44
67,17
60,31
52,17
28,29
48,41
61,21
69,63
38,33
23,63
9,53
57,25
33,42
3,53
67,10
33,31
23,32
53,35
21,48
61,51
49,17
63,15
35,49
19,42
39,37
29,37
62,19
27,31
15,53
15,57
41,46
65,51
16,23
11,67
9,25
10,39
55,11
9,33
19,63
51,31
41,20
52,13
10,51
24,65
13,57
21,64
49,25
67,7
55,26
69,8
15,39
19,45
49,46
47,41
47,13
15,65
17,68
13,39
30,49
1,39
63,29
45,43
21,65
53,45
17,47
16,59
5,59
61,36
45,31
7,52
18,27
67,14
10,25
15,58
66,7
13,41
27,27
52,39
29,42
25,34
61,12
7,63
0,53
21,66
69,21
25,28
39,39
19,31
33,53
11,45
31,54
17,25
26,49
43,40
1,55
20,67
61,38
23,31
43,25
7,34
28,35
47,32
27,51
9,37
54,37
17,22
25,30
45,42
49,8
41,27
17,48
11,53
67,26
12,51
57,20
67,25
45,33
31,67
64,67
40,51
53,27
33,41
57,22
57,11
51,39
49,45
9,39
3,57
6,33
19,37
5,65
41,21
15,59
1,56
25,37
5,63
34,57
27,29
14,31
14,27
0,41
23,43
8,59
19,39
33,23
54,41
29,34
19,29
39,51
19,55
15,15
36,47
23,65
11,35
61,37
31,18
25,53
17,55
18,59
20,55
1,51
18,63
32,49
49,26
16,55
3,47
61,5
25,33
56,21
29,67
20,47
1,53
25,43
23,59
27,43
70,15
46,39
20,59
16,25
61,14
7,57
39,41
43,27
26,67
8,53
58,41
7,62
20,25
2,51
21,47
17,36
63,5
46,43
24,53
29,20
28,39
55,37
41,14
50,39
1,47
21,21
55,31
13,33
13,69
33,69
14,49
19,59
51,45
25,56
12,23
69,1
51,7
60,19
50,35
17,32
13,58
59,26
19,18
41,33
22,39
58,5
43,26
44,35
17,19
65,27
36,59
49,15
69,11
22,51
12,47
43,19
45,45
13,45
31,15
40,35
57,1
52,43
69,23
61,15
42,23
41,32
47,7
21,34
49,42
17,17
1,54
4,49
53,15
27,15
24,45
49,13
63,19
68,23
21,63
59,27
45,25
25,63
42,21
65,21
65,7
11,41
8,41
43,18
3,63
49,35
59,13
51,19
15,45
15,42
42,47
61,33
11,69
27,21
25,46
57,2
15,56
61,0
17,29
5,62
15,68
33,35
5,32
57,27
39,33
53,25
42,25
4,41
43,33
47,36
67,16
39,25
35,41
47,31
25,39
47,21
63,22
57,29
49,11
27,42
18,43
23,58
37,35
69,7
68,1
25,41
1,59
14,25
49,9
12,29
1,62
62,3
29,61
19,43
59,43
51,27
9,59
21,30
50,15
9,64
51,33
8,67
17,63
13,60
51,17
21,52
3,37
35,43
23,22
61,2
37,9
7,31
51,22
18,65
23,27
13,61
7,23
23,9
25,44
29,44
61,26
7,68
7,47
24,37
37,47
45,23
23,19
13,56
18,51
4,63
19,11
46,49
5,55
31,40
21,50
29,60
8,45
64,35
19,33
26,13
19,54
27,20
13,64
63,17
21,38
1,37
27,67
3,52
54,29
68,17
1,67
22,23
43,3
57,59
49,19
17,35
21,16
37,41
9,29
21,5
70,11
13,38
51,15
52,25
1,49
2,33
29,47
15,41
21,25
11,42
7,43
67,21
15,63
15,69
53,20
59,5
17,67
13,68
9,23
47,25
3,55
13,37
29,31
7,49
45,67
27,35
47,37
59,9
59,3
68,27
28,45
27,41
21,59
13,66
23,41
17,50
66,23
28,15
8,27
23,51
59,37
67,9
65,69
13,21
27,23
19,53
45,29
12,31
2,43
35,57
5,37
22,57
51,16
1,43
49,14
13,42
70,19
22,19
25,47
19,57
49,12
61,17
17,21
33,34
51,23
41,43
25,18
1,69
53,46
13,54
19,32
27,37
54,33
59,18
25,20
62,11
55,45
55,10
61,3
42,45
46,41
41,36
27,17
49,36
5,31
5,66
3,65
31,38
61,23
56,31
6,65
67,20
7,27
64,7
21,43
29,43
12,25
5,60
9,67
57,40
52,11
36,9
10,45
29,36
9,27
33,21
67,13
29,49
66,3
31,36
32,43
56,9
53,49
33,45
33,51
41,35
37,57
25,59
11,50
45,26
16,31
6,55
31,55
32,33
63,4
21,41
17,41
19,67
23,55
3,48
57,39
16,39
23,64
61,25
9,69
17,53
23,69
60,29
28,51
13,43
18,53
16,51
31,57
54,3
19,46
12,63
31,65
23,37
56,1
67,18
58,25
22,27
57,28
7,51
4,39
31,47
57,38
51,6
17,51
7,25
47,27
60,33
15,64
5,53
34,47
23,53
23,35
41,48
5,41
11,33
39,38
43,42
1,63
49,3
55,5
51,42
58,15
11,55
23,15
59,21
10,21
11,63
47,8
48,39
25,49
11,59
1,58
13,27
11,43
64,51
25,65
25,61
11,21
30,45
5,45
13,67
27,39
24,11
65,9
57,50
11,51
19,1
13,40
17,20
2,65
49,38
65,13
19,10
20,5
47,17
65,19
17,23
31,35
13,46
67,19
37,45
52,27
57,32
3,66
36,37
57,31
41,26
12,69
47,33
23,66
9,63
12,37
67,3
52,37
30,65
49,27
53,31
47,12
55,25
17,62
22,55
63,31
17,37
11,57
56,43
57,23
31,27
37,38
29,65
44,39
7,36
16,29
65,16
59,19
39,46
18,13
25,25
3,43
29,27
7,59
46,25
33,37
60,5
5,56
23,67
1,64
19,26
36,51
48,45
27,19
47,29
61,13
15,61
35,39
5,39
40,11
21,61
9,55
13,15
52,49
19,41
29,55
30,37
17,31
24,17
8,31
57,10
29,59
20,9
21,20
22,25
62,35
45,38
55,49
63,8
27,25
61,31
26,55
33,57
47,23
44,47
45,27
61,27
19,65
57,12
17,56
65,18
62,29
63,62
10,67
69,13
13,63
3,49
49,39
38,41
53,34
29,40
5,35
9,49
49,29
26,43
31,49
22,33
51,29
67,11
6,59
41,34
51,11
61,19
19,64
14,23
49,16
27,59
53,59
55,21
15,35
45,39
57,47
29,33
62,13
6,51
39,35
21,29
41,9
22,11
28,47
3,41
5,43
68,5
17,30
24,15
63,13
15,33
15,67
43,45
25,26
55,22
24,61
55,36
32,57
33,47
45,24
69,9
19,69
34,45
29,70
51,24
44,45
18,35
1,44
25,29
19,22
9,41
57,21
11,58
63,9
42,43
21,44
37,49
19,21
50,29
65,12
25,68
66,1
69,19
5,67
64,5
69,2
19,25
14,51
65,3
57,5
29,19
55,27
29,50
52,41
7,56
20,61
23,38
49,37
10,43
15,47
45,32
23,29
36,35
29,54
19,49
47,10
43,43
49,41
29,58
65,5
43,21
35,55
43,50
39,19
11,25
27,5
5,61
21,3
4,53
15,29
28,25
65,23
39,47
21,33
13,44
14,29
29,66
25,23
69,43
31,46
59,51
9,17
43,61
17,43
55,41
51,21
29,48
64,23
60,21
9,61
67,1
53,3
27,45
47,39
40,39
47,43
28,17
49,43
63,32
31,60
6,41
41,41
31,37
51,35
27,55
5,46
8,43
59,8
14,33
52,31
51,13
3,27
25,50
41,45
25,31
15,55
2,61
35,37
31,31
31,69
62,7
13,51
3,34
25,27
3,61
7,41
68,43
61,39
9,65
13,25
59,61
58,1
3,35
1,35
17,27
13,49
1,46
48,33
17,16
28,31
39,49
65,25
15,44
45,13
46,29
35,42
21,53
57,34
21,37
35,46
29,41
48,11
3,29
17,46
23,36
17,33
63,3
61,24
54,17
58,37
31,45
23,28
62,31
26,39
33,52
28,67
10,63
21,51
65,15
15,48
59,29
5,36
23,17
31,51
35,47
9,31
7,70
14,47
5,27
6,49
53,11
26,65
31,21
35,67
29,35
11,49
39,43
25,52
25,17
68,11
59,23
7,45
58,31
49,1
17,61
31,41
69,61
53,14
41,23
16,27
48,31
29,69
55,35
5,47
69,4
16,19
59,36
1,41
21,67
33,59
7,69
1,68
55,7
3,51
17,39
55,6
43,47
21,23
21,39
54,13
27,13
14,21
19,40
12,35
15,49
37,36
6,39
3,67
64,13
13,55
23,24
39,45
5,68
54,15
18,57
51,0
26,69
53,5
31,53
27,49
5,58
12,41
45,47
10,37
53,23
46,35
49,49
21,69
29,21
51,36
1,60
19,70
65,28
17,70
19,48
37,37
13,65
54,25
39,26
18,67
5,50
67,15
15,27
57,7
41,39
10,59
53,37
51,8
60,23
24,13
45,41
54,5
23,25
24,41
9,56
57,26
68,13
9,45
47,34
27,58
9,40
40,41
67,24
24,49
50,11
17,57
37,44
63,23
57,9
44,21
15,36
23,48
68,21
15,40
63,21
51,34
9,60
64,25
40,43
57,24
61,35
47,14
9,47
52,21
65,26
20,31
23,33
17,69
55,29
15,62
9,48
7,48
35,34
53,32
20,29
19,60
33,32
54,18
57,14
57,36
21,22
6,35
58,49
36,38
38,45
51,20
55,68
44,50
40,37
9,16
45,36
5,42
22,66
18,14
24,26
38,68
20,70
44,30
66,66
34,40
36,46
61,34
64,12
58,44
31,42
39,42
8,12
68,14
30,18
64,56
0,17
50,49
55,42
8,17
34,43
50,45
12,62
67,8
12,60
24,56
21,36
54,4
58,60
58,62
39,50
15,60
69,12
48,53
22,17
40,9
34,34
40,68
44,38
44,62
10,0
6,53
0,28
68,22
68,10
14,14
40,44
66,20
20,68
62,12
8,52
29,18
24,57
30,27
16,67
42,58
16,38
18,45
42,59
24,35
66,32
20,32
38,8
44,67
12,55
39,32
12,28
34,25
52,60
26,57
20,24
36,7
66,6
51,30
64,40
6,17
50,60
36,8
51,50
60,62
54,21
56,25
38,40
20,16
6,36
22,16
7,32
2,21
48,50
28,33
0,69
66,17
36,22
12,54
63,0
61,8
32,36
51,38
29,2
56,30
3,18
55,58
14,30
62,8
28,59
55,38
46,54
68,70
34,55
44,59
52,70
26,24
18,42
68,15
24,64
46,37
33,54
3,36
9,8
4,45
18,33
53,36
66,33
6,29
18,41
47,68
25,22
47,42
53,26
16,16
22,36
8,34
56,63
4,2
48,68
40,59
58,14
42,32
24,47
21,40
16,36
62,60
18,9
32,58
70,20
44,44
16,62
50,44
38,34
16,26
16,37
48,37
44,54
54,20
54,70
40,14
42,52
39,52
6,8
36,52
66,34
58,53
63,12
9,68
56,23
63,34
8,47
66,16
25,14
2,26
28,56
5,34
44,41
58,26
47,56
6,61
34,66
8,22
20,14
20,52
38,35
14,17
46,52
32,22
54,62
16,43
40,50
37,52
54,65
67,56
26,44
64,27
10,64
2,18
37,56
58,47
14,48
32,37
4,67
47,50
46,40
15,30
58,50
65,22
48,63
28,69
60,41
7,58
10,30
68,0
30,60
40,58
67,68
40,40
12,49
10,19
26,51
56,19
18,30
0,46
31,34
28,30
61,30
59,62
34,16
48,30
21,42
40,48
27,4
8,62
6,44
60,56
14,39
58,42
70,18
26,22
56,28
19,14
2,63
27,40
70,32
5,16
30,50
6,50
68,20
50,48
0,22
38,70
29,10
68,16
14,56
3,0
22,60
0,56
27,2
0,50
37,10
42,9
70,43
56,34
2,31
34,32
52,57
44,3
70,22
45,60
22,59
70,36
12,50
20,27
22,6
2,62
4,15
49,22
49,60
22,48
34,36
6,27
52,56
40,33
8,55
44,0
19,4
20,34
64,29
3,60
43,64
4,47
36,62
15,52
58,29
69,18
27,34
25,32
8,2
48,24
28,26
12,57
36,20
32,6
62,32
8,58
2,52
34,35
41,62
27,68
35,52
38,42
41,8
38,10
30,35
60,57
48,58
15,70
19,50
41,66
45,70
34,59
39,68
58,68
19,34
65,56
57,44
4,8
26,36
25,38
54,38
12,6
66,30
20,60
20,38
32,26
22,31
36,21
65,66
0,11
3,44
18,12
10,46
50,26
20,69
25,66
17,2
13,0
10,62
28,46
40,6
38,36
20,57
10,44
36,3
18,52
1,38
44,34
48,0
4,11
34,48
31,62
33,58
21,70
36,32
18,38
11,68
66,68
30,57
30,8
70,33
5,10
33,64
15,32
32,40
22,54
4,0
41,44
60,32
48,32
40,54
32,53
2,69
11,30
22,8
24,2
0,16
25,12
17,42
34,52
42,8
58,56
10,16
54,50
26,37
46,30
10,26
22,29
11,32
4,18
8,28
52,36
62,61
29,16
32,15
38,24
13,34
3,22
4,30
13,26
26,66
11,36
16,57
22,43
58,36
8,14
21,2
55,56
12,45
55,32
50,47
8,6
30,52
30,40
26,48
4,42
59,2
45,50
67,50
22,46
11,60
30,59
2,30
54,14
39,40
35,48
21,26
24,23
38,44
64,36
42,29
39,70
48,20
30,58
18,16
40,15
50,21
6,42
32,8
24,38
14,65
36,27
68,68
12,42
8,11
42,49
16,21
8,44
21,60
3,14
56,11
31,10
22,18
40,64
4,70
19,30
69,10
54,49
8,60
60,34
40,10
14,22
42,69
47,54
63,30
36,23
47,48
27,56
16,15
64,3
56,60
50,58
27,66
16,10
18,54
1,12
56,66
1,42
30,20
58,40
8,39
4,7
37,60
22,32
29,30
52,54
0,58
50,43
40,28
16,40
64,64
14,43
48,13
32,31
11,10
6,60
26,2
43,0
50,22
6,22
12,44
31,20
3,56
8,57
66,18
51,58
30,16
32,23
23,16
14,44
16,28
41,50
20,10
44,58
2,14
43,22
48,69
40,62
70,6
5,54
52,20
24,12
48,42
64,68
42,64
18,23
4,31
18,55
59,58
38,23
44,70
20,66
32,34
2,35
18,56
9,24
63,14
4,19
31,66
4,62
28,5
8,40
5,52
68,8
57,66
62,34
15,26
68,3
0,66
16,24
2,49
18,15
32,13
34,60
6,13
28,6
34,67
48,26
60,40
2,54
24,25
9,50
58,19
12,67
51,64
2,70
54,24
66,52
60,69
6,64
5,12
40,19
49,66
66,45
4,20
58,13
38,32
24,8
47,0
52,45
70,41
13,36
69,54
43,30
70,5
55,24
54,22
38,27
39,2
40,49
60,54
35,66
26,32
21,28
13,6
4,28
32,56
65,24
24,52
42,48
0,64
25,54
70,3
57,8
16,47
56,36
64,37
48,25
30,48
3,70
20,3
54,11
18,29
26,52
37,4
48,52
24,34
14,36
38,2
33,44
44,55
10,20
42,5
68,24
6,26
30,68
49,10
65,58
56,33
16,1
26,42
36,70
35,12
64,39
66,14
0,14
65,32
16,20
14,20
6,58
6,48
59,70
48,35
69,70
40,0
32,68
16,8
11,40
5,0
68,46
4,16
54,7
48,60
58,57
36,15
43,6
62,30
26,20
64,9
19,36
52,26
7,64
56,44
5,38
54,31
16,44
53,4
66,8
68,39
18,36
38,26
13,14
32,62
68,48
44,4
50,32
58,35
14,5
68,45
10,14
7,2
28,50
69,42
14,64
26,18
70,57
20,33
70,62
68,59
20,30
4,10
48,65
39,34
13,2
41,12
44,60
37,6
22,22
60,55
23,0
4,32
40,52
67,4
17,58
65,60
20,63
59,38
62,16
8,8
24,48
20,2
32,48
14,55
14,62
18,68
52,47
20,19
34,46
38,6
29,52
38,9
0,67
55,2
32,2
11,64
24,18
8,7
22,49
22,53
42,31
62,66
60,24
64,16
10,48
62,69
4,46
26,62
46,10
46,31
23,34
8,29
44,46
26,4
10,2
26,21
18,47
25,42
48,59
52,50
8,10
55,34
16,32
36,18
9,2
4,65
50,0
1,52
44,11
64,31
12,26
61,46
63,24
14,63
46,65
50,34
40,29
20,7
40,24
56,54
70,25
10,69
18,44
52,42
56,26
56,40
64,8
63,52
43,52
22,26
24,16
23,14
55,50
63,50
47,44
48,10
2,12
56,59
10,10
70,0
22,30
25,48
46,15
21,68
26,23
31,56
46,44
30,34
28,54
46,24
9,28
28,22
6,45
30,30
38,22
30,51
0,40
34,64
60,22
19,56
28,68
15,34
44,15
12,2
36,26
32,29
10,4
20,36
38,37
58,66
70,2
27,54
32,38
46,32
70,1
6,67
34,17
40,38
12,27
29,62
14,32
38,30
52,59
3,30
65,6
6,30
32,19
32,25
10,58
20,56
61,52
2,6
68,2
8,51
56,3
0,24
45,28
60,64
37,18
30,14
56,2
36,17
38,28
26,54
54,10
22,38
30,5
17,8
10,65
66,15
47,26
17,28
13,16
8,25
44,8
18,21
61,44
58,46
50,50
64,18
47,46
10,24
30,23
36,68
24,32
61,20
18,58
18,24
42,20
26,64
60,9
66,27
24,3
63,64
16,61
62,47
16,12
20,28
48,47
24,63
66,61
34,4
38,47
35,56
70,28
26,53
44,10
68,38
66,22
23,12
66,26
27,8
22,14
34,18
4,57
56,46
54,34
31,8
60,30
20,26
6,18
48,4
52,14
12,16
53,42
16,34
54,52
14,18
38,4
40,30
57,0
68,66
62,15
64,30
4,34
20,17
54,6
38,66
24,33
14,60
2,48
62,28
60,50
15,28
50,1
70,67
49,30
54,1
9,38
24,42
2,16
34,54
58,8
12,22
46,69
1,22
18,28
30,19
26,10
16,14
42,41
17,26
18,11
59,44
43,56
48,29
30,39
4,61
54,58
14,67
36,40
38,57
0,36
28,42
45,0
30,46
47,40
48,34
36,60
36,44
52,15
14,26
50,56
36,45
52,3
0,23
58,48";
