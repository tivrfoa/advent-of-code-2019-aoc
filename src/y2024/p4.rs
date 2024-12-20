use std::collections::*;
use crate::util::*;

pub fn p1(input: &str) -> usize {


    0
}

pub fn p2(input: &str) -> usize {
    let mut qt = 0;
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let rows = grid.len();
    let cols = grid[0].len();

    for r in 1..rows-1 {
        for c in 1..cols-1 {
            if grid[r][c] == 'A' {
                // M M
                //  A
                // S S
                if grid[r-1][c-1] == 'M' && grid[r-1][c+1] == 'M' && grid[r+1][c-1] == 'S' && grid[r+1][c+1] == 'S' {
                    qt += 1;
                }

                // S S
                //  A
                // M M
                if grid[r-1][c-1] == 'S' && grid[r-1][c+1] == 'S' && grid[r+1][c-1] == 'M' && grid[r+1][c+1] == 'M' {
                    qt += 1;
                }

                // M S
                //  A
                // M S
                if grid[r-1][c-1] == 'M' && grid[r-1][c+1] == 'S' && grid[r+1][c-1] == 'M' && grid[r+1][c+1] == 'S' {
                    qt += 1;
                }

                // S M
                //  A
                // S M
                if grid[r-1][c-1] == 'S' && grid[r-1][c+1] == 'M' && grid[r+1][c-1] == 'S' && grid[r+1][c+1] == 'M' {
                    qt += 1;
                }
            }
        }
    }

    qt
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // solved in python
    fn test_p1() {
        assert_eq!(171, p1(IN));
    }

    #[test]
    fn test_p2() {
        assert_eq!(2003, p2(IN));
    }
}





// -------------------------- INPUT



pub static SAMPLE: &str = "";

