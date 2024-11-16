use std::collections::HashMap;
use std::collections::HashSet;

fn set(s: &str) -> HashSet<char> {
    let mut set = HashSet::new();
    s.chars().for_each(|c| { set.insert(c); });
    set
}

fn sorted(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}

fn p1(a: i32, b: i32) -> i32 {
    let mut ans = 0;

    for n in a..=b {
        let s = n.to_string();
        if set(&s).len() < s.len() && sorted(&s) == s {
            ans += 1;
        }
    }

    ans
}


fn main() {
    assert_eq!(1764, p1(152085, 670283));
    // assert_eq!(0, p2(IN));
}


