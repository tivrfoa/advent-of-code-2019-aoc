use std::collections::{HashMap, HashSet};

pub fn p1(input: &str) -> usize {
    let map = process_input(input);


    0
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



pub static IN: &str = "";
