use std::collections::*;
use crate::util::*;

#[allow(dead_code)]
fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (tp, dd) = input.split_once("\n\n").unwrap();
    (tp.split(", ").collect(), dd.lines().collect())
}

fn solve_no_memo<'a>(s: &'a str, max_tp: usize, tp: &HashSet<&str>) -> bool {
    for i in 1..=max_tp.min(s.len()) {
        if !tp.contains(&s[0..i]) {
            continue;
        }
        if tp.contains(&s[i..]) || solve_no_memo(&s[i..], max_tp, tp) {
            return true;
        }
    }
    false
}

pub fn p1_no_memo(input: &str) -> usize {
    let (tp, dd) = parse(input);
    let max_tp = tp.iter().map(|s| s.len()).max().unwrap();
    let tp: HashSet<&str> = HashSet::from_iter(tp);
    dd.into_iter()
        .filter(|s| solve_no_memo(s, max_tp, &tp))
        .count()
}

fn solve<'a>(s: &'a str, max_tp: usize, tp: &HashSet<&str>, memo: &mut HashMap<&'a str, bool>) -> bool {
    if let Some(v) = memo.get(s) {
        return *v;
    }
    for i in 1..=max_tp.min(s.len()) {
        if !tp.contains(&s[0..i]) {
            continue;
        }
        if tp.contains(&s[i..]) || solve(&s[i..], max_tp, tp, memo) {
            memo.insert(s, true);
            return true;
        }
    }
    memo.insert(s, false);
    false
}

pub fn p1(input: &str) -> usize {
    let (tp, dd) = parse(input);
    let max_tp = tp.iter().map(|s| s.len()).max().unwrap();
    let tp: HashSet<&str> = HashSet::from_iter(tp);
    let mut memo: HashMap<&str, bool> = HashMap::new();
    dd.into_iter()
        .filter(|s| solve(s, max_tp, &tp, &mut memo))
        .count()
}

fn solve2<'a>(s: &'a str, max_tp: usize, tp: &HashSet<&str>, memo: &mut HashMap<&'a str, usize>) -> usize {
    if s.is_empty() { return 1; }
    if let Some(v) = memo.get(s) { return *v; }
    let mut qt = 0;
    for i in 0..=max_tp.min(s.len()) {
        if !tp.contains(&s[0..i]) {
            continue;
        }
        qt += solve2(&s[i..], max_tp, tp, memo);
    }
    memo.insert(s, qt);
    qt
}

pub fn p2(input: &str) -> usize {
    let (tp, dd) = parse(input);
    let max_tp = tp.iter().map(|s| s.len()).max().unwrap();
    let tp: HashSet<&str> = HashSet::from_iter(tp);
    dd.into_iter()
        .map(|s| solve2(s, max_tp, &tp, &mut HashMap::new()))
        .sum()
}

pub fn p2_same_memo(input: &str) -> usize {
    let (tp, dd) = parse(input);
    let max_tp = tp.iter().map(|s| s.len()).max().unwrap();
    let tp: HashSet<&str> = HashSet::from_iter(tp);
    let mut memo: HashMap<&str, usize> = HashMap::new();
    dd.into_iter()
        .map(|s| solve2(s, max_tp, &tp, &mut memo))
        .sum()
}

/// Based on Elizarov:
/// https://github.com/elizarov/AdventOfCode2024/blob/main/src/Day19_2.kt
pub fn p2_dp_rev(input: &str) -> usize {
    let (tp, dd) = parse(input);
    let count = |s: &str| -> usize {
        let n = s.len();
        let mut dp = vec![0; n + 1];
        dp[n] = 1;
        for i in (0..n).rev() {
            dp[i] = tp.iter().map(|p| if s[i..].starts_with(p) { dp[i + p.len()] } else { 0 }).sum();
        }
        dp[0]
    };
    dd.into_iter().map(|s| count(s)).sum()
}

/// Errichto https://www.youtube.com/watch?v=Mu0XXZeCFqw
pub fn p1_dp(input: &str) -> usize {
    let (tp, dd) = parse(input);
    let test = |s: &str| -> bool {
        let n = s.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true;
        for i in 0..n {
            if dp[i] {
                for p in &tp {
                    let k = p.len();
                    if i + k <= n && !dp[i + k] {
                        dp[i + k] = s[i..].starts_with(p);
                    }
                }
            }
        }
        dp[n]
    };
    dd.into_iter().filter(|s| test(s)).count()
}

pub fn p2_dp(input: &str) -> usize {
    let (tp, dd) = parse(input);
    let count = |s: &str| -> usize {
        let n = s.len();
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        for i in 0..n {
            if dp[i] > 0 {
                for p in &tp {
                    let k = p.len();
                    if s[i..].starts_with(p) {
                        dp[i + k] +=  dp[i];
                    }
                }
            }
        }
        dp[n]
    };
    dd.into_iter().map(|s| count(s)).sum()
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sample() {
        assert_eq!(6, p1(SAMPLE));
    }

    #[test]
    fn test_p1_no_memo() {
        assert_eq!(213, p1_no_memo(IN));
    }

    #[test]
    #[ignore] // timeout
    fn test_p1_debug1_no_memo() {
        assert_eq!(0, p1_no_memo(DEBUG1));
    }

    #[test]
    #[ignore] // timeout
    fn test_p1_debug2_no_memo() {
        assert_eq!(0, p1_no_memo(DEBUG2));
    }

    #[test]
    fn test_p1_in() {
        assert_eq!(213, p1(IN));
    }

    #[test]
    fn test_p1_debug1() {
        assert_eq!(0, p1(DEBUG1));
    }

    #[test]
    fn test_p1_debug2() {
        assert_eq!(0, p1(DEBUG2));
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(16, p2(SAMPLE));
    }

    #[test]
    fn test_p1_dp() {
        assert_eq!(213, p1_dp(IN));
    }

    #[test]
    fn test_p2_dp() {
        assert_eq!(1016700771200474, p2_dp(IN));
    }

    #[test]
    fn test_p2_sample_dp() {
        assert_eq!(16, p2_dp_rev(SAMPLE));
    }

    #[test]
    fn test_p2_dp_rev() {
        assert_eq!(1016700771200474, p2_dp_rev(IN));
    }

    #[test]
    fn test_p2_same_memo() {
        assert_eq!(1016700771200474, p2_same_memo(IN));
    }

    #[test]
    fn test_p2_in() {
        assert_eq!(1016700771200474, p2(IN));
    }
}


// -------------------------- INPUT

pub static SAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

pub static DEBUG1: &str = "bwwww, gwwug, ruw, wrbuwrbw, ugugb, rwwgw, wwb, ubgg, rubbwu, wgbuurbb, wrrbru, gwg, uuwwg, ugrrgr, uugrb, grwb, gwrb, buwrwuw, wrb, ruuggwr, rub, guwwr, gwrur, uug, ugguwru, uwr, rwuug, bwgw, ruu, wrbb, wub, wuwgugg, grgwrww, bgubw, wruuuu, bwg, wu, wbrbgr, bgwg, bgrubu, ggru, bgguww, rubgugrw, grwguu, wbg, ugub, urbwuurr, grb, ug, gbu, gbgwurbw, rrrburr, uggru, rubuwrrw, guwgrb, rbr, uruw, gggur, ugwg, bbwrr, ggbbbru, grbug, uuubww, rruug, ubbgwgr, brb, rbbuwbu, ggbrru, uuwugb, ubbgr, bubg, grbb, rgb, rwur, ubuuu, gwuwb, bubgrgr, wuuu, bwwg, urugg, wubuwubw, wgrb, gwggrgu, rwbu, bbbubgrg, rw, ggug, gwbgwbrb, bgugww, ggurrwg, rbbwuwu, wgw, bwb, bbw, urru, ugg, uwguu, wggw, wgr, urrr, urguwr, bbb, bgwbgub, guw, gu, gurwr, rgw, wbb, wggu, brbbg, wbw, rur, gub, rrbbr, uugruww, rbbrb, bwgrgr, rwrurww, grurg, brur, wbr, rgu, gguwr, bwurb, grguw, wuu, rwu, ubu, wbbwwub, urg, rww, bgg, wrwg, ugr, uru, urwggw, buwur, bbrgbwgr, bggbbgw, bbgu, rurrr, ugww, uurbur, brbw, wgwrbr, wbuuw, gwgu, rbw, gbwugwb, gbw, bwwgwr, wurrgwu, wwbg, rburuu, wrruwg, gg, bbbwugg, g, bg, wwguuu, gruwgu, ggrr, ur, bwwru, rwgub, rgwuu, wur, wuwrugw, uw, bubggbr, ubug, gwbbrw, rubgwg, rwbuub, rgrbugb, rbgb, wwgb, brbgw, uwwr, rbrg, uwrrur, uuwb, wug, bwwgrgb, rg, wuwww, bgrb, uurgw, rrburg, wubg, rwrwur, rwb, wrw, bwrw, wg, wwgrubwg, bugb, uuubguuw, gruu, wggruuu, urwu, wuugb, rrwww, bwrugbrb, wrgg, bru, ggww, wrbrrg, ugbu, rwwrrguu, wrg, buguu, ugbbww, urwur, grr, uuw, rrub, rb, bww, wgwr, ubb, bbbuub, ggbrw, rbwb, ubw, uuwg, wgrrbr, grw, gwu, wuwwgr, rwuw, gbbr, bguur, gwgrrw, bbu, ggg, grg, urbwwrr, bwgbuw, bw, gbrw, wbwrb, rurg, uurg, guugb, uww, brrbggbu, ruww, ru, rwguw, wgwbug, wwu, wgwwbur, bu, rguu, uwwwg, rug, wwg, brg, uwu, ubr, burg, ruug, rwr, uurugu, gr, uwubu, rr, rbwrbw, gw, uwwuu, ruuwb, rbg, wwrw, wruubg, bgwgrwbr, wuru, ggrgr, br, rwuu, grgbgg, urw, uub, rgrbb, wbuwb, wgwbg, ubwu, gwb, uu, rrrr, rru, gwr, uuubu, grgw, gbb, gwug, wbggwu, gggr, wr, wuw, wgg, buw, gbuwrwu, gb, uwb, bgubgb, rggbbbu, rgubgggr, rbgr, ruwr, wwr, grgbrb, ugrubr, ubgu, ugw, ugrr, ubg, ubgb, urb, wgb, wbwrbw, rugrr, ub, gbr, bugrg, ugu, rgrw, rrrrurr, bgbw, wgwg, guu, wrrubu, www, wrbu, bgrguwr, wwur, ubgbgug, wbwu, buu, gbgubrw, b, ggr, rrg, wwwb, wgrrrw, ubgrug, rrr, ugbb, wugubbur, bgwb, buwww, ugb, buub, wbrru, wuwguur, gwrggg, bgb, gru, bwwr, bgr, gbbw, wuruw, gbrgu, uwg, ubrg, ggu, bbwg, ugrbug, bbr, r, ugbwub, ggb, bbg, bggwb, bub, gbg, burbr, brw, wbuu, gugbb, uwubg, gbgwr, bgwwru, rgugb, uubgr, bb, gug, rggb, wugub, wrgu, wrwguug, gur, wru, bgbr, bug, bwu, ubbur, uurugb, ubgug, bgrr, bgbb, uwbrrug, bburur, rgr, brbbrw, rrrrwu, rgg, urwuwr, wwgr, bur, rugwg, bgu, urbg, wwrgbuw, rrgwrbb, uuu, grwgg, bwr, wrbrb, wrr, uwuuur, rrubu, ugwrg, rwuwg, wgu, rwru, rrww, urr, rwg, bgw, rbb, bgbubrg, bwbb, u, wggurggu, bubgw, ggw, gugg, ugrru, wbu, rggrrb, ubrgggwg, bwuu, rurb, wgrrg, brr, rubr

urbrbuwgbgwwwbgbwrugruburguwgwbgrugwrrbwbubgrwgww";

pub static DEBUG2: &str = "a, aa

aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaax";

pub static IN: &str = "brgru, burbw, wrwg, rbbbg, bwrb, rurbw, gbbgbu, bubggb, ggrb, gburb, wguwgbw, grwwrgbb, rwu, ggurgwb, wbb, rru, rwgb, bbrr, brgw, rrrrbu, bbuugrbb, www, rggg, bwbuw, wwuw, rwgrb, wruw, wwug, wu, wbg, grrrwbu, rbr, wru, bbgubur, wbbu, ugb, urrgw, wbgwg, bwrgwrg, gugbb, rruuwb, gbwg, u, uwbr, wbbww, rugu, guggu, burbbu, wub, gwbw, gbrw, brur, wr, rbrrugw, uuur, bwb, rurw, rruguww, gwguur, gur, ggwbug, uubru, wrbbub, wggrwr, brwu, uugu, wugbwgu, burug, gu, rgbggb, ggg, gwu, uwbg, rrbrw, rg, guwrw, rw, bwbwguw, gubbur, gbur, wbu, bwub, rrggb, gbwbb, rguur, gbgu, wb, rub, urwb, bb, wgw, rwr, gbu, wwbug, uuwwgwg, guug, bwrwuw, uwg, rugg, wbw, rbgg, rwrwwb, rurgu, uug, rbwb, rbbwwg, gwruru, bwugggw, rwbr, wbbr, rrr, rrwg, ubu, uww, wuurr, gwggu, bbg, rrggbr, ubrruwb, ugg, bbw, gwwgbw, ubrbgr, wg, wrg, wur, r, wwrbw, bwgu, uuwrw, wug, gwrw, rww, gwuu, rgg, rrb, ruwwwgu, rurgb, ww, rrw, rrwug, wuwrww, wuruwg, gwwb, grbg, brrub, urbbbwr, wwwwuwu, bbu, ub, wwgubu, rbw, wuugg, rr, wbub, ggr, buugu, rgw, rwgr, wgwuu, bgb, wwurrgu, uuwgw, uwuw, guggwgug, bggbbb, ruuwur, rgrbgru, wwuggwbw, brrwgur, bbrruwwg, ubw, wrw, ruwwg, ugbwwu, wrbu, grg, rgr, rbwwbuw, br, ugrbb, bwrwrb, gguu, buurg, wgwr, gbr, gub, rrrgggrr, bru, ubwwb, rurr, bggb, brgbbuug, bggbuuru, wwur, rbb, grgu, wrru, wubu, wgu, w, wwgb, brgub, rwbbr, ggb, ruwbr, wuw, gbg, bwbuwu, bbuu, guuu, bgrw, wrgwr, ubgrwrug, bgu, rrwu, rwburb, bggr, rur, gbb, rwwwwr, bug, urb, rug, bgg, ubruw, rgrw, gr, wwu, wrbbg, ubb, grrr, buw, rububu, gwbru, bbbgwurw, uggggr, bbbuug, grwb, ugbwuu, ruu, rugwb, gbuub, wrubw, ggbr, ggw, ugww, gwg, ugr, wwr, ubugb, wuu, wggg, buuur, rrrwrgb, rwbg, rrg, bwwrrw, wbrrugbg, wwgwrruu, guw, wruggw, uru, ruw, bgbrr, grb, brr, rrbbgbw, uwu, ubbgwr, bwwwg, brgr, urgrbr, ubuurrr, ruwrgbb, bwr, rgu, wugwwu, guu, rrwrbu, wuwg, uwugggg, rbbugrww, rbu, grbu, brgb, uuwgg, ggwb, uuwbrrw, brwg, uwb, rgwgg, gugu, rrurbb, rgbgr, grugg, grub, rwubgw, wrrg, gurbu, ggu, uwwb, uub, gwwurr, urur, gubrw, urgrrrg, gwrbw, wgwwu, urw, bwbb, wggbrrwu, grrgrbw, wbuwrg, wugu, wrwwgr, wgr, bgrug, brb, rb, uwwg, bg, ugrwgu, brg, bruu, urr, gg, uuu, uuw, ggugu, bbbgg, urbrrb, grgrb, rurg, guuub, gww, grr, burw, uwuu, wrb, wbwrr, bwgrug, grw, wgwb, gwb, gwurbrb, rwbur, uwgw, bwruubr, uwggb, gru, rrurru, rwbgb, brw, bur, rbgu, ugw, wwg, ubgwg, g, ubr, wbr, bbr, wgrr, wgb, wbwb, bggguub, ggwubrr, rbg, uw, bgrrggu, wgwu, bubbu, bub, bu, uwwgr, bgbu, ubg, ru, bwrbgug, wwb, wwwgb, brgwbwgg, rrwwwr, gubbb, wrwrg, wgg, rwg, bwrrw, uur, rbbwbr, gurwub, ruuub, uwwr, wgrgww, wrgbrrwg, uwbwugg, wbbrrrr, rgwrb, rwgw, brww, guugbugw, ggguwbr, ur, wbbw, wwgbw, bwu, urg, bgur, gb, ug, rrgggwu, ubgu, wubb, urubuguu, bww, rwb, grwguw, ruwg, uu, bbgwu, ruub, uggbug, wugrgg, ugbuub, ruuuwg, gwr, uggruwwr, wrr, gug, ugu, gbugru, bbwbwwu, wwwu, wgbubuu, rwugww, urgubrwg, rgbwuw, wwbrb, buu, uurbbr, bgr, rwrug, bguw, wbwubbu, bgw, wguu

bwgrururrgrgbugwgggbbuwwbuwgbbuwwbwrugbuw
bwgwbrbrwrwuuwugrrgbwubrrwbbrrrgrrrwruruugubrggwbbwguuu
rwgbwbggrbgbwggwguwwuuwwrbwurwwggbwwwbubwbwgwur
bwgbuubgbbgwguuuwuwwuugbrgurwbuwbwgbuwwwwbbugrruurwurgrgbr
bwgwubwwwubrubuwrrwruruubrrbbruwrbgwbgubbgwrruwbbrubrbubrw
bwgwuuwbubuwgurwggbbrwbwwwgbrrwwbgwgrbbgbww
wwgugruggbbbuurwugbrurruwbbbbugbgurugrwugwuguggugrgwuwru
bwgwrgwuwrgurgwbgubwuuwugbwrggburbbgwruurwguwwuugbrb
bgwubbrgrwwbggwgubbwruwuwrbgwrbbrbwrwwggbu
urrwrrgwwbwrbburwguwgurrbuuwbbwwugrwwrbwwugbgubuggw
wgwwwwwgurrwgrwwruruwbwbgwgwwwbuwwuurugwrg
bgrwgrbubwbbwurbbbrrwbubwrwrgrggwubbgrrrgggrrgwww
bwgbwrgbrbwuwuwwbrrrburgbgbwbrwrbwwrwwuwrgwbbbgrbggubrrw
ggrbwbgurrbuwbgbuurrgbwwgwwggwgrwwbwubbuggrbugw
bbrwgwwggbgrgwrrbbrggrggwwrgwwgburrwrwgbrbbbrgrgrwbu
bwgwwgrrrwgwrurbwrggbwrbuburbbbrgggwwrbrrrgbwbuururwguwrwrbw
rwbgwuurrugbwuruwwgwbrgggbubuubgwwbwgubrbubgwrurwbrggrgww
uubrrwrggrrbgrwuwuurrrrwbwubwbwrugbrggbubrgbwrbruuubrwrr
rwrurggrbuuggruwwruwgrwuuurrbgrbugrubbgbwbwggurbrrrggrw
bwgbubgwgwgrbugwbrrubuugrrwurrbwwbwwuwbbgwwb
wubwuruuwwgubbgbggbuwrbrwwrbrbwruurbwbwubrrugb
guurrwurrugubbrwbgrbgrwbggbuubbugwrurbruuggb
bwgwwurggbgrwuruururwuwbruggrrburuuuubggg
wbbgubuguggurgbubrgwuwrgrrgbbruugggugwrrurwgwgrwwbwbgubrb
bbgwrwbbburwrgrwggbrrwuubbbrburwwguwgbgrwbrurrbbbbgrgugu
bwgbwwuwgrwwrwwwwggwwwgurgwbbbbwwrrrgbuubuubuwgrgubwwb
ruugurgrwbbbuuwbubbgubwbgwbbgrwgwrrgwrbwbbr
rubrwrbgrwbwwggwbgbuwburgwbrbuwugrubbgwgugwuuurbwwggwbrw
bwgwwrgbbugwrguwuggguubrbuwgbwuwbbrububrrrburburu
bbbwgrrwwwgburrburugurrubwrrgrrgbrgwggrbuugburrurwug
brrgrbwwugwrbwbwwwrrbrbwbgwbbggbuuruwrbguugrbrrw
buuwwubrrwbwrbgggrrwubgguwgrbwwguruguuwwrgwur
brbrrwrburbbgurrguurbrwrwbgrgrgrguwwubugbuwwrggggbuwwrwb
bwgwwugurggrwwuwgwububgwrbwrbuwwrrrrwwwuuuuuwugurwwru
bwggrgwguugubuuurrgrurbwuwbuwbrbrgwgrwuuggrgbwgugugggbrgb
bwgwurgbwgrwurugrggbrrurbbwwbbuubuguuwububgrwrugwgbr
bwgwrbrwwurrrgrgubbrubrrugrubburubuguugwbb
guwwbbruwwurrruugurggggbbuugrbburbbubwwbrwwwuguuuubwg
rrwgugwbgwugwggbbwwbguwgrrrguubrwurwbbbwguwbwgrw
rbgubbwwguuuguugwwugrrurwrrwwwbbgwrrrbrguwbbwburwr
bwgbwbgbubruurgruuuurwbuwbruwwgrugwuwbrrgurbwuw
rbruguubrrrwguwwbwrwgwrruwrwwbgrruubrrwgbuwrrgbggugrbrug
bwgggbbgbbrwruurgggbubbrwbrbgrgwrwburrbgugwb
urbrrwwubwuugwgurguwgwrbrgrrwwbbuwurbbrrruwr
bwgwgrbrwbbgugubbbgrggburbbubgwwrgwruwwgwrbgubwbguugrwwr
uwrwrbgrwrgburururbrrbgwbbgguwugugubgwbbrbuuguuwrrwgw
ruuwgwwwbwbgbggbwbbgbwurgrgggbrruggggrbbwbrwgug
bwgrrbbuubwugbbgbbgbguburbwbbuggbgwwurgrrwr
bwgrgugrbruwgrurgggwwbwubrrgubgguugrgburbguwrwbuwurwgbuurgwg
bwgrbbuwwubrbrrubrguggrwgruubbrguuubrwruurgb
uwwuwubgrbwrubuwgurrwguubgrbrrwwbwwrwbbbwrwwugr
brgrwrguuwuwwbwggrugwurgbggwgbggbgrrbruuubuggwbugrwrugwr
brbwrurrbburgurbubwubugbrbbgbgbwgwgggwgrggwwggu
bwgbubwburugguubbgwgbrwrbbbbwrgbuburwbrwrbggrwwrwgr
bguggggwuwgwubgrrruwwgbrubrubwbwrbwrgurrgrgurugwrbug
rrggrwwwbguggwguggrbbwwwwuubbwbwgwgggrwggwgrggrb
bwgggrggrrwgugwbbbwbrgbgggrggruuwguuuwuwbwurgrgw
bwgbbgggbbbubbrwuwggbgurugugrgwrgwbrgwuuu
ugwbbubrrugbggbbgrbubwgrrurgbbbruguuwuwrgwurrwwbg
bwgwgubwgrgbwgwwbrbuubuuugggwgwbbrubgwwwggw
rgbggwubrrrbgbuwggbrburwrbbuwwbbgrrbwbggrrrgwgbwg
bwgbwuwbbubwubugrrwrgbuguggugubgrubbuubrrwruuwbrgbrurrbu
wuwwgbwrrrgwbgwwrwugbgbbugbubgurwgwruwwrubrg
bwggwwbrwgbbururbuwrbwubugwruwbguuugurub
bwgrrrrgggurgrbbbbwwwurggrgugugggbgwbuggr
bwgbbbuwbwrugbbbuubrrgbbururururuwbwrwrgwgbwguuuuuguwruubu
ggwwubgubrwrruburrbbuurbguurrbwgubggrgburrrwrrggguwggwuu
ruwburbgwubbgwbuurwgrwguurguuwgbbrwbrwbugurwrbbrgbrgrr
gruwbgbwwrbgrguuurrrruugrgbuuwbbrwbwgrbggbubwururwbr
grruwrrgrgrbrrbbgrgwbwuwrbuugbgruggwgwgrrugugwbubb
bwgrggugubuubuwugwggguwrbgugurbuwbbbburbrubrwgrwggu
bwgbbwbrbrwbggggbrrbwrububggwurwbbgrbwbwugurgbggwbrgubbggb
bwggwwbwwbgububwrgbbwugwwurbgwwrrrrugggbbugruwrwb
bbbwgrgggwuuwubrbwwgwwurwwbrbuwgwrruugww
gwggwurggwrrrwgbubbggwbrbbwwgubbrggugbrrgrrgubw
bwgbrwuuwwwugwbrubbuwwuwbuwbrbwrbwgrurwguwuruugrrgubbrg
gbgruwbbuwgwguugugrurrgrwwuwggbuwwugrwbgrgwbbwwrbbr
ubbwgwuuwuwbwrrgugugubgwgbruurbggwugwrwuwgruwrgbwgbrrbgg
bwgbrrwgrggrgguwrrbrguubgguugwrgurbbwrwrbwuwgwgwur
bwggbbrgbrrgrwgrwgggrrwwggggwbwgbbbburbubgwrrrbbwgbw
rwwgburwgurugurgbrwrrrgbrugguubrbubwrgwubggubrurrurbwwwrrw
ugrubwugrbgruburwuurrgbgggwuguwuwwrgrwrrugwbuwrruwwwb
bwgwwgrrwbrwbugwggbrrwuuruwgrururugwggubrbwrwbbwbwgbrwwwub
bwgggbbwgugrugrrbbrgrburbgrgubguuggrbgrbuwgurggwwb
wububwbubbwrwbubbwugrggwrbugbrgbwgrburgurwgrubgbuwrbggwu
wuggrrrubgwubgrgwbuwgbwbrububuurbgrwgrrwwrbrbwbbuwrurbbb
bwgwugwubgwuwrgubggrbwrgwwgwgbbbgrbbrggrwwgwrruugwggbb
bwgrruuwgbuggbbubwbuwgrgbwbbuwrrwurubwgwrrwgurb
rgrgugrrwrwrrbgubbbbbuwbwrrwurwgbgubwrrbgggwuurwubuugw
uwrgurrwrwbguubuwbwuugrbwbbbugwwwuububggruuurrrbrrwwbwgww
grrgbgwgrbwbbuwbgurwruwrbwbggrrurgbuwurgbwrurbu
wgubgbbwgurrugbuwwuwrgwrbrbgbgwwbwwbrwggbbuu
rbwrwwbbwwrubburubwurugbgbwgwrrbwuguwbbguwbbbbgbru
bwgwurgrruguwrbwguurbuwwubruugbwggrrgbwbrwbububwuu
bbbwggubrbburgbwrgugugrugggrwrgbugbgugwgrrububbrbrrggwrgbb
rguuwgbbbwurwbrbrbugrrbbggurrwrbbrugugbbrgwb
ugbbbuwrbrggurgbuwuwrbguurggurrbwgbbwurwww
bgwubwugwrrggwubbrrurubwggurgbrwubwuruwgww
bwgrubgwwuubggguwrrggguwwrwurwrwrbgwwgwbuuuwbburbwuu
bwggwgwrrrwgbbbuurrwwuburggwugbbwugbbbrwguurur
bwgggwggbruugbbwwrbgrbgrwwrbwwwwggwgwuwuu
wurwugrgrrrrgwwrguugbbrrrrwrwbbuwugguwggbbwwwgrbu
rrbbrwwrwwgwurubbgbugbrwwgrgbwwggggrrgguwrwugwwwbbwwbuw
bwgbwubwgbbggurubrwbuuguwugrruruubbbgubwrrrrrwwuwuubrgbuw
bwgguwgrbrbuggbrwgrbwruwrrbbruwubgwwwrwb
bubburrrubugrbrugrgwuguwwgurgubuggbbbrbrrgrurwrbgrwugrbgg
bwgwrwbwurbrbgrbwwruurbuwruugurrwwgbuwwuwrbugguwbwgwbbbw
bwggwugrbuwuwuubgrwubrbbbwrrurwgrgbgbbbrrgururb
bwgrguurrwrwbruwrwbbubbrgrbbgbggbrbwwrwwwrrrrbwgwbgguug
uuwubrbwrbbrubwguruuwbbgubbbuuwbbrbrugwrwwwwurrbrggu
rrwgurwwggwwgururgrrwugbbubbrbbwuugrbggwrurgwbwwb
rbuuubrwbuuggrbgbugrgbgugwgggrbwruurbwrwubwrrwwrguurrbbgrr
bwggwwbwwwubburgwbrrubbgbwubrubrwgrgrwwwgbgbbgb
rgrrrrugbwgbwbrrwwwgbrrurwggguwwuugbwrgguwrguburb
bwgrwbgbbbgbugbgwrwubbguurgwrrbubgrurrwrrgwubug
bwgruugurwrgwbbggwrburrbubugguwwuubuuwgruurwubrubbgwr
bwgggrwbwggbgggrbubuubwbggugbrgbbwbrwuwurrurbbrrubwbuubgwr
bwgrbrgwrrbbrrwuwbguguuwuugrrwgbbrwwrggrwrbbwrbbwgrgr
bbwbbrgbwwugrwwwguwuuruggguubggbrrrburuwuu
bwgwwbrwgbbbgurbbrbbubwrgbgwbbbgggbrggrwwbgubwb
bwgrrwgrrbbwwgbwuuwgwgbgugubbbbguggrrrggrbrbrugrguub
bwggbgbuwbuuugbbbggugwuuuwwuwwrruwwuggwbwwgw
bwgrbwurbggwuggrbwgwurwwubggbgbrugugwwurwbwrrg
bwggguwrwbgrgbubbwwggrguwrbwrrbrrbuuwggwg
bwgwwugrurwuruuwbwugubwrrrwrrrwbwuwbruwbwwbbu
bwgbgwrwwrggubwwuubwwbrwrwwuwggbwugurwugrwwubuwubugww
ugbbrgwbwggbbgubgrgbrgrwwbbwubbbwwubbbbgbgwurwgwburubbbgbg
bwgwuurrwgwgugwgbrurrgwbrgwbugrbrrrwbuuwrbuwwwrwgbbrbrguurb
bwrgwuubuggwbggbggwuggbrrbbgbwgwrrrgrgwgrggwrg
bwgggrugwugbgbrugwbuubruggurrwwgggbwruuwbuuwrwbg
bbubbrbgurrubgrguuubrrrwwgwbwuguwrbwgbguwuubuuwgrubrwrwuru
gbgbrurbrwgbgrwugugrwuuuurbbrruwwgburuubww
bwgwgrbbwrruwgubwgwwurbwurrwbwwbbbbuurgbg
ubrwbruguwbbwwgubgbrbrbbgrbgwgubgwgrwwbbggrbwuubwwgbbgwggu
bwgbwubwwrbwrggrgbrrburggrguguururubrrwbwbwrg
bwgbwwrbwwwgwurwggwrwwrrrbguggbrubgugbrrrgbbbug
bwggwbrbwguuwwuugrgwwrguugrwuggubgbugwwurugbrg
bwgwurbwrrrburwgrrbrubuwgubwwrrburggrwggwrbuuwbbugurwwg
bwgrbwrwwggugwbrbgguruubwwwrrbrbwgwwbbgrrbrruurwg
bwgrgrbbuubwrurwrbuwwrbrrgbbbruuburwguurgbwrgbrbugrg
buwwwwrgwrwgwgurrwwbwuggrwbuugbbbggwwwggwuugw
ubwuuwbbbubwgwuurrrrgrrbbrbruwrburbuggrwbbbrwubwgg
bwgwbwggwgrgwgwbwurguwbggrgwrggwrwbuwgrgwwbrwruwrbubrwgu
bwggugrbbuurwbwguwwwggubuwggbbruuwwurbbgbgggbggrrb
rwgbugruwgbwuuugbbruwurrubgwruwwwwrbrgugbrrrrrwurgrgggbuw
bwgguwbgrbbbwubwbwugbgbwwbbwwbgbwbgbbuurwrgbrbb
bwgrwgrwgrgwgrrrgggrruwbgwrgbbubrruwburrrbguwrbw
bwgbgbrwbwggwrrwwrrwwbwbgggwrgurwggwurwrwbgwwbr
wggrrbrwgbguwrwwwbuguwrurbrrbbrbrwwuwgbgrrbuwwwbugrwgbbrw
bwgbubwwwguurgwbgbgrwbugbuwurrbwggwuggggbwwgwruwbubgubugwg
bwurbbugwrubggwbgrbuwbgrwwurrgrgggrugwrgbugrwbrguwruu
wgbrgrwgbwgrguwwbugrbwggwrwrwrbgrgruugbwrwrbgbbgwwruwubbr
ggrbbwwbuwrgwrgbwwrbbrrbbwrugwgbggwrrwbgguwr
bwgrwggrwurrbggwwbuugubuggubggbburbbububrwbgrrbrubuwuru
bwgrubguuwugbrggrgbgrwwbuwrbgwrwbrwrwbbgrg
urbgurrrrrwuwbwubuwbwuwruurggrbugwrbrwrruurwwbwrbgrgugbuu
bgwggrgugggrwrrugggburwgruburrguggbbrggururuggwubgrbbgu
bwgwwrrrbrgugbwwbrwrgbbwwguwbuuubgugwwwwurbbbbwbwbbrwub
ggwrggwrwgbbubuubruuuwgwuwgbuguubgubuguruwrrruruwgwb
bgbwugwrgrwubbrbbguuuwgrubbwubbbgwwuggbwwrwub
grruwwwgwbruwwuwrbgwbbubwubggbbruguwwbrgbwwbrbgurbguu
ggugruwgggbuwurwgrruwwgugwrbubbrgguuwwwwuuurwwguwwrrwu
bwgwrwwgrbrubwrrbruwggubuuwgurbbbruwwrbgubw
bwgguggrrgbrbgubgburwrrrbwwubgbgwgwgwuugwugu
rrwwgurgwuurbrrbwrwwgrgrubuwubbbburwbbwrrbgg
guwwrrwbbuwbugbguurgruubbrbrwgbburggurbuwuu
bwgbbbgbrwrgrrbbubrburgugrrbuguggguugwbuwbbgbbggbubggbuurub
bwgbruurubgwgubrbbgrwgruwwgrrgugrwrrrrruwgruurwgruggruwwrwg
ggrwwbrurgbwrwubuwbrrwrbrbwuuggrbbgbrwbwrbrbrbgubbwgwgbw
brbrubbugubguurugrbwuggbwwgwuuwgggguubuwrgggbgwgbwgrbrr
wruruuwruugbbrubbbbguwuwwwuubrrwgwgrbbuwwguuwwrwr
bwggrgrrbrrrrbguuwurrrurguwugbubgbbuwbrburrrwuggrgbbwwuw
bwgguurrburguubwgrwuubwbgrrbggubuurbggguruguwrbbrrgwwrru
wwugrbbrwwwwbwugwuugwbuggrgwgbggbgbbbwuwuuruw
ggbgburbrrwrbrubbbwwgbbugwbbbggwurrrgbwrgwuubwubgbbrgubgbu
bwgwbgrrggruruurugurgwrbbbwbwwruuwrugrrrrwwwrwbgggwrwggg
rugbgwurrbugwwbrgguubwggwwbbwbbrwgguwbwrgggrbwwurrbrgrbrgb
wuwgbwugbbrwwgurwggrrgrurwrbbuwgbrwruubrgr
bwgggugrgrgwrgbuwgwbwruuuuuguwuwbrbrbgrugrubrgggbbgrbb
bbbwgggrwgbguubgugrwgrwrguwgwubwbwuruwrwbgbgurbbuu
uuguuwggbwgruuggrbguwrrgbbuwggrbwwurwguwrrwbgrg
bbbwgggwgwbwwgbrwuwwwuwwwwwuruugbwwgrwwuwurbrrububwwbrrw
bwgwurrbbbrruwwguburbrgrbbggurrwgbrgrwggrruuurgwbrug
wgwwwrbwrgubrgwugrbwguwuwrbbrugrwwgbgbwgwrgru
bwggwrgwruwuwwbwurbbwwruguwubuuuwwrbbbgwbwrguwbr
bwgbuggbugwbubrugbgbuugrguuwwugguwggbwrurugu
bwgggrwubuwrwgbgrgbbbubuwggbubguwrwurbbwurrwr
bwgbguguuguuwggrbbrwbggbrbuwbrrugbgrbbbruubuurgugu
bwgguguuwuuubwgurrwurrurwbwwggwuubuguggwgugubwwu
rgwgbrrwguuggrbbrrrbbuugrguwbrubgbguwggwwrgb
bgbrwwggbbgbbbbuwbbbrbugbrrwrwrgwgbruwuruwwbbbgwbug
bwgrruwwgugbgrrrbruwgbuguwrrugrggrwggbgruuww
bwgrgrgguwbburuwubwbbuwrubgwuwbgubwwggrwrggbwubgwuwbwrbruu
wugurwgbrurugrrwbgbugruggwbubgwurrguuwugbwrbwruurbwubbwbu
bwgguruwrwguuwwggububurbwrggrgbrrwrwuurwgr
bwgrrbwwwuuuurrurwwwgwurbrggbggwbggurgwbrbuwwgbrgbwwu
rbuwuugrgbwgwbuurgruggrguwbbburgubuwuurwuguwrr
burgbrbuwrwbwbgwbwgruurbwbuggrgwwrwbwwgurrrrrgrrbuwburwg
ubgurwrgbrrwguggugguguurububruwrbubrrgwgugrbgguruwgwuwuub
ubgwuwgrurwuuwubrwuugwrrgrgubrwbwggruugbrrgguuugwwbuug
bwgwwugrugbbruurggrbuugbwrbwrbrbwggburrrrbuurwrrwwgwrb
rwwrbrbwbrugbgrgurgbbgwwgurrggubrgwbrgrggwwuuwuwggbg
bwrwuuwuguwuwrguugururgrrwuguugwwwwrrrugbwruwrbr
brbwuggguwguwrgrguwububbwrrbgrubrwbrubgwrrbrbrrbrwbbgru
wrrwubuugrwgbgbrwbguruuwububgbbgrbbwgwwrurgwuugwbuurbubugr
gguguwwgbburuurbwwbwbbuwruruwwgrrbrbgbruubbbrb
bwgruuwbrbbwgbrgwuuuggbwwbwbgrbgggggrugurbburuggwuuubbr
rgbruwgbbrgbubwbwrggbwbuuubrggbwgwbrggggrurugguwugwwwwruwu
bwgrwuggwrububwwgwugugrwwuwbubwgbwrwwrbburwurrrwgrbgurgrwgru
bbugrggwwgwrruubuwwgwruuwggugruwwubbbwgururruwbrrwruwgrrrr
wwrrugguwuguubwugwurwuurbwubwuwgbbgruuurbgb
ugbwrgwwugwrguurbgubwgbwuruugurwwggbgggubb
rwbbbbrgbugwuwrurrbrbwbbuubggubgbrruwgrwugrwg
wwuubugbbruurrgrwguwrgwuggrgwrrbgwbrgwuwrururugguwgbwbrgr
bwgruubuwruwrbwwwgruwwgrrbruurgguwwuwwggrbruggbguburbrubguwr
bwgbrwbrwbbwguubgrwbgrurrrbwurubrwugubgwuwugbwgugugbwugbgbb
uwrrrubrwbururbbrbbbubwgbrguwgrwwugrubgwbrwbr
grrrgrrbggbuwrrrrbuubgguwguuburggguguggbubbwwuggwr
uwuuurgwgubrwrwgguuugbrbbbwubgubrwbbguwbgbrbbur
gubwrbrgbrrwggwburwrgrbrrurwgrbbwgrrwwugbww
bbgwwrrrwburbbugrwwugrgrrugrrbgbbwgrrbbgugb
bwbbrwrwwrbwgwuuubrrbrurgwwrgwrrbrgbbuugururubg
rbrgguuurrbuwwggwugururbgggwgrrbbwrrguuwgrwgbrwwbgubbgrg
uuubuwwbubbwurrubrrwrrubrruugbrurguubwbbgrgguurbguubbrru
uuugwbbwgwwrwwwrrbuubwwrrggwgwubuggrwrwwrrwb
gwgrbubbwbrbwbgrbbrgggbwbwubwggrubgurbbrggwrburgur
gurgubgbuwwurrguggrrwrugrwwwgbrggbbwwruuwrbbwwrwgguwbgguw
ruwwubwwbubuggrwuuwggggrwrbrwbubugbwwbwgwuuwgg
bwgbwbwbuuwwbbwgrgrgwbgwgrwwggbrugwgubrbbwgugwrrwwwgbwrrr
bwgwrrggwruguwwgrgubwwwrugbrgurgubrwgwwwrwgbwggwbugbwwwbub
bwgbugwurbbwurwggrrbgrrbrgruwguwwbwwgguuurbbgbrburubwbbu
bwgbgbwwwwugbbggbrrwrbguuwwrbwubbuuguwwggrgu
wugrwwwwwrrwugbrgbrrruwbrgwwbbgwgrgrgrgrurrrburrguubwuuuu
rwuwgbuwgrurrbgwwwuggwbwrrbwrguuguguuggwrgrwbubrgbgrug
bwggwguugwuugrgwgurgbguubrrwwgbrgbrwbuurrubrgbwgburubrbwgru
rgugbwurbgrwgburwgrbbbgwububgwwburbuuuwwbubbbrrbbbguuwbrg
bwgbugbwgbggbwuwwbwwuuuggrrgwwuburbgrwrrrrbrbwgbwbwuguww
bwgwrrwgwwuuggurwubgbugwbrubbgrgwuggwwrbgbwgrwgrbbub
ruwuwbrggrwwbwgwbwubwuwguuububbwbbugrurggwbruu
rugwwggubbuwbwuuuurgrrbbuuubwwrwrggbuuguggggurwg
bubguuruwbrrrwwwwugbgguwrrrrwbgugwwrgubuuwgbbwwgbubub
bwgwugwbrubrbwbgwbuwggwgwwwwwrgbggwbuubruggbbbwruurrwububrg
rbrgggurgwbbuwruwgwwbwgugbwguggguwuruwgrbgrgbwubgrgubbbb
rrggrubwburggrbbggwgguuuurgrbrwggwgrbuguwwuwbgrgugb
bbbrwgububggbubwwwurbbruburrggrguwbwguuruwbwugwbbw
bwgbrguuuwrbbrgrwrgurggrwwrgbbrugwuwbbrubgrguwgurruwwbruub
wbuuwwbwrrburbbuwruuuruwuwbbgrwgugbbwurwruwrrwugw
gbwuwuuburwggwbuguwwguwurgrbggwgruguuwwugg
wbbrwwrggruubbgubgurwburbgbggrwruwuwrwburrbggrwggwgwrugb
bwgbgwgbgbwubbgwwwgwurubrrwuuugruggbgguuuwr
bwgggbwgwurgwgwguubggrwurwubrrgbrgrwgubbggrbwgurrgrrgrubggw
bwgrrgbrgbwrrwgwgbrbuwrburrwbuuburrrbggwwgbrrbwrggr
uuwrbwgbgurgbgwurrbwugugwwuuwbbbggbbwugwuwbrbbg
ubwbururuuuubbgrrggrbuuwrrwrbbgwruuuugwrbw
uwrgubggbubggwwrbbwgrgbwgbguurubugwgrwwwugugurubwrwruwwr
ugrrruugbrbggubuubwrwruggbbwugubwgbuubrgguugug
bwggrwwubrrgrwbrbrwrubwbgubuggrburugrugggub
bwggwbuuurrbbbwbrwubwubbbrruggggugburuuwuwuggbrg
bwgruwwbgrubrugbuwubrruggrrrwuruuwrruurwbrgg
gbwurgbbbgugwuwwgrggbwrrrgbuwurwgruwugbwuwgbgwrg
bwgbwurwuwggrgugbuwggrguwbbwrugwbuguwugubgwgbrgbrwrwuggw
bwgrwbguwwgrwuuugugrgbubbuwgrgrgbgbgwbrgurwbrubbubrwuwubgrrb
bwgwububbrwbrurrbbwrwurguguugwggguubgbgrurrgu
bwrurgguuwgrgubwbbgwbgbgubwwrbgwuuuggwgurgubgrgrurbrurwrgw
grrbugbruurgubrwgwrwuruguwwbuurggggurgwwbbwwwguwu
urrwwbbrwuugbrurrbbbwwbruwgggrggwrrwrwbggubggwr
bwgbggrggguubwuurgggbububgggrrgbwurbgrggwrgbrwbgwwwrgrb
bwgwrwwrwwurbgrwgwrwguguwrbrrwrrugbuwrgwu
bwggwurgugrbgggrrwwwrwgwbbwwugugbbwbwubbwbwurrwgbgurwbubwwgg
grrwgrwbburgwrrggbbbugbubwrwrbgwgbbwggwwbwbwwwggrggwr
bwggbgbggbbubbggwrugbwwrwbbugbrguubwbwrgrbuwrruruw
urrgwrwrgrbwwwrbuurrbbgbwurrbuguuwgwgwwwrubbwbbrrgb
bwgrggubuubgwrbwggguwbwwugwwbrgugrbugrgbgbwrbgrbrbwwrrrrb
gbbwubgrwrugbwwwubgbwwbuurrbggbburbgwuugurwrwu
bwggubbwbuwwbuuwgbbwgbbguruwrurubwrbggbgbbwwuwururwww
bwgwwbrrgbwbrgruwbrwurwubwgrbggwggrrurubrgrgrbbbwruubrurrr
bbbbbwgwwbgbrgrrugwbwuugwbgruwggwwbbugwbuugbrgurwubruwb
bwgwwbrgbugwbrwwwgurrgwbbbbrwwwrrgbgbuwbggrwwrrruugrgrgwbub
bwgrbwrggwrrrwbbwubwbbbubggggbuwwbggwwbrurgwbbw
buururuurgggururgwrrwgrrguwwwwgrbuwwrrurugruuuruuwwwurwgr
rbrwwuwugwwugubwbuubbbbguwgruggrrugurbgwbb
bwgwrubugbuuwbbrgwggrggrrwbrbgwruuwrwwbguuuwwwruggwgguwg
wbwubuuwgbrrbrruwbwrrbrwbggugrbwbwuurrwwruuruuggbbubruwwb
bwgguugbwgbuuwrwbrwwbbgwgbgubrgbbwrguugbugwuwuwrgrr
rwgbbugrwrbgugrrurbburrbbgrgugwwgguwwrgrubgrgbguw
bwgguuugrrrwbburgbrbwuuguuwwubuwwrrrbrgrbrgbgbu
bwgwbuggwurbuwbwwbwurbwbbrubgwbwguguguwggwwwbgwuur
gugrrbwguwwgwwgubwbgrugugbbbbwurrwwurwubbuggugubuwruwgg
wugwubugggwwruguggbgggbuwbbugrrrruwwwbwrrrrwubgwurrwbuuu
bwgrrrrwgggwgruwuugbbgggbbgwgguuubrugggrbgwbbgrgubgrwuwgg
bwgbwrbrggbburguwubggrwurrrggrbgwrwwuggrgruwurur
wwbwwgwgwbggbgrbrrruggwbrgwrugbrrrbwrbwwwwguwggwu
bwgrurubgwrbbuggwrwbrbrgbbbbwruguwbwbrurwruggrugrbwu
bwgrrrgggwbbwguugrbrgbubrwbwgbrbwguurbgruwgrwbrwuwugwburru
bwbwbbubbuwwwrrwwruguuugwwuwwurugruurrrgbw
bwrbrwbwgbwbbugwurrruwurgwbwugubbbrgrwggubugw
ggrwwrwbbwbrwrwwrbuuubbrgbrwwbwguurrubuwgwug
buwrwrwurugurgwwurrwrrbuuwrbgwwbbbrrrgrurugbruggu
bbgwwwuuwwbwgwuugwrburugbrwwuwgubgrrwgwwug
bgrrrrrubbwwurbbuwbwwbrbbuubwrrubrrwwbruuruuwwrbubbbwurbgu
bwggwrrbwwgrrrrubrgbguugbuwrbggrbwrugwrwubwguubuub
bwgrgrwbgruggwwbrbrwwrubgwruubgrggwwwubwrgbbuwg
bwgrgwwgrugrbuuruwwrgrguurbrrrruugwrggwub
wrrgbwrrrburgubuwrbruwuuurbgrrwwggwuwrrrguu
gbggrbrwbwrggrwwrguwuwgruubrgwwuwrbugrgubgbwbgruurbggubuwu
bwgbrrbuuwrbgwguwbgbgbrbbrwrwggwrrwrbbbbuubrbub
bwgbwwgbwwggbwrrrrubggrgrwbwgrgurruguwbrrrgg
bwgbgwwrwgbbwuwgruugwubbbwwwururbggbrggbuurbwwrbbbwbrbwu
gbwgrwugbrwbbuurbruwrwrgbugwrbugwggwgbbruruwb
rbugbwuwuruuugbwbwrgwuwrgugbbrwbugbguuwgbwuubu
wgrrbwbrrwwrbububwgbbuuwgurwggugwburwrggrurwuubrwbwbbguugg
rgbuggbwuwrwwrrbrgbwwgwgwrrgbrrruugruwrbbwurgwr
rubrwbuwubbgugbbwurgbbuwbrbgggrwubrwurrbubuurwgwwbbrbbbw
grbbgwwbbgrrrwububururgurwwruuuwbbuuuuwbgwwwrbgu
rbugwugbwwbgwrruugbbbuguwbgurubuguubguurbwubwwgwbwuguggrbg
bwbuwbururbgugbgwbggwbgwgrgwuwwuugubuwurgbgbuuubuubggbb
bbbwgwrgubuurwwgburrgbwbgbbwurgbgbbgwuuggwbwwwwrrgggwburww
bwggrurwwwggrbbuuwurgrugrwgrbbrgbbuuguwgubuwg
gwuurbuuwurgwwuuwbbwrrggwwbgwbruuwwrbbrruurggbbwrgbubrrgr
wbgbrbuubwuugwrbuwgwrgbbrubrbuugruruubgubwggurgbbwruw
bwgrgwruurguwbbrwgrurrrgwwwgugrbrrwwrbugruwugwbgwubrwwu
bwgbgugwwgrwbrgruwuuruwbrgwbwggrbbrrgwrrrgurgubuurbuwrrwu
bwgwwgrrrbbggwrubbuggwrrbubugwgrugwrbgwwrwbggwbuurrugr
bwgbuuggwwggbwrwgrrbbrggwwrbguruwbbrggurubwu
bwggwrrbrbwgbugwwgbwrwwbwbgwrwwggwubuurbwgggwburbb
bgwggbgbbwrgwggbbrubwgubrgrwbbgubrbuuwwugwbuwgbu
bwgwguwurbuugbbugurubgbbwguuggwbrrggwwwwuwgwgrwbbuugugu
ruurugwwruurbbbgubrwwgurwggwgwrgggwruuugrbbw
burrwwgruuuurbbgggbrrgbuwgugrwwrgbbwurrggurbwrww
bwggwubbwbubbrgrguwwuwgruugwbgwrrbubuwwggg
uuuwgwwubuwbrrrgurgbbwbuwrbuuuuwgbbrggwwbgurguggrurrrrb
rurugrrbubrbbwrrrwugbwuubwgbgbbubbgbgwbrburbbubgugguwbbrg
ugbuurgwgwbgwrwwrwrwrggrrbwuwrbuwwubwrburwgbrgguuw
rbrbwguwgbgbruuuggrbrbrbrurggwwbgurbubwuuwwggwrbggwgb
wbrurrwbgrburrruwrwrbguwuguuuuuruwwwwgwbwubggwwwr
bwgrubuwgruurwuubwrrwwubrwrubwrurbbwgwgwwuubbgb
bwgrgwgwggwwrrubuuggwwwwgbuugurgwbrrwrbuwrggww
wbrubgwrwwwguuuguugwrrwrguuruuguuuwrwwrrgrgurbbgggugurrwb
bugrwbgrubrguuwrugbbwrgrubrwrwbbuwubgrugrr
uuubbwbrgubbwubrbuwburubbrbbwruwgurwubwwwgwuwbrgwub
gwbrwwbuwbrbwgbggrrrurugbggbbbbuwgwburrbbwrrurr
wbgggwwgbgwuugruwbugrwbrrwwrgugrrwguwwwrbwgbg
rubwuuwwwwwuburrwurugbgbububggrrwuggurbguggwggbwuruwru
urbrgbwgrwuurwguguwubuuwuggbwwrbbuuwrrgrbwrrg
bwggbrbguuggwwwggrwwbrrbububbwbrrbwwbuuggbuuuwubburwuwb
rwwgurwgrwbbuwrwurbrbwwuruuuuubuwrrbrwgbuw
rbgwbwrrwrwurrgwwrgbwurwbrrrgrbbgbrwbwrugwwbrwwruwubu
rbguwbbrububwbrrubrrubbwuurrugbgrbwwgbuwrugbugburgguruuu
wwuwuuwburgrrrurwwgrggrrrbrubgbbwggrbwwgrubuuwgb
bwgrwurwuuggrgwuugrbwgrwrwrrggwwgwwgwbbguubwwwbgugr
uuuurwrbgrubugwgwwwbwuwbuuwbbrwubrwbwuubwbgbuwubwurw
bwgrgwwrggrbbrwbggwuggggrrbbwwuguuguuwuu
ggbwbbugbrwgggrgurbbwbbgbbrugrbwgbbrrrbrbgwu
gwwugrbwwurwbrwwwgwurubguguwwuubbwwwurwguwgg
wrrrbwrrurbrgwggrrrwggwbbbwrwuubbrwbrrwrubbwgwgbruruub
rbwrwbbruugrrgguguwggguwrwrurwugurggrrrguugbww
brgbrwbbbwrruggwwguuggugrbrugwrgwuwrbgwwurbuuubwwrr
wururgbwguuguruubwuwbwgurgububgbubwgubgububbrwguwr
bwgrggwugbrbwwwwbrggggurwgbuwbuwbuurrbgrgr
bwggwuwrgbbgubrwrguurggrbuububbwburubuurrurwuwbubwrbr
rgbrbbuuuwururgbuurrwbbrguwububrgrggrwrububbwguw
rguwurwruuwbwwuwgruugbwgwwbrwgubrwbbgurruwgurgwbugggrwbw
bwgrwwurugwbwwuuggwuwwrwgwbrgrwrgwbgurggw
bwgrwgwgrrwugrgrgbuurrurwwgbrwbrrwwuggrgbrb
bwgwgbuwwwrggbrbgrguwgrbbbuugrbbbrubrggruurrgwwrubrgurrw
bwggbrgrguuuwrbbbwwrbrrbbggbubrrguurgwuwrrwbbbu
bwgrgwbgrrgwwbgwrruubuguwuuubbuwbggbwguwgggwrrgbrrurbwgr
uggruburrwugbrwrubugrgwggrbwburgrgbrwbggubruu
gbrbrrrggbgruuugwugwbubgbguwbbwrubwbrrguub
guuwrbwbrbwgrbuururuggrbguruwwrwubbrwbrurg
rbwrwbgruwgubrbbgwrrrrrurrbbrrwgwubguubrwwuuuwbbbuugw
bwggrwrgrwwgruugwbrwrurgwrbwrwgbgbrwbwuggrgw
bbbwgwrrgbubuuwuwwrugburbbugrwwugbwggubwwuubuuwrgrubrbgbgw
urbgbwwbrwwbuwrrbggrbrbrwuwurrbuwrbrrgrwwbrbubbbbugb
bwgrbruubbggguwrrrgbbbburubwbrbwrwgrbgubgwggugwuub
gbwbggrgbrubuguwguguwuwbrurbgbggugbgrugubgb
uwuubrbwrbrgwugwwgrwubuuwubrggruguurrwrurwrrru
wrwgguwuuguruurgrwwrbwwuwwggbgrrgwuwgrugwrrwrbguwb
uguwwgbbrwwgwbgbbugbubwuuuubrgbrurrbuuuwubwrbbrgg
bwgguubgbgwbwrbgbubwggrububgrwwrgwwwugbr
bwgbgugrrgwbbgruubrrbwrgwwbwuwrgwwwwrruggwbrwugru
bwgwrbuwggguwrgrwrggbgbwgbbugwbbgwruuubugrubwgbuwb
bwgwwrwwgrrurgbbbrggbgrrwugwugurwrwgbbwuguurrubwwbr
buwggrbguuburggbwbrrbubbbgrbwgbwwwgubwruwrbubgwwgwwrguugbw
bwbbwwgwgugrgggbrrbbwugruuggruggrburruubbrur
bwggwubbwgbbbbgwurwguwuwwwbubwgrrgugugbbrgwggrwww
bwgrrrbwwugrrrbrwrbgwuwwguggburuggbuuburrgrguubgwwwb
bwgbwggrrrwguwwuwugrrbguwrwwwgbugbruuuuugrrbubu
bwgruubbbgurbrwbbrwuwbwgrwubwgguuubuwuruurururbw
bwgbggwbbwbwbrbburwggubgrwbgrubgbwubgrgurwwurur
bwgrgggbrrrbrgrruburgburggwrrbwbgrwgrgwwbgwuwg
bwgwwbwgwrgugbggrrurwruggggrwwrururwuwurgb
rrgrgurbwwrbbbgbbbgwurwgrurrrbubgguuwbgrrbuuwwbrbw
rubbubgrrwgggbugggbuwbbggbuurbuwubwwgrgrrbwuguggguwuw
bwgbwbuwgbbrbggbguurbrrbuugrbbbwrbbugbwrwwruurbgwwb
ugwurubwguubrbwurguugbugwrgbwwbgubgbwgbgbgurb
bgbbbuurggwbbbwbrrugbgrbbwuguggbwgbrgrgugugggwbrgwrwwruu
gubbbuubbrrwwwbrugggubbbwubwwbrubgggugbrggbrgubbuugwbuugb
bwgbbugrrggggwwuuwwggbgwbuwrbgwwrrbwgwwrwgwgrwgbr
bwggwwbuwbbrbubgggbbwbbbgwugurwwwgrgrrbrrwrgbrrwgbgwrrrgwbwg";
