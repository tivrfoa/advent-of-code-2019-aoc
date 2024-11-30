use std::collections::{HashMap, HashSet};


#[derive(Debug, Eq, PartialEq)]
struct Reaction<'a> {
    ing: Vec<(&'a str, usize)>,
    out: &'a str,
    qt: usize,
}

pub fn parse_input(input: &str) -> (HashMap<&str, Reaction>, HashMap<&str, (usize, usize)>) {
    let mut map_reactions: HashMap<&str, Reaction> = HashMap::new();
    // material -> (available, used)
    let mut qt_mat: HashMap<&str, (usize, usize)> = HashMap::new();
    for line in input.lines() {
        let (ing, out) = line.split_once(" => ").unwrap();
        let (out_qt, out_name) = out.split_once(' ').unwrap();
        let mut input_chemicals = vec![];
        for ic in ing.split(", ") {
            let (qt, name) = ic.split_once(' ').unwrap();
            input_chemicals.push((name, qt.parse::<usize>().unwrap()));
        }
        let reaction = Reaction {
            out: out_name,
            qt: out_qt.parse::<usize>().unwrap(),
            ing: input_chemicals,
        };
        if let Some(_old) = map_reactions.insert(out_name, reaction) {
            todo!();
        }
        qt_mat.insert(out_name, (0, 0));
    }

    (map_reactions, qt_mat)
}

pub fn p1(input: &str) -> usize {
    let (map_reactions, mut qt_mat) = parse_input(input);
    qt_mat.insert("ORE", (0, 0));
    solve1("FUEL", 1, &map_reactions, &mut qt_mat);
    qt_mat["ORE"].1
}

fn solve1(goal: &str, goal_qt: usize, map_reactions: &HashMap<&str, Reaction>,
        qt_mat: &mut HashMap<&str, (usize, usize)>) {
    if goal == "ORE" {
        if let Some(v) = qt_mat.get_mut("ORE") {
            *v = (v.0 + goal_qt, v.1);
        }
        return;
    }
    let reaction = &map_reactions[goal];
    let t = if reaction.qt >= goal_qt {
        1
    } else {
        goal_qt / reaction.qt + if goal_qt % reaction.qt > 0 { 1 } else { 0 }
    };
    for ing in reaction.ing.iter() {
        let need = t * ing.1;
        let have = qt_mat[ing.0].0;
        
        if need > have {
            if have > 0 {
                if let Some(v) = qt_mat.get_mut(ing.0) {
                    *v = (v.0 - have, v.1 + have);
                }
            }
            let diff = need - have;
            solve1(ing.0, diff, map_reactions, qt_mat);
            if let Some(v) = qt_mat.get_mut(ing.0) {
                *v = (v.0 - diff, v.1 + diff);
            }
        } else {
            if let Some(v) = qt_mat.get_mut(ing.0) {
                *v = (v.0 - need, v.1 + need);
            }
        }
    }

    if let Some(v) = qt_mat.get_mut(goal) {
        *v = (v.0 + t * reaction.qt, v.1);
    }
}

pub fn p2(input: &str) -> usize {
    let (map_reactions, mut qt_mat) = parse_input(input);
    let num_ore: usize = 1_000_000_000000;
    qt_mat.insert("ORE", (num_ore, 0));
    let mut lo = num_ore / 365768;
    let mut hi = num_ore;
    let mut max = 0;

    while lo <= hi {
        let md = lo + (hi - lo) / 2;
        if solve2("FUEL", md, &map_reactions, &mut qt_mat.clone()) {
            max = md;
            lo = md + 1;
        } else {
            hi = md - 1;
        }
    }

    max
}

