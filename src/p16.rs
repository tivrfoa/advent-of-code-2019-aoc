use std::collections::*;

// base pattern: 0, 1, 0, -1
const P: [i32; 4] = [0, 1, 0, -1];

// fn get_pattern(pi: usize, p_times: usize) -> usize {
//     for i in 0..4 {
//         for j in 0..p_times {
//             return i;
//         }
//     }
// }

pub fn p1(phases: usize, input: &str) -> String {
    let mut nums: Vec<i32> = input.chars().map(|c| (c as u8 - b'0') as i32).collect();
    let len = nums.len();

    for _ in 0..phases {
        let mut new = Vec::with_capacity(len);

        for i in 0..len {
            let p_times = i + 1;
            let qt = p_times * 4;
            let mut pi = 0;
            let mut sum = 0;

            for j in 0..len {
                pi += 1;
                let m = pi % qt;
                let idx_p = m / p_times;
                sum += nums[j] * P[idx_p];
            }
            let single_abs_digit = (sum % 10).abs();
            new.push(single_abs_digit);
        }

        nums = new;
    }

    let mut s = String::new();
    for i in 0..8 {
        s.push(('0' as u8 + nums[i] as u8) as char);
    }

    return s;
}


pub fn p2(phases: usize, input: &str) -> String {
    let input = input.repeat(10_000);
    let offset = (&input[..7]).parse::<usize>().unwrap();
    let mut nums: Vec<i32> = input.chars().map(|c| (c as u8 - b'0') as i32).collect();
    let len = nums.len();

    for _ in 0..phases {
        let mut new = Vec::with_capacity(len);

        for i in 0..len {
            let p_times = i + 1;
            let qt = p_times * 4;
            let mut pi = 0;
            let mut sum = 0;

            for j in 0..len {
                pi += 1;
                let m = pi % qt;
                let idx_p = m / p_times;
                sum += nums[j] * P[idx_p];
            }
            let single_abs_digit = (sum % 10).abs();
            new.push(single_abs_digit);
        }

        nums = new;
    }

    let mut s = String::new();
    for i in 0..8 {
        s.push(('0' as u8 + nums[offset + i] as u8) as char);
    }

    return s;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sample() {
        assert_eq!("01029498".to_string(), p1(4, SAMPLE));
    }

    #[test]
    fn test_p1_sample2() {
        assert_eq!("24176176".to_string(), p1(100, "80871224585914546619083218645595"));
    }

    #[test]
    fn test_p1() {
        assert_eq!("84970726".to_string(), p1(100, IN));
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!("84462026".to_string(), p2(100, "03036732577212944063491565474664"));
    }

    #[test]
    #[ignore]
    fn test_p2() {
        assert_eq!("171".to_string(), p2(100, IN));
    }
}





// -------------------------- INPUT



pub static SAMPLE: &str = "12345678";

pub static IN: &str = "59773590431003134109950482159532121838468306525505797662142691007448458436452137403459145576019785048254045936039878799638020917071079423147956648674703093863380284510436919245876322537671069460175238260758289779677758607156502756182541996384745654215348868695112673842866530637231316836104267038919188053623233285108493296024499405360652846822183647135517387211423427763892624558122564570237850906637522848547869679849388371816829143878671984148501319022974535527907573180852415741458991594556636064737179148159474282696777168978591036582175134257547127308402793359981996717609700381320355038224906967574434985293948149977643171410237960413164669930";
