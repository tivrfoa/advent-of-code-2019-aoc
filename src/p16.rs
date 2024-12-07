use std::collections::*;

// base pattern: 0, 1, 0, -1

const PHASES: usize = 1;
const P: [i32; 4] = [0, 1, 0, -1];

// fn get_pattern(pi: usize, p_times: usize) -> usize {
//     for i in 0..4 {
//         for j in 0..p_times {
//             return i;
//         }
//     }
// }

pub fn p1(input: &str) -> usize {
    let mut nums: Vec<i32> = input.chars().map(|c| (c as u8 - b'0') as i32).collect();
    let len = nums.len();



    for _ in 0..PHASES {
        let mut new = vec![];

        for i in 0..len {
            let p_times = i + 1;
            let qt = p_times * 4;
            let mut pi = 0;
            let mut sum = 0;

            for j in 0..len {
                pi += 1;
                let m = pi % qt;
                // let idx_p = m / p_times + m % p_times;
                let idx_p = m / p_times;
                sum += nums[j] * P[idx_p];
            }
            dbg!(sum);
            new.push((sum % 10).abs());
        }
        dbg!(&new);

        nums = new;
    }

    0
}

pub fn p2(input: &str) -> usize {


    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sample() {
        assert_eq!(171, p1(SAMPLE));
    }

    #[test]
    #[ignore]
    fn test_p1() {
        assert_eq!(171, p1(IN));
    }

    #[test]
    #[ignore]
    fn test_p2_sample() {
        assert_eq!(171, p2(SAMPLE));
    }

    #[test]
    #[ignore]
    fn test_p2() {
        assert_eq!(171, p2(IN));
    }
}





// -------------------------- INPUT



pub static SAMPLE: &str = "12345678";

pub static IN: &str = "59773590431003134109950482159532121838468306525505797662142691007448458436452137403459145576019785048254045936039878799638020917071079423147956648674703093863380284510436919245876322537671069460175238260758289779677758607156502756182541996384745654215348868695112673842866530637231316836104267038919188053623233285108493296024499405360652846822183647135517387211423427763892624558122564570237850906637522848547869679849388371816829143878671984148501319022974535527907573180852415741458991594556636064737179148159474282696777168978591036582175134257547127308402793359981996717609700381320355038224906967574434985293948149977643171410237960413164669930";