fn solve2(goal: &str, goal_qt: usize, map_reactions: &HashMap<&str, Reaction>,
        qt_mat: &mut HashMap<&str, (usize, usize)>) -> bool {
    if goal == "ORE" {
        return false;
    }
    let reaction = &map_reactions[goal];
    let t = if reaction.qt >= goal_qt {
        1
    } else {
        goal_qt / reaction.qt + if goal_qt % reaction.qt > 0 { 1 } else { 0 }
    };
    for ing in reaction.ing.iter() {
        let need = t * ing.1;
        let have = qt_mat[ing.0].0;
        
        if need > have {
            if have > 0 {
                if let Some(v) = qt_mat.get_mut(ing.0) {
                    *v = (v.0 - have, v.1 + have);
                }
            }
            let diff = need - have;
            if !solve2(ing.0, diff, map_reactions, qt_mat) {
                return false;
            }
            if let Some(v) = qt_mat.get_mut(ing.0) {
                dbg!(goal, ing, &v, diff);
                *v = (v.0 - diff, v.1 + diff);
            }
        } else {
            if let Some(v) = qt_mat.get_mut(ing.0) {
                *v = (v.0 - need, v.1 + need);
            }
        }
    }

    if let Some(v) = qt_mat.get_mut(goal) {
        *v = (v.0 + t * reaction.qt, v.1);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_sample10() {
        assert_eq!(165, p1(SAMPLE1));
    }

    #[test]
    fn test_p1() {
       assert_eq!(365768, p1(IN));
    }

    #[test]
    fn p2_sample() {
        assert_eq!(82892753, p2(SAMPLE_13312));
    }

    #[test]
    fn test_p2() {
       assert_eq!(3756877, p2(IN));
    }
}





// -------------------------- INPUT



pub static SAMPLE1: &str = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";


pub static SAMPLE_13312: &str = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";


pub static IN: &str = "1 QDKHC => 9 RFSZD
15 FHRN, 17 ZFSLM, 2 TQFKQ => 3 JCHF
4 KDPV => 4 TQFKQ
1 FSTRZ, 5 QNXWF, 2 RZSD => 3 FNJM
15 VQPC, 1 TXCJ => 3 WQTL
1 PQCQN, 6 HKXPJ, 16 ZFSLM, 6 SJBPT, 1 TKZNJ, 13 JBDF, 1 RZSD => 6 VPCP
1 LJGZP => 7 VNGD
1 CTVB, 1 HVGW => 1 LJGZP
6 HVGW, 1 HJWT => 2 VDKF
10 PQCQN, 7 WRQLB, 1 XMCH => 3 CDMX
14 VNGD, 23 ZFSLM, 2 FHRN => 4 SJBPT
1 FSTRZ, 4 VTWB, 2 BLJC => 4 CKFW
2 ZTFH, 19 CKFW, 2 FHRN, 4 FNJM, 9 NWTVF, 11 JBDF, 1 VDKF, 2 DMRCN => 4 HMLTV
1 KVZXR => 5 FPMSL
8 XBZJ => 8 QDKHC
1 VQPC => 9 FHRN
15 RKTFX, 5 HKXPJ => 4 ZFSLM
1 HKXPJ, 8 LQCTQ, 21 VJGKN => 5 QCKFR
1 DCLQ, 1 TQFKQ => 4 KVZXR
4 NWTVF, 20 QNXWF => 9 JFLQD
11 QFVR => 3 RZSD
9 RFSZD, 6 WQTL => 7 JBDF
4 BLJC, 3 LQCTQ, 1 QCKFR => 8 QFVR
6 VNGD => 5 VQPC
7 CTMR, 10 SJBPT => 9 VTWB
1 VTWB => 9 DMRCN
6 BCGLR, 4 TPTN, 29 VNGD, 25 KDQC, 40 JCHF, 5 HMLTV, 4 CHWS, 2 CDMX, 1 VPCP => 1 FUEL
1 TQFKQ, 3 FPMSL, 7 KDPV => 6 RKTFX
8 HKXPJ, 2 WQTL => 6 WRQLB
146 ORE => 3 KDPV
9 KDQC => 2 XMCH
1 BGVXG, 21 KVZXR, 1 LQCTQ => 4 CTVB
1 LQCTQ => 5 VJGKN
16 VNGD, 5 VMBM => 1 CTMR
5 VCVTM, 1 FPMSL => 5 HKXPJ
4 HKXPJ => 5 BLJC
14 FHRN, 6 ZFSLM => 1 NWTVF
7 QCKFR, 2 VNGD => 7 VMBM
4 TXCJ, 1 VDKF => 2 QNXWF
136 ORE => 6 BGVXG
5 LQCTQ, 11 DCLQ => 9 XBZJ
3 VQPC => 7 ZTFH
114 ORE => 3 ZWFZX
1 HJWT, 18 KDPV => 7 TXCJ
1 VJGKN => 2 VCVTM
2 KVZXR => 1 HJWT
12 ZWFZX, 1 FHRN, 9 JFLQD => 1 CHWS
3 QFVR => 5 FSTRZ
5 XBZJ => 4 HVGW
1 ZWFZX => 8 LQCTQ
16 WQTL, 10 TXCJ => 9 KDQC
3 FHRN, 12 LJGZP => 5 TPTN
1 JCHF => 7 PQCQN
7 KDPV, 17 BGVXG => 7 DCLQ
1 CKFW, 3 TKZNJ, 4 PQCQN, 1 VQPC, 32 QFVR, 1 FNJM, 13 FSTRZ => 3 BCGLR
2 FSTRZ => 5 TKZNJ";
