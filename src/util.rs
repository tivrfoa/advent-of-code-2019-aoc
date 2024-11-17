use std::collections::HashMap;
use std::collections::HashSet;

pub fn set(s: &str) -> HashSet<char> {
    let mut set = HashSet::new();
    s.chars().for_each(|c| {
        set.insert(c);
    });
    set
}

pub fn sorted(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}

pub fn freq(s: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    s.chars().for_each(|c| {
        map.entry(c).and_modify(|qt| *qt += 1).or_insert(1);
    });
    map
}

pub fn left_pad<T: ToString>(data: T, len: usize, c: char) -> String {
    let mut ret = String::with_capacity(len);
    let mut s = data.to_string();

    for _ in 0..len - s.len() {
        ret.push(c);
    }

    ret.push_str(&mut s);

    ret
}