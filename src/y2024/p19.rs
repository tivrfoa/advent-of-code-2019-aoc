use std::collections::*;
use crate::util::*;

#[allow(dead_code)]
fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (tp, dd) = input.split_once("\n\n").unwrap();
    (tp.split(", ").collect(), dd.lines().collect())
}

fn solve<'a>(s: &'a str, max_tp: usize, tp: &HashSet<&str>, memo: &mut HashMap<&'a str, bool>) -> bool {
    if let Some(v) = memo.get(s) {
        return *v;
    }
    if s.len() == 1 {
        return tp.contains(s);
    }
    for i in 1..=max_tp.min(s.len() - 1) {
        if !tp.contains(&s[0..i]) {
            continue;
        }
        if solve(&s[i..], max_tp, tp, memo) {
            memo.insert(s, true);
            return true;
        }
    }
    memo.insert(s, false);
    false
}

/// How many designs are possible?
pub fn p1(input: &str) -> usize {
    let (tp, dd) = parse(input);
    let max_tp = tp.iter().map(|s| s.len()).max().unwrap();
    let tp: HashSet<&str> = HashSet::from_iter(tp);
    let mut memo: HashMap<&str, bool> = HashMap::new();
    // dbg!(max_tp, tp, dd);
    dd.into_iter()
        // .filter(|s| solve(s, max_tp, &tp))
        .filter(|s| {
            let tmp = solve(s, max_tp, &tp, &mut memo);
            println!("{s} {}", if tmp { "solved"} else {"impossible"});
            tmp
        })
        .count()
}

pub fn p2(input: &str) -> usize {


    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sample() {
        assert_eq!(6, p1(SAMPLE));
    }

    #[test]
    fn test_p1_in() {
        assert_eq!(171, p1(IN));
    }

    #[test]
    #[ignore]
    fn test_p2_sample() {
        assert_eq!(171, p2(SAMPLE));
    }

    #[test]
    #[ignore]
    fn test_p2_in() {
        assert_eq!(171, p2(IN));
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
