use std::collections::*;

pub fn p1(input: &str) -> usize {


    0
}

fn input_to_char_grid(input: &str) -> Vec<Vec<char>> {
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
        assert_eq!(171, p1(IN));
    }

    #[test]
    #[ignore]
    fn test_p2() {
        assert_eq!(171, p2(IN));
    }
}





// -------------------------- INPUT



pub static SAMPLE: &str = "";

pub static IN: &str = "";