pub static IN: &str = "SAMXXSXXSXSAMXMMSMMMMSAMXSXMMMAMXXMSSMSMMMMMXSAXMAMMSSMXMXMAXSXSASMSSMXMMSAMXMMXMMSSSMMMSAMXXSASMSAMXMSXXAMAMXMMXMSAMXSMSMSXSMAMXMASXMXAMSMM
MAMMXSMMMAASMMMASAAAAXAMASXMASMMSSMAMAASAASXMMMMSAMXAAAAMMMSAXXMASXAAASAAXMXASMXSAXMASAAMMSXMXAMASXXAMXXMMSAMXXXAMXXMXXAXAAAXSMXMAMMAMSAMAAX
SAMXAXAAMAMMSXMASXMMMXAMASASASAAAAMMSSMSMMXAXAAAMASMSSMMXAAXMSAMXMMSMMAMMXAMXXAAMMXSAMMMMAMASMSMAMXMSSMAMAAAXXMSMSAXXAMXMMMSMAXSSMMSAMMASXSM
SSSMXMXSXAMXAXMXXMMAMSMMXSAMASMMSSMXAAXSXMSMMMMSSMMAAXMAMMSMMXMASMAXAAMSSXMSMSMMMXXMAMSXMASMMAAMAMMAAAXMMSSMMAAAXMMSMXMASXAMMMMMAAMSASMXMAXA
SAMXMSMMXMSMMMMSMMMAMAXSMSXSXSXAXXXMSSMMAXAAAMAAAAMSMMMSAXAAXMMAAMXSAMXAXAAXAASXXMMSSMAMSAMAMSMSXSAMSMMMAAAAAXMAXMAMXAXXMMMMSXXSSMMMXMAMSASM
MSMAAAAMXMAXXAAAASXSSMMMAMAMXXMSMMMMXMASMSSSMSSSSMMMXMAXMSSSMMMASXMMMMMMSMMMSMSMMSAAAMMXMSSMXAMXAAXXAMSMSXMMMMXSXMASXSMSMXSAMXXMXASMSASXMASM
SXSMSMSAXXAMXMSSSMAAXMAMAMAMSXMASAASMSMMXAXMAXAMXXMASMAXXAXXAAMSMMMAAAXXMAMAAAXXAMMSSSXMAXXXSSSMMMMMMMSAMASMXSAMASXSMXAAMAMASXXMAXSAMXMAMAMX
MASMAMMASMMMSXAMMMMMASXSASASAMSASMSMAAXXMXMMXMAMXSAAXMASXSSSSMSXMXSSSSXSMSMSMSMMXSXAAXAMAMAMXMAMSAAASAMAMSMAAMXSAMASAMSMMMSASMMAXXMMMASXMSMX
MAMMSXMAMAAASMMSAXMMMSASAMMSXXMXMXXMSMMXASAMMSMMMAXMASXSAAMAMXSMMAXMAMXSAMXMAMXMSMXMMMXMXAMXASXMXSSXMASXMAMMMMAMXSXMAMMAXXMMSASASXSXSXXXMAXA
MXSAMMMMSMMXSAXSMXXMAMMMAXXXMMSMXMMMMASAAXMMAAAXAMXMASAMMMMAMSMAMMSMMMAMAMAMXMASAMMAAMXSXSXSASMSMAXXSAMXXXMAXSMXMXAXMMSMMSMASAMASAMASMMMSSMM
AXMASAAMAXMXMXMMXSAMSXMMSMMASAAAASASAAAMMMXMSSSXXXXMSMAMAXMAMAXXMMAAXMAMMMSMAMXXAMSSXSAMXMXSXSAAXAMXMASMMSSSXSAMMMSAXAXXASMAMAMAMAMAMAAAAAAM
XSMASMMSASMSMXMAASAMAASAMASAMSMSMSASMXXAXSAMXMAMAMXMAAMMMSSSSSMMASXSMXAMAAAXXMXXAXAAMMMMAMXXAMMMSSMMXAXAMXAMAMMMSAXMMSMMXMXAXAMAMAMASMMSSSMM
SXMASAMMXSAAMSMSXSAMSSMASAMXXXXMAMAMAMXMXSASMMASMMASXSMAXMAXAAASMMMAMMMSMSSXSAAMSMMSMAASMSSMSMSXAMMMMSSMMMXMXMXAAXAMAMASAMSSSXSSSMSMXAMMMAXS
AMMAMAMXMMMXMAAXAXAMXAMXMXMMXSASXMSMSAASMSAMAMASAMAMXAXXSXMMXMMAMAMXMAMAMXAAMXMSMAXXXMXMXAMAXAXMSSMSAMAMMMAXAAAXMSMSASXMAMAAAAAXAXXMSMMXMAXX
MMMSXMMMXAASMMMMMMSASXMMMSXMXSMMAMXAXSMSAMMMSMXSMSSMSXMMSAXXXSAMXSMMSXSASAMXMXSAXAMMMSAMMSSSMSMMMAMMASXMASMXSAMXXAASASXSXMMSMMMSMMSMAMMMMSSS
XMAXAMXASXSXAASXSAXASAXAAAXMAMXSAMMAMMXMAMASAMXMXAAXSAMASXMASXSXAAAAXAAMMXSAMMSASMSAASAXAMXXMAAXMAMXMMXXXXAAMASMMMXMMMMMAMAMXAAXMAAXASMMMAAX
XMASXMAMMMMMXMAXMASASXSMMSSMASAXMAMXAMXMASMSAXMXMMMASAMASAMXXAXMXSMMSAMXAASASAMXMASMXXMMSMMSXSXMSSSXSMSASMMMMAMXASAXMAXSAMASXMMSMXSMMXAAMMSM
ASAMXSXAAAAXSMSSMXMXMXMAAAXMSMMMSASXMAAXAMXSAMSXMSXASAMASAMAMSMMMXAMMXAMMXSAMASXMXMXAXSAXASXAMAXAXAAAAXASAMXMSSSXSSXSAXSASASAXAAXMMASXSMMMMA
AMASAMXXMMSSXAAAAAMAXMSAMSSMMAMAMAMASMMSMMAXAXXAXAMXSMMMSXMMMAAAXXMXASXMSAMASXMAAMXXSXMASXMMSMMMSSSSMXMXMMMAMAAXXMAXMMXMXMXMMMSSSXSAMAXMMAMM
MSMMMSMSMAMMMMMSXASXSMXXMXAASXMASASAMXMAMXASXMSXMXSMXXAMXXMXXSMMMSSMMSAAMXXAMAMSMMMAMAMAMASAXAAAMXXXXXSMSMXAMMXMXSMSMXSAMXMMSAMXMAMMMMMMSASX
XAXMXSAAMXMAXSXMMMAXAAMSMSMMMMSMXMAAXXMASMMMXAAXSXSASMMSSSMSMXASXXAAAMMMMSMASXMAAXXAXAMAMXMAXSMXSXMAMMAAAMSSSXMMAMXSMAASAMXAMMSASMMSAMSMSASM
SXSXAMSMSMSAXSAXSMMSSMAAAAXMXMAAXMMSMXMAMAAAMMMSAAMXSXMAMXXAAMAMSSSMMXMAAASMMXXSMMSMSXSAMMXSXXXXSAMMSMMSMMAAMXXMASAMMMMMSMMMMMSASAAMAXXASAXX
XASMMMXXXAXMASAMXMAAAMSMSMMMASMSMMAMAMMSSSMXMAXAMXMASAMSSMXMSMAMAXMXXASMSXMXXSMMXXAXAXMXXSAXMSAMMAMAAXXAXMMSMSMSMSXMAXXAXMASXXMAXMMSSMMMMMMM
MMMAASMSMSMSASXAAMMMXMAAXAASXSMMAMXSAMXAAMAXSSSSXAMMXAMXMXAAAXAMXSSSSMMAMXMSMMAXMASMMMSAMMAMAXMASMMSSSSMXAXMAAAAAMMSMSMMXSASMSMSXAAAXXAXAASX
XMASASAAMAXXAXMSMSSMMSMSMSMSAXASAMXSMMXMSMMMXMAMMSSSSSMMASMSMSSMXXAAAAMSMSMAAMSMSMXXSASASMMMSMXMMAAAAAAXXSXMSMXMSMAAMXASAMXXAAAASMMSASMXXAMX
MMAXAMMMSMSMAMAXAAAMAMAAAAXXMMXMXSXSAASXMASXAMAMSASAMXAMMMXXXMAMXSMSMMMMAXSMSMAAAXXXMASAMXMAMXMMSSMMSMMMAMAMXMASXMMSSMMMMSAMSMSMAXXXASAMXSSS
XMAMAMMXAAAXXMXSMSSMXMSMSMSSXMASXXASAMXASAMXSXMSMMSMMMXMAXMXMXSMAAAMMSAMSMSXMXSSMMMXMAMAMSMASAMAAAXAXAAXAXAMASXMASAMMXSAMSAMXAMXSAMXSMMMMXAM
XMASAMSSMSMSMSXSAMXMAXAMAMMMAXAAMMMMMMSMMMSMAXSAMXMASAASXXMASAMAMMSMASAMAAMAMXXAXXMMMXXAMAMASMMMSSMXSMSMSMXSASXSAMASMMSAMSAMMAXAXAMMXXXAAMMM
MSMSXSAMAAMMAAAXXAMXMSASXSSSSMSSMSAMAAXAAAXAMMXAMASAMMMMAMSAMAMSXAXMASMSMXMAMSSMMSXASASASMMXMXMXXXMXXXXAXAXMASAMASXMAXSSMMAMMMMMSASAMMMMSMXM
AAMSXSAMSMSMMSSMMMSAMSAMAMXAAAAMASXSMSSSMSSSXSXSMAMAXXAMAMMAMMMXMMSMAMAAAXMAMXAAAMSMMAAMSXMAASMMMMMSXAMAMXMMXMAMAMXSAMXMMSMMASMXSMMMSAASAMAA
SSMSAMXMXAXXAAAAMAMAMMMMAMMMMMMSAMXAMXAXXAAMAAAMMSMMMSMSAMSAMSXXAAAXAMSXMAMSSSSMAXAXMSMXMAMXXXAAMAASMSMMMAAAAMXMMXASASXXMAMXXXMASXAXXMMSASMS
XMMMXMAMMXMMSSSSMASXMAXSASXSSMXMAXAMSMMMMASMXMMMAMSMASMSXMMXSMAXMMSSXXAMSMXMAMXMXSASMMXMXSMSMMSMMMXMAAAASMSSSMSSSMASAMXSXSSSSMMMSMMSMSXSAMMM
XAAAASAXXMXMXAAAMASAMXMSASAAAXSSSMMXMXXAMMMMXMAMSMXMXSAMXXXMAMMXSAAAXSAXAXXMAMMMAMXMAMAMAMAMAAXXSMMMSMSMSAAMAAAAAMAMAMAXAMXXAAXSXMXAAMASXMAS
SSMSMSASMMSMMMMXMSSXMXAMXMMMMMAAASXAMSSMSXAMMSSSMAMMMMAMMXSSXMAAMMXXMMMSSSXSAMAMASAMASMSAMAMSMSASXAXAAAXMMMSMMMSAMXSSMAMSSSSMMMMAMSMSMXMXSAS
MAAXMMMAAASAXSMSSMMMSXAMAMAXSMMSMMSXMAAAAMMSMAMAXXAAASXMMAAMXMMSSSSMSAAXMAASMSXSASASASMMXXSMXAMAMSMMMSMSXSAAAXAAMXAMXXXMAAAAXMASMMMXXMAMXMMS
SMMMXAAMSXSAMXMXAASAXSSMXSASXMXXMAMSMSSMMSAAMAXXMSMSXSMAMMXSAXSXAAAASXMSMMMMMMMSASAMXSAXMSAAMXMAMAXSAMXSAMSSMMXXSSSMMXXASMASMSMSAAXXMSSSXMAS
SXAXSMMMMASMSXSSSMMMXAXAMMXSAXMASMMXMMAAXMASMXMAMSXMAXMAMAAXMMAMMMMMMSAAAAAAMAAMMMMMXSAMXAMXMAXAXSAMXSMMMMAMXSAMAAAMAXXAMXAMMXMSMMMMMMMAAMAM
MSSMMXAAMMMASAAXAAAXMAMXMAMMXMXMAXMAMMSSMXMXASMMMMAMSMSXMMSXMXSAAXMSAMXMAXMXMMSMMAASXMAXXMXASMSMSMMSASAAXMASASASMSMMASMMMMMSMAAXSXMAAASMSMSS
XAXAASMXMXMAMMMSSMMSMXMXAMSMMMAMSMSSXXAMXXXMAMMMSSMMXAXSXMXASAMMMMMMMSSXSMSMSMMXSSXSASXMSXSMXMAMXAXMASXSSSXMASAMAAXMASAMXMAAXASMASAMSASAXAXX
MASMMMSAXAMAMAXXXXXXMSMSXAAXAMXSMMAAXMMSXAAMAMMAAAAAMXMSMASAMAXAAAXMAXXAAAAAASMAXXASAMMSMAXAXSMSSMMMMMAXAMXMAMAMSMSMASAMSXSMSSMMAMSMMMMAMXMX
MMMASASMSXSASMSMSMMMASAMASMSMSXSXMMSMMASXMMMAXMMMXMMSMAXMAMAMMXMMSMMXMMSMSMSMXSMMSMXMSXAMSMMMSXMASAXMMXMMSMMSSMMMXSXXXMASAAXMASMAMXXXAMXMSAS
XMSMMASAMXXAXAAMAMASMMAMXXXAAAAMSMSMAMMSAMASMSSSSMXMAMSSMMSSMAASXXXSSXAXXMAAXAMXMAXASMXASXSXAXASAMSMAMXSASAAAAXAXASXMASAMSMMSXMMSXSASMMMSMAM
MXAAMAMAMSMSMSMSXXMSASXMASMMSMSMAMASMMAXAMAXAAAAMAAXXSXAAAAAMSXSAXAAAMMSAMXMMXMAMMMXSAXMAAXXXSAMXSXXAMSMASMMSMMMMASAMASAXXMASAXXXAXXMAASMMAM
ASXSMMSSMMAAAXMMXSXSAMXMASXAAAAMMSASXMMSXMSSMMMSMSASMMMSMMSSMMMMMMMMMMXSAMAXSXSXSXXSMMMSMMMMXXASMMMSXMAMXMXAXMXAAMMAMASAMXMASASXMMMSXMMSASMS
MAMXAXAMAMXMSMAAXSAMAMXMASMSMSMSXMASMSXSAAAXXAXXAMXXAAAMXMAMXAAASXMXMXXXASMSMAMAAAXMAMASAMXAMSMMXAAMSSXMMXSXMSSSSMSAMXMMAAMAMAMMAXAAAMASXMAM
MXASXMASMAMMMXMSMMMMAMXMASXXAXASXMSMAMAMMMMSSMXMXMXSMMMXAMASMMSXMAMAMSMXMMXAMAMXMXSSSMASAMXMAXAASMSAAMSXMAMAAXAAAMMXMAXAXXXAMXMSSMMXSAAMAMAM
XMMMXAMXXAXXAMMMAXXSASXMASMMXMAMMMMMAMASXAMXMAXMAMXXAAXMXMAXAXXMMAMAMAAMSMXSAMMMMXMAMMXMXSAMSSMMSAMMASASMAMXMMMMSMMSMSXSSSSMSAMMMXSAMMAMXXSS
SXASXMSMSMMMASAMAMXMMMAMASAMXMAMAAASXSASXSXMASASAXSSSMSASMSSSMSASMSMSSSXAAMXAXMASAMAMAAMXMASAAAAXXMSSMASXMMSMMSAMAMSAAAXAAXASMSAAAMXSXMXSAAX
XSAAMAAAAAASMMXSASXMASXMASMMXSASXSXSAMASMXAMSMASASAXXMAMXAXAXXMAMAAAAAAMMXMXMMSSSXSASXMSASXMXSMMSMXXMASXAXAMAAMAXXMMAMXMMMMXMASMMMSASASAAMAM
XMMSMSMSMSMSAAASAMXSXSAMASAMASASAXMMMMAMASXMMMAMMMMXMSMMMSMMSAMXMSMMMXMASASXMAMXMMMMSXASASAAXMAAXMAMXMMSMMSSMMSMMSMXAMAXAAAMMAMAAXMAMXMASXAA
XXAXMMMMXMASMMMMMMASMSXMXSAMXMAMXMMAAMXSMXMASMMXSAXAMAXXAMAXAMMXMXAXXSXASAMXMAXAMAAAMMMMMMMSMSMMSSSMSXAXXAXAXXAXAXAXMMXSSSSXMASMMXSMMSAMAXMS
SMASAAASMMAMAMSAMSMMASMMXSXXXMXMAXSASMAXAMXMXAAASASXSXSMASMMMSMAMSMMAAMMMAXXSXSSSMMMSAXXXAAAMAAMAXAASMSMMSSMMSSSMSSMSMXMMAXXMXXMSASAAXAXMASA
XAXSXMMXAMMSAAXAXAXMXMASAMMMMMAMXXMAMMMMSMSMSAMXSXMAXAXMAMAXXMMXMXXAXMMSSMSMSMAAAASASMSSSMSSMMXMXMMMMAXXMAAAMXMAXAXAXMSAMAMMMMAAMASMMXMMXMAM
MXXMMMXSSMAMMSSXMMMSXSAMASXAAXASAMXSXAMAAAAXMAMAXAMXMSMSXSAMXSMSMMMMXXSAAXXAXMMMMMMASXXAAXXMASXMMSAAMSMSMSSXMASMMSMXMAMXMSMXASXMMXMMMSMSAAAM
XAMXAAXAXMMSAXXAXXAMAMMSXMMSMSSSMSAMXSMMMSMSXMMSSMMAXMASMAXXXXAAAAAMXSMSSMMMMMAAXXMAMMMSMMSMMMAAAMXSSXAAAAMAXXMXAXMASMXSSMMSMSMXMXSXMAASMSSS
XSSSMSSXMAAAMMSMMXXMXMXXXSAAXMASAMMSAXAXXXMXMSMAAASMSMAMAMSXSMSMSMSMMXAMMXAMASMSMMMMXSAMAASMSSSMMSMXMMMMMMSSMSSMXMSXMAASAMXAAMXMAMMASMMMXAXA
MXAAAAMAMXMXXAAXMMSMMXMAMASXSMAMXMSMMSSMMMXMAAMSXMSMAMMMXXXAMAMAXAMASMSMSMMSMSAMAMXMAMASMXSAMAMMMAAAMAAXXXXAAXAMMXAAMMMSAMSMSMAMSXSAMMASMMSX
AMSMMMSXMAMMASASMAAAMAMMXMASAMAMXMXAXAMASMASXMXMAXMAAMMSMSSSMAMXMASAMAMASXAAAMAMMMAMXSXMAXMXMAMASMSXSXSSMMSXSMMMSMSSMSMXXMAMAMASAAMXSSMSAXAA
SXMSMXSXSXSAXXAMMSXXXAXXAXXSXMAMXXSMMASAMSAMAXASAMXSMSXAAAAXMXMXXXMAMSMAMMSMXMSMXSXMXMMSMMMXSAMMSAMASMMAAMMMXAAAXXAAMAMSXXMXSSMMMSMAXAASXMMX
XAAAMMMAMAAXSMMMMAXSSSSSMSMMMXAXSMMMAXMAMMMXAMAMAMXMAXMMMMXMMSXMAXMMMAXAXXAMSAASMAMSAMAMAAXXXXMXMAMAMASXXMAASMMMSMSXSASXMXSAMAXXAXMXMMMMMXMX
MMSMXAMAMAMAMMMMMMMXAAAXAAAASMSXSAAASMSXMASMSSSSSMXMSMAMXSAAAMXMASASAMXMMSAMSAMXMAAAMMSSSSSSSMSMSAMXSMMMMSMXXXAAAAXAMXSXAAMASAMMASASXMMMAASX
MXXASXSASXMXMAMMASMMMMMMSSMMXAXASMMXXAMXMMMMAAXAAMSMASMMASXSAMMMMAMXAXXSAMXMXMXMSSSMXXMAAXMAXXAMSMMMAMAAAAMAMSMSMSMAMAXMSMSAMXSMMSAMAXAMMSMA
XAMXMXAASXAXSAXSAMXAXSXMAMASMMMXMASAMMMSAMAMMSMMMMMXAAXMASAMAMASXAXSXMMMASXMAMAXXXAXMXMMMMMMMMMXXMAXASMSSMSAMXAXAXMMMMSAXMMAMAAXAMAMXSXMXAXM
MXSASAMXMMMMMAMMASMSSMASMSAMAMXAMXMXMAASASMSSMMSMMMMMSMMXMMSXSASXMMSAMXSAMAXSSMMMSMMSMMAMXXAAAMSASXMMMAAAASXSMSMSMAMXMMMSXSAMSSMMSAMXMAMSMMX
XAXXXMMASAAAMXMSAMXMAMAMXMMXAMSSSMMMSMMSAMASAMSAAASXMAMSSMXAAMXSXMASXMMMXSMMMAMAXAMSAMXMSMSSSSSMAMAASMMMMMMMMAAMAMAMAXMAXAMXXMAMXAXSAMAMAAMM
MSSMMAMAMMSXSAXXMMASMMXSAMSXMMAXMAAAAAXMAMXMAMXSMMMAMMXAAMMMSMAMAMMXXXXAMXXMSAMMSAXSASMAAAAAXMAMAMSMMAXXMMMAMSMSMSMSXSMSMMMSMSMMSMMMAXASMSMX
XXAMSAMXSAXAMXXAXSAMXMASASAASMMXXSMSSSMSAMAMAMXAMSSSMMMSXMSAXMXXAMXXMMMSSMSXMSSMMMMXAMMSMSMSMMMSMMXMSMMMSASXMMAAAAXMASAXAMXAXMXAAMSSSMASAMMS
MMSMSAMSAMXAXXXMMMMSXMASMMMMMASXXMAAAAASXMAXAXMAMMAMMSMMAAMMSMMXSMSAASAAAMMAAAXXMASMAMAMXXMAAMXAMMXMAMAXMASMASXSSMSMAMAMXMMXSAMXXSAAXMAMXAAM
XAMAMAMAMSSSMSMXAAAAMMXSAAXASAMMAMMMSMMMASAXSMSAMMXMASASMMMAAAASMASMSAMSMMXMMMXMSAMSAMMXXMASAMSASAXSASMSMAMMAMXXAMXMASMSMMMMXASMMMMMMXXXXMAS
MMSAMSMSMAAAAAXSSMXSASAXXMSXXAAMAMMAMAMSXMMSAASMSXSMASXXXAMXSXMAMAMXXMXAAXXSASAXMAMSXSXMSAMMXMXAXMASASAAXAXMSSSSXMASXMXXAXAMSMMAXAAASMMMXSAM
SAMXMXAAMXSMMMXMAXAXMAMSAMXAMSMSASMASAMXAASMMMMAMAMMXSXMMASXXXSXMASMAXMMSMASAMXMASMSASMMMAMMAMMMMXAMMMXMMMSSXAAMASXSMMAMSSXMAASAMSSSMAMAMXAA
SMXAXMSXSXMAXMXSAMMSXMMMMMMSMMASMSMMSAMMMMMMAMMAMMMMXMAMXAMAMAMMSASAMXXAAMXMMMMMAMAXMMASMSMXAMAAXMXMXSAMAXMMMMMMAMXXAMASAAMMMMMXMAMMXXMXMXAM
SXSAXAXAXMXSAMXXASXXXXMASAAXAMAMAXMASAMXSAMSASXMSAASAMASMAMMMMMAMASXMAMXSMMMSAAXAMAMXSSMAXMSMSSSSMSMXMASXXXMXXAMXSXMXSMSMSMXSASXMSSMAMSASAMX
SASMSSMAMMAMAXMXMMMSMXMMXMSSMMSSMSMASAMXMAMSMSAAMXMSAMAXMAMAAXMSSXMXMXSAAAAASXSXMSAXAXAMAMMAAAAXAAAMXSAMXMXMASXSAXMAMMAXXMAXMAXMAMXMAMMASMMM
MAMAXXXMXMASXMAMXAAAAAMXSMAXXMXMXSMAXMAMXMAMMSAMMSXSAMMSSMSSSSMAMAMXMAMAMMMXSXMAMXMSMSSMMSSMSMMMMSMSAAAMXXAMAMAMASMSAMXSMMMMMMMMAMXSSSMXMASA
MAMXMSMSAMXXAAXASMSSSMSAAMAMMXAMSMAMXSASAXAXAMXMMXASXMSAXMAMAAMSMMMSMSSXXASASAMMSAMAAAMAMXXMAAAXMAMMMMXMASAMASXSASAAXXAXXAAAAAXSAMXAMMXSXMAS
SASMAAAXAMASXSXMAAXAMAMXSMMSMSMSMMAMASASASMSSMASXMMMAMMSSMMSSMMXAXAXAMAMSAMASXMASXSMMMXSMMSSSSMMSXSAMXSXXAASASXMASXMSMMMSXSXSSMSMSAMXAASAMXM
SASMMMSSSMAMSAASMSMMMSMMMAXAXMSAAMMSXMAMAMMAAXAXAXMXMMAXXXAAAXASMMMSMMAMMMMAMMMASXMASAMXAMAXAXAASMXMXXMAXSXMASAMXMMXAAMXMAMXAMXXMAMXMASMMMXS
MXMASXXAAMXMMSMMAAAXAXXAMMMSAMSSSMAMXMXMXMMSMMSSSMSSSMXSASMSSMXSAAXAXMASAMMXSAMAMAMXMAXSAMMAMXMASASMMSASMMMMMSAMSMMSSSMAMAMAMXMAXAMAMXXMASAX
SXAASMMSMMXAMAXMXMMMMXMSMSAXMAXAMMAMSSMSXSXXXAAAAAXAMAMMASXAMMASMMMXSAMMMSMASMSMXSMSAMXSMMXMMAMAMXMAASAMAXSAASXMMAAMMMXASASAXAXMSMSMAAMSXMAS
SAMXSAXXMXXSXSAAASXSMSMXAMASXXMAMSAMAAAXXMASMMSSMXMMMSMMAMAMSMASXSAMAAXXAAMXMMSXAXASMSASASAMXSSMXSMMMMAMSMSXXSSMSMMSMXSMSMSMSMXAAAAMAXXAXAAA
SASASXXSASXMAMMSXSAMAAAMAMMMMXSSMMMSSXMMXMXMMAAXMXMXAXXMXXMMAMXSAXMAMMMMSMSASAXSSMAMAMXSAMASAMAXMAMXXSSMMAMMMXMMAMMMMXSAMXMMAMXMMSMSASXMSSSM
SAMASAMXMMAMAMMMASAMSMSSMMSAMXMASMMXMASMXMASMMSSMAMMMSSMMSASXSXMXMASMSAAAAMAMAXSXMXMXMASMSXMASAMSMMXMMMAMXMAMAAXAMAASXSXSMSMMMSMMAXMASXXAXAX
MAMAMMMMXMAMXSASASAMAXAAMAMXXAMAMXSASMXSASASAAXXXXSAMXAAAMAMASXMXSAMASMSMXMSMMMSAMMMMSMXMAXSMMMMAXMAMAMXMASXSXSXSXMXMAXMSAMXSASASMSMMMXMXMMM
SAMSSXSXMXMXAMXMMMAMMMSSMSSSMSMMSMXASMASXMASMMSAMXSXSSMMMSAMXMASAMAMXMAXMSAXAMASMMAAMXXSXSXSAAXXSXMXMASASAMMAXXAXMSMMSMAMAMXMASAMXMASASXSSXA
XMXXAAMAMASXXMAMXMAMXAMAAXAXAAAMXAMSMMAMAMXXAAMAMAXXAMXAXSXSXSAMXSAMXMXAAMXSXMAXASXSSXXAAMASMMSMAAMXSASASXMSMMMAMAAAAAAXMMMMMMMMMMSXMASAAXSS
XSSMMMMXMASAMXAMSSMMMXSMMMMMSSXMMSMMAMMSSMASMMSMAMSSMMXMASXMXMAXAXAXXXSMXXAXXASXMSAMAMMMXMAMAXAMSMMAMMMAMMXAAMMSMSMSMMSSSXMXASXMSAXXMXMMSAMX
MMAXXSXMMMSAMSXMAAAMAMXXAAMXMXAAAXASAMXAAAXXSAMXASAAASXSAXAXMXXMMXSMMMXAAXXSSMXAAMXMAMAAXMAMASMXMMMXSAMSMSSSSMAAMAAAXXAXXAMSMSAMMXXAAXXSXMXS
ASAMXMSMAAXAMAXMSSMMSMMSSMXAMMAMMSXSMSMSXSAMMMSMMMXSMAAXAXSMMASMMAXASMSMXSAAAXSMMMXXMSMSXSAMXMMAAMSMSXXMAMXAAMSSSMSMAMXMSAMAMSXMAMSXMAMAAXAX
MMASAAASMSSSMMSMMAMAAAAAAMSASMXSASMMAXAXAAMSAXAMXAAMMMMMSMMAMAAAMMSAMAAAAXMMMMSAMXMAMAXMASXSAXSMSMAMMMXMXMSSSMMXMAMMMMSMSMSMMMAMXMAXMAASMMMS
XSAMMXMMXXMXAMAMSMMXSMMSAMXAMXAMAMAXXMSMMSXSXSMAMMSXMAAAXASXMSSSMMMXMXMSMASASMSAMMAMSASMXSMMAXSAXMSMXAASMMAXAAXXMAMSXAAAXXAXASAMXMXXSAXAMAMA
XMMSMXMXXMMSSMAXAXMXAXAMASXMMMXMMMSMSMXAXMAMXXXAXMAAAMMSSMMAAAAAXAMMSAMAMXSASASMMMSAXAXXASAMXMMASAMXMXASMMAMXMMXMAXXMSMSMMMSMXASMSAASMSMMAXS
AMAAMSMSMAAAASASXSMMMMASAMAMASASXMAAAXXAMMSMSMSSSMSSMMAAAMSXMMSMMMSAMMXASMMAMAMXAMAMMMMMXSMMSSMMMAMASMASMSMSSSMXMSSSXAMMMSMAAMSMAMMMMAAAXSSX
MMMMXAAXSAMMXAXXMMAAXMAMAMMSAXXSASMSMMMMXXXAXAAAAMAAAMASXMAAMAXAMAAMSSSMSAMMMXMXMMXXAAAXMMXAMAAXMXMAXMXMAAAXMASMAAAMSMSAAAAXXXXASXSXMSMSMMAX
XAXMSMSMXXMXAMSSMSSMSMXSMMXMMMMSXMXMMSSSSMMSMMMSMMSSMMAXMMMMMASAMXSMSAAASMMXSMSMSAASXSSMMSMSSSMMSAMXMMAMXMSMSAMXMMXMAMXMSSSSMAMAMXMAAAAAAMSM
SASXAAAAMSMSMXXAAMXMAXXMASXAAXXXMSXSAAAMAXAAAXMMXAMMAMAXXAAAMXSXMAXXMMMMMSMAMAAAAXMXMAXXAXAXAXXMASMASMSXXMAAMASMXXXSASMAAXAAAAMAMASAMXSSSMMS
MAMXSSMSMSAAXMSMMMSMMSMMMMMSMMMAMXAMMMSXXMSSXSASMSSSSMMSSXSMSASAMXXAMXAXXXMAMSMSMXMXMAMMMMSMMMSSXAXXSAAASMMXSAMASXMSASMMMMSAMMMSSXMASXMXAXAA
MAMAXMAMAMMMMAMXMASASMXAAAAAASMSMMMMXAMMMXMAMSAMAXAAAXMAMXMXMXSAMMMMMAXMAXSAMXAXXXMASAMXAAXAXAMXSMMMMMMMXAAAMASAXSAMMMMAXMXMASXAAXMAMAMMMMMS
SAMXSSMMSMSXSMAMMASXMASMSMSSSMAAXAXXSAMXMAMXMMAMMMMMMXMASXMASAMAMXASAMSASMSXSAXXMXMAXXSSSSSMSASAMXAMXMXMSMMXMAMMXASXXASMMMAXAMMMMMMSSXXAMAXA
SXMXMMMAAXAAASXXMAXMXMAMAMAMXMXMASXMMXMASASMMSSMXAMXXAMAXAMXMASXSSMXASXMMAMXSAXMSSMSSMXAAAAAMMMAMSMSXXAMSXXMMMSXMAMMSMSASXMMXSAAXXXMAMSASXSM
SASAXMMSMSMAMAXMMSMMASMSXMSSSSSXMAAXSMSASMSAMXAXSSSSXXMSSMMMAXMAAASMSMMAMSMAMMMMAAMXAMMMMMMMMSXSMSMAMXMMMXMXXSAXMAMXAMXMMAXAXSXSXSXMAMSAMXXX
SASXSXXXXAMSXSAAAAAXMSASMMXAAAAMXSMMSMMMMXXAMSXMAMAMMSAMXAXASMMMSMMXAAMXMAMMSSSMSSMSSMAMASAMXMAXAMXAAASASXSASMAMSSMSSSMXMMMSXMAXAMSMXMMMMMAA
MMMMAMAXSSMXAXMMMSSMASAMAAMMMMXXAMMAXMASMXSAMXSMXMXMXXMASXMSAXXAXAXAMMMXSMSXAAMAAAMAAAXSASXSAMAMAMSSSMSAMAMXXMSMXAAAAMXSMMAMAMAMAMAXAMAAASAM
XMAMAMMXXMAMMMSSMAMMXMAMXXXXAXMMSXMXMSASAXSAXAMMAMAMXMXAMAXMASMMSAMXMAMXSAMMMSMSMMASMMMMXSMXAMSXSMAXMXMXMSMSMAMMSMMMXSASAAMSXMMSXMXSMSASMSAA
AMASXSXMXMAMMMMAMMSXAXXMMSSSXMASXSMSXAXMAMSXMAMMAMASAMMSSXXXAXAXAMAMXASAMASXAAXAMXMXXMXXAMXSSMXAXMMAMMSSSMAMMMMMAXXXMMAMXMXSXAXSMSAMXAMMXMMM
SSMSAMMAASASAAMSMXAXMSAMAMAAMSMMMAXMASXSSMSMSAXSMSASMSAMMXSMXSSSMMAMSMMMSAMMMSXMXAXSASMMAXAXAAMMXAMMMAAMAMXMAAASAXSSSMSMAMAMMSMMAMXMSSSSSSSS
MAMMAMAMXMXSXXSAAXMAMAMMASMMMMAAXXSMAMXMAASAMXMAMSAMXMASMAMAAXMAAMXMAXAAMMMAAXXXSMMSAMXSXMMSMMAMMXSXMMSSSMASMSXSAXSAMAMSMMASAXAMXMAXAAAAAAAX
SAMSXMSSSMASAMMMSSXMSSSSXSAXASMSMAMMASXMMMMMMSSMMXSSXXSMMXSSMMSSMMMMASMSSSMMXSAXXSAMAMASXAXXXSASMMSAMXMAAMAXXXMMXMMAMAMAXSXMMSAMASXSMMMMMMMM
SAMSMMMAAMAMAMAXAXAAAAAMXSMMXAAAMAMSASMSMXMXAXAAAAXSAXXAMXAAAAMAMXXSAMAMAMAMAXMMAAXAXMASXSMAXSXXAXSAMMMXMMMSAMSMMASXMXMMMSAAAMAMASMAXXXMASMM
SAMXAAMSMMXXXMXMMSMMMMMMMMXMMMSMMMMMASAAXSAMXMXMMXXMXMSAMMSMMMSAMAXMMMAMMSAMSMXMXMXMXMXXMXMMMMMSMMSAMSSMXAXMXAAAXXMSMXAXASMMMSSMMSXMMMXMAMAX
SAMSSMXMMSMSMXXSMSASXSAXASAXAAAAAXMMMMMMSAMXSASXSMXMSASMMAXAMXMMMXSASMSSXSXSAAMMMXAMMMMXSMXAAAAXXAMSMAAASMXMMSSSMSMAMSMMASXMMXXAASAMAXMMMSSM
SAMXXXAXXAAAXMASAMMSASXMXSASMSXSMMXAXMAMMXXMMAAAAAAXMXSXSXMSXMMXMASAMAAMASMSASMAASMMAXAMXAMSSSSMMMMAMXXMXMASAAMMAASAMAAAAXAXMASMMXXMMMMMXAMA
SMMMASXMSMSMSMXMAMXMAMASXXMMAMAMAMSSMSASMASAMMMSMSAMXAXAMSAXAMSASXMMMXMMAMAXXMXMXMASMMSAMSMMAAXMASXMSMSMMSASMSSMSMSXMMSMSSSMMASMMSMSXSASMMXM
SASMAMAAAAAMAMMXAMXMAMAMMAAMXMASAMSAASASMMMAXMAXAXMASMMMMAXMAMSASXAMXSAMASMMMMAMSMAMXAMSAMXXMXMMASAXAAAAMMMSXMXMXXSMSMAXXAAMMAXAXAAMASASXSAS
SAXMAXMXMSMSASXXSSMMSMSSSMMSXMAXAMMMMMXMXXXXMMXMAMXMAXAXSASXMMMAMXMMAXAXAXAXXXAXXMXMMMMXMMMMMAMMAXXMSSMMMMASASMSMXXAAXAMMMMXMXSSSMSMAMAMASAS
MXMSMXSXMAAMASMMAAMAMAAAAMASMMSSXMAAAMSAMXSMSSMXMASXMMSAMAMXAAMASMSMXSAMSSSMSSMSSMMSSSMSSMSAXAMMXXSXMMMXXMASXMAASXMSMMAMXAMXSXAAXAAMXSAMAMAM
SSMXXASASMSMMMAXXXMAXMMMMMXSAXMAMSSSSMSMSMSASXAASASAXAAMMXMMXXMMXAAAMMXMXAAXAAAAXXAAXAXAAAMMXMXAASMSXAAMMMMXAMXMMXMASAAXSASAMMMMMSMSMSMSXMXM
XAMSMMSAMAAMXXSXAMSSSSSSSXXMAMMMMXAXMASASAMMMMSMSAMAMSSXSSSMMSSMMSMSMMASMSMMMSMMSMMSSSMMMMMSASMSMAAMMSMMAAMXSMSSMASASMMMMXMMMAAAXAAAXXAXMASM
SSMSAXMMMMMMXMMSSXAAXAAAMXMASMSSMMSMMMMAMSMXAAAMMAMXAXAAXXAAASAMXXAMAXAMMAMAAXAMXMSMXXASAMXMASAAXMAMMAAMSXSAXXAAXAMASASASMXSSSSXSMSMSMMMMAMM
MMASMMASXSAMXAAAMMMMMMMMMSAAXXAAXAAAAMMMMMMMMMSXSXMASXMSMSSMMMAMMMXMAMXXSAMMMXAMAMXAASMMMAMMXMASMSSMMMSMAAMMSMSMMXMXSAMASAAXMAXAAAAXSAAMXMAS
XMXXSSXSMSASMMMMSAMXAXMMASMSXMMMMXXXMSASASXSMAMMAAXAXXAXAAMASXMMMXAMSSMASXSAMMSMMSSMMMXMXAXMXXXXAAAASAMMMSMMXAXMAASXMMMSMMMSAAMSMMMSXSXSXSAM
XMSAXXXMASAMXAAAMMXSSXXMASMMASAXXSSMASASAXAAXXXASMMMMMXMMMSMMASMMMMMAAMAMMMASAXAAAXAAXSSSSSMSMSMMMSMMASXAMASMAMMXMXAAAXMASAXXAMXMXXSMMMMMMAS
XXMXMASMXMAMSMMSMMMSMXSMMSAMAMMSAAAMXMAMAMSMMMSMAAASMMAMMMXMSMMASXMMMSMASXSAMXSMMSSXMSXAAXAAAXAAAXMXMXMMMSAMMMMSAMSSMMXSAMXSAMXSMXMXAMXMAMMM
SMSMSAMXXXAMMXAXMAMXAASXMMMMXMASMSMMAMAMAMAAXAAXMXSMASXSAMAAAMMMXAXMXAMXSAMASXAAXAMXSMMMMMMXMSSSMSMAMSXXAMXMXMAXAXAXAXXMAMAXASAAMAMSSMMMMSAX
AAAMMSSSMSMSXMSSMMMMMMMAXAXMXMASXXXSSSSSXSSSMSSSXSXXAMMSASMSMMAXMMMSAAMSMAMASMSMMMMMXAAMASXAXMAMAXMAMXAMXSSSSMMSSMMXMMXSXMSSMMMMSAXAAAAAXAMX
MSMSAAMAAAAXMXXAMSASXSSMMSMMAMASXMASXAMMMMMMMAAAXAMMMSMSASAAASMSMSASMXMASXMAXMAXXSMASXMSASAXXAXMMMSMSMAMAAAXAAMAXAXAMSMMAAAAXXSASXMMMMMXXAXX
MMAMXXMMSMSMMMMSMSASAAAXAAAXXSAMAMXMMMMMAAXAMXSSMMAAAAXMMMMMMMMAXMMSXMSXSMMSSMSMMAMXSAAMAXMXSSMSXAMXMAAMXMMMMMMMSMSXSAASMMSXMXMASAMSXSAXXMMM
SMMMSMMXXMAAXAAAMMAMMMMMMSSMASAMXMASAMASXXSSSMAMAXSMSSMSXXXXAAMXMMASXAXAMXMMXMAASAMMXMMMMSMXAMAXMMSAMXSMXAXXXAXXSMAMXXMMAXMASXMAMAMAAMMXAAAA
SAAAAASASXSXSMSXSMSMAXXXAAAMAMAXSSXMSMMXSAAAMAMSXMAAXMMMAMXMSMSMMSAMMMMXMAMSAMSMMASMXSXMXAMXMMMMAXMASAAMSMMMSSSMXMASMSMSAMSAMXMAXAMMSMASASMS
SSMSSMMASXMASMMMXAAAXAMMMSSMAXAMMMSXXXXAMMMMMSMSMSXSMSASAMSXXAAAMMASAXAASXMMAMMXMAXAMAAMSMMAXAXXMXXAMMXMAAXXAAAXAMXMASMXMAMXSSSSSMSXAXAAXXMM
MAMAAXMXMAMAMAXSMSMSMAAAAAAXSSSXXAMMXXMSXSAMMAMMASAAASXXASXAMSMMMSAMMMSXMXSSSMSSMMSSSMXMAXSXSSMMSSMSSSSSSSSMMSXMSXSMAMXSAMSMMXAXMASXMMXSMSSS
MAMMAMMSMSMSMMMMXMXXAXXMSSSMXAMSMASMMMMXAXSXSASMAMMMMMSSMXMSMMAAMMMSXMMAXAXAMXAAAXAMAXXMSXMXMAMMAAXAAXXAXAAAAXAMXAXMXMAMAMXAXMSMXASMSMAMAAAA
SMMXMAXAAMAXMXSMAXMXSMSMAXAXMXMXSAAXMAXMSMMMSASMSSSMSAAMSXXAAXMASAMXASMXMSXMAMXSMMASMMMMMASMSSMMXSMMSMMMMSMMXSAMMSMAMXMSAMXMMAMAMXSXAMAMMMMM
AMAAXSSMXMSMMASMMSXMAAAXMSSMAMMMMSSXSXSAMASAMAMMAMAAMMSXSXSMSMXASASXMMAXSAMSASXXAXXMASMASMMAAMASAAAXMAAAAXXXASXMAAMMMAMAMSMMMSXMAXXMMSMSSMSX
MMMSMMAMAMMAMAXAXSAAMSMSXAAAMAAXAXMMAMMXSAMMSAMMASMXMXMAMMMMMXMASAMAXMXMMAXSASASMMSMMASASAMMMSAMASXMSMMXSMMMASXMXSSXMXSAAAASAMAMSMAMXSXAAAAX
SSMMMSAMSMSAMMSMMMXSAMXXAMXSSSMMMSSXMASXMXMXMXXSXSMSXAMAMAMASAMXSASXMMMSXMMMXMAMMAAAXMMXXAMXXMAXXAMASAMAXASMSMASAMXMMMSMSMSMAMXMAAXMASMSMMMM
SAAXMMAMAASXMAAMASAMASAMSSXMXAAAAXAXXMASMAMSMSXSXMAMMAXMXXXAMAMXSAMMXXASASXSMMXMMSXSMXXMSMMMSSSMMAMASAMASXXXMSAMASAMAAXXAXMXXMSMMSSMXSAMXMAS
SMMMMSXSMXMMMSASAMXSAMAMAAAMSSSMMSMMMMMMSMXAAMASXMAMSMSXSSMMSAMMMMMMMSMSAMAMAMAMXAMXAMXMAMXAAAXXSAMMSAMAMAXMMMMSAMXMMSSSSSSMSXAAXAXXAMMMAMXM
MAAAASMXXSAMXAAXMAXMXSAMXSMMAXMAXAXAASAAAXSMAMAMASXXSASAAMAMSXMXAXMAAXAMMMAMSMASMASMAXXSASAMMSMMSXSMSXMASXAMAAMMASMSXMAXAAAAXSSSMXSMXSXSXMAX
SMSMSSMXMASASMSMMXXMASMMXXAXXSSMSASMSSMSSMSSXMASMMXAMAMMMMAMMMSSXMXMSSXMXMASMMXSXXMMXMASXSXAXAMMXMAXMASXXAXXSSSMXXMASAMXMSMMMAXXMASMSAMXASMS";
