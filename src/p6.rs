use std::cmp::{
    Ordering,
    PartialEq,
    Reverse,
};
use std::collections::{
    BinaryHeap,
    HashMap,
    HashSet,
};

fn find_links<'a>(key: &'a str, direct: &'a HashMap<&'a str, Vec<&'a str>>,
         all_links: &mut HashMap<&'a str, Vec<&'a str>>) -> Vec<&'a str> {
    if let Some(o) = all_links.get(key) {
        return o.clone();
    }

    let mut links = vec![];
    if let Some(conn) = direct.get(key) {
        for d in conn {
            links.push(*d);
            links.append(&mut find_links(d, direct, all_links));
        }
    } else {
        eprintln!("Key not found for {key}");
    }

    all_links.insert(key, links.clone());

    links
}

pub fn p1(input: &str) -> usize {
    let mut total = 0;
    let mut direct: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut all_links: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let (l, r) = line.split_once(')').unwrap();

        match direct.get_mut(r) {
            Some(links) => {
                links.push(l);
            },
            None => {
                direct.insert(r, vec![l]);
            },
        }
    }

    for k in direct.keys() {
        let links = find_links(k, &direct, &mut all_links);
        total += links.len();
    }

    println!("Total = {total}");
    total
}

/*
 *
 * Another approach:
 *   - Find all links from start and save number of steps to reach each of them
 *   - Find all links from end and save number of steps to reach each of them
 *   - Intersect and return minimum sum.
 *
 */

#[derive(Eq)]
struct State<'a> {
    steps: usize,
    obj: &'a str,
    visited: HashSet<&'a str>
}

impl<'a> State<'a> {
    fn new(start_point: &'a str) -> Self {
        Self {
            steps: 0,
            obj: start_point,
            visited: HashSet::from([start_point]),
        }
    }

    fn move_to(&self, dest: &'a str) -> Self {
        let mut visited = self.visited.clone();
        visited.insert(dest);
        Self {
            steps: self.steps + 1,
            obj: dest,
            visited,
        }
    }
}
impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.steps.cmp(&other.steps)
    }
}

impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for State<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj && self.steps == other.steps
    }
}

pub fn p2(input: &str) -> usize {
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut start_point = "";
    let mut end_point = "";

    for line in input.lines() {
        let (l, r) = line.split_once(')').unwrap();
        if r == "YOU" { start_point = l; }
        if r == "SAN" { end_point = l; }

        edges.entry(r).and_modify(|v| v.push(l)).or_insert(vec![l]);
        edges.entry(l).and_modify(|v| v.push(r)).or_insert(vec![r]);
    }

    let mut heap = BinaryHeap::new();
    heap.push(Reverse(State::new(start_point)));

    while let Some(obj) = heap.pop() {
        let obj = obj.0;

        if obj.obj == end_point {
            dbg!(obj.steps);
            return obj.steps;
        }

        if let Some(links) = edges.get(obj.obj) {
            for l in links {
                if !obj.visited.contains(l) {
                    heap.push(Reverse(obj.move_to(l)));
                }
            }
        }
    }

    panic!("failed to find SAN");
}


pub static IN: &'static str = "WGB)S14
WN4)27C
18L)M18
1HY)6ZP
TQ9)KQ6
HQ3)HH1
FLC)F1Z
D6R)ZPC
2VD)GK3
YY3)3TP
PBL)3CK
5K4)CB5
V5M)CNN
L4T)RHS
HHH)66F
Q3Y)DTL
DGN)YY3
CCT)L3B
Z6X)FM2
2QQ)VK9
MX3)C9J
4JK)BPX
8BP)N13
PBW)6Z6
2LT)DT9
JHX)GXM
5LW)BHQ
DNK)ZBT
29Z)T9D
WNP)TDC
S38)GL6
DW9)V2F
4MG)3FW
Z9Z)CPK
FKL)QNH
55D)HT2
D1D)N4Q
Y7W)1Y8
SFQ)79W
JSR)62W
4WN)J18
VK9)J2H
LS5)DCX
6LR)P4X
HDV)DGQ
1K9)KD1
2PX)17C
KSB)GL8
B4S)VTV
ZW1)KNR
BVH)43P
VKP)6L3
P5K)MHR
XHR)STT
WBG)5X5
HZF)8JQ
B47)NW4
J5V)3ZW
KGP)VVR
24K)PK8
31V)LXC
5XG)RHP
P1G)HN8
R76)3GY
5CH)17Q
TVC)XJM
598)RD3
J66)LKC
4DY)YSQ
M4Y)NLL
SMP)M2M
TBR)WNP
K22)KGP
MQ5)8MN
B9Q)6HQ
P9S)X92
TJK)ZQK
XS7)7KL
H6J)DX1
MTP)3Z6
B17)B7P
S12)PC2
47V)5KW
KCY)HWP
FB2)S38
V5M)FNT
GXM)QPR
HXR)2R2
2LV)NDP
6HQ)12S
22P)4HL
T8Q)9FB
8YW)TVZ
DR1)NNN
9TH)87Z
79W)TM2
5GB)HQM
1HY)4WN
LFV)RYJ
YCN)ZMK
8SR)SB3
P9H)PH9
ZGQ)T3J
KWW)1HY
TLF)RPG
PFD)HZR
9SF)7PY
DCX)VCC
D1R)2RB
GXC)NN9
ZZW)SCC
G44)Q8D
923)3J2
KY2)8F4
1XQ)7LD
GHX)Q6M
TZ5)V32
LM9)1XK
Q7N)Z7H
YKD)73H
9RZ)C2Q
5KN)P1G
3FJ)L73
ZPC)VYT
Y7D)FFY
C8W)J1Y
X5T)55D
Z3F)GK8
WRS)PRR
T9M)JK2
81P)5WT
7KL)5BK
S3R)VCD
56L)D1R
PR2)92L
91F)2F4
ND4)PJ6
9KY)YD4
CLH)5D7
J2F)L7Z
M4Y)PYG
891)P34
VV6)18L
RQQ)X8P
7SR)8G6
WJ8)CDL
9FB)TXD
RKK)2H5
3W8)2QQ
27C)YFC
RZZ)91F
4CP)BWH
T4L)LS5
788)G7S
47V)3W8
FGK)719
16Q)4KW
5H6)PC9
KGS)TBR
44Y)BVH
GMF)VFM
LKC)PM4
DPL)DXS
2WD)X5T
XWX)NYR
N44)Y36
72S)56L
W25)4MG
P9S)HBC
W84)3YP
NW4)S78
58Y)LDB
QJ9)VV1
5D3)4Q7
T3J)5M4
394)ZW7
JXL)QVK
7KL)FTL
885)ZGQ
58Y)8SR
GXN)PBW
HH1)JB4
H6J)W95
VYK)SQS
CCS)7CZ
PJ7)NLR
2VW)MSP
ZWK)H6X
HJ4)C1S
H41)1L3
8B9)64N
RZR)WBJ
FNT)VL7
K5M)S5Q
XJM)TC7
QWT)7Q5
43P)MY2
YP7)51N
TDX)FWZ
DB3)NCK
37M)H2L
Z3X)XRS
SGV)R2T
Y2F)63M
ZVY)JTX
DJB)KQD
848)FQP
SX3)FM5
PH9)ZPG
75S)Z9L
GPD)Y7D
9Y6)52N
SL4)S3R
4TH)T6F
K4V)D8V
89S)18F
GDN)WN4
6HT)TQZ
V1Q)JVG
R55)2LC
KH3)NT5
Q53)3DN
SRV)JND
XMC)MKK
T5J)6HT
HZR)M1M
P34)3RY
HF6)SD2
PTM)C9X
3MZ)T9M
R76)MFF
B9Y)3MC
NFG)5FC
M63)4CP
FRG)PVQ
58Z)GDM
ZT8)4L5
F5B)KF3
SQT)NTZ
M2M)252
35Y)5WF
C9J)8BP
W8H)F78
H8Z)2VW
91P)5FP
VTV)YN7
2KM)1K9
KSX)TR8
9DK)XD6
MFF)KQR
414)6L9
FQ7)T5K
G8M)WPX
794)FMS
WZV)XS7
VVR)5CH
R8G)9RH
B4D)2RT
PJ6)GWB
63M)NHF
8G7)8B9
QP5)9ZW
FW6)CDM
S5Q)172
T24)TPD
YRT)GMF
1TJ)SGV
RV3)C3D
661)MHS
QYT)D2K
T49)MFP
GY7)T2Q
686)Z4G
J49)R9L
R5S)67X
L7Z)5RM
RPP)WG9
5KW)2KM
5N2)Y7W
Z3X)JFD
KD8)4H5
5MP)RVK
12S)S2Y
TPD)D5F
51N)81P
DCH)SGQ
L6N)VKP
2XQ)6LR
3DN)S3L
VS4)83N
8DJ)WZP
DCX)FX1
SF9)Z3F
R49)S99
D1C)794
TKN)L83
21R)GP3
5RM)TG3
ZMK)R49
1QT)152
9DX)GXC
GYC)TQ9
JND)LMK
D8Z)SCW
VNZ)VS4
C1S)9RZ
LKF)D8Z
G4J)R44
92L)J66
88P)657
8Z5)R55
VV1)KRY
N44)2QK
KBC)KKG
91P)L6N
SVH)7W3
P9Z)34H
BWH)9TH
JNX)RZZ
YFG)ZT8
DSM)FF3
BMK)ZR6
7W3)V82
T9D)H2S
2QF)PFD
NDQ)F13
ZVB)MX6
KRY)7FB
KKG)HJ4
QNH)MFQ
5X5)VQM
HQM)HF6
HLT)TD2
WV4)FWH
N2T)5B5
D1R)P89
HKT)3MZ
ZQK)1DK
QQQ)FLC
73Z)TTM
ZZW)769
8G7)TYL
MFP)WMS
RQS)2YC
NLL)JHX
KCY)CSP
9F8)51H
SGQ)B27
4KM)VYK
JDY)MTW
T8Q)DB3
1VL)VV6
VV5)B4D
SPF)JR5
LYS)6CK
YMK)2VD
TD2)1VL
JKH)QHX
VD4)58J
9QQ)HKL
8JP)HQ3
NHS)31S
81Z)Q5W
R7Q)Z9M
WMS)ZK2
3J2)GY7
MFQ)CLH
S14)934
HY7)YBT
4SY)63F
NQF)PPQ
T9W)RZR
WL2)6QM
LZV)WRQ
TVZ)T9P
4X5)GN5
NQ8)FPQ
J5J)K51
Y8T)WGM
FPQ)B53
1XK)TKX
XDW)72V
WW8)9QQ
XX7)Q7N
CDM)GHX
VCC)HPP
QRK)56B
MTW)2QT
7V5)58Z
PYY)T24
9HB)J8F
TTM)PTB
FF3)ZY3
ZW7)D1D
T4H)ZTG
2PW)DSM
9WB)4TH
17C)FKX
T6F)QP3
G6R)XHR
H5T)QYT
DX1)Q9L
GJF)ZF3
LJP)JXL
QHX)3XY
DNF)8KQ
8Q1)NDQ
GP3)6MY
FPQ)QQQ
XRS)923
Q6M)7BS
K21)B47
TQZ)WJ4
9PB)3PQ
8G6)X7M
L3B)YOU
L5V)G2L
B8Y)JVS
GL6)MTP
9QZ)NRN
486)T8Q
HNN)PNM
NFK)B4S
G9C)LHT
4K9)SL4
8X5)179
VQM)47V
CNJ)J4R
ZD5)2PX
9TQ)X9Q
Q3Q)9DK
17Q)1KX
5GN)24K
K5Q)1NF
LCK)9WB
TYL)PYL
7XG)R2L
LXC)ZWK
Q62)SPF
89C)N7Z
GK8)GR7
6X1)5N2
XM8)8Q1
MCD)GXQ
S2Y)N7G
CB5)C8W
NHF)44B
QPR)GF5
HGX)YMY
3FW)2LV
5WF)RQQ
841)N31
Q9L)876
WQ8)HZ7
6K1)QVC
C2T)FQX
J3M)HLT
H2S)K5Q
STQ)8ZF
VDX)NQF
YSR)G8M
CSL)NLF
MHS)3FH
YN7)VWC
RSW)X11
FXS)L54
YBT)HX4
BHQ)FRG
83H)K4Q
NT5)2ZB
GWB)4K9
YMY)5KN
4Q7)C3G
D3J)HZF
32D)GN7
VGG)G3D
LVG)JXR
25V)GDN
L6V)KL8
FW2)STQ
V6H)8G7
COM)CB6
6Z6)SQT
W81)6M3
D2K)XF6
2NX)9KY
KRQ)LKF
P1B)VVQ
QTV)Q3Y
DTZ)ZLY
R3T)FQ7
D92)72S
H8N)9Y6
FWZ)WGB
VQW)LHJ
2HB)848
9ZW)NT8
NLR)QTV
31V)DSZ
92J)WXY
8LK)QQ3
769)ZD5
8L9)T5J
TB2)V5M
VZQ)57T
Z7H)JMR
94D)YCN
ZPF)6WK
M1F)6C2
MHV)ZCS
Q53)FBC
RPG)P3N
RHS)JDY
FTL)FB2
J47)R3T
Y9S)4JK
ZVY)TK6
LTX)BM5
D8V)3R6
J18)S19
PVQ)WL2
ZPV)QFW
719)CSV
XK9)9Q9
BM5)L5V
LDF)WZT
MSP)DTZ
HRQ)JBG
19C)GSP
GPB)HGX
2F4)8HL
886)C8J
ZF3)LM9
NQ1)394
WM9)M79
PM4)GNT
6J4)ML2
5WT)HQS
KQD)5K4
JBT)V1Q
JVS)DDT
3G2)52L
8ZF)D6R
4BQ)5H6
G7T)7T6
ZY3)83H
HYC)G9C
MX6)XC9
2NW)2SH
YXJ)JSR
QNH)YJN
TG3)886
N7Z)G98
5D7)KNW
8TN)KH3
C78)TSW
87Z)DFM
QGG)Q53
NRN)YP7
TTB)C2T
ZLY)25P
7KS)D5X
LNX)CNJ
QVK)YMK
CNN)N43
5Q9)MWG
SCC)XFV
885)G7T
4BS)4BQ
N4Q)Z6X
FQX)7W4
MLL)NF8
52N)PZ2
DNF)KBC
6C2)CCS
LZ8)P1B
CSV)686
PZ2)KKC
JMR)327
3TP)N6L
3W8)YFG
S62)J5V
FF3)VXL
4X4)MHQ
3TP)7Z7
L83)VDN
Q8D)2HB
JB4)5LR
VYT)SAN
L54)X63
15J)XF2
FWZ)WLR
R44)K5M
TK6)Q5J
J81)QP5
114)BGP
QQ3)PJ7
D5F)HNN
MFF)WW8
J18)MP3
9JN)M8G
2YC)CSL
R2T)4TS
ZBT)WQ8
XFV)MPD
R9S)XDW
8HL)99X
4MG)2QF
8X5)BMK
CN7)KSB
YJN)44Y
X11)WWY
5MP)VDX
R2L)PFY
6ZP)HPW
WGM)GPB
WCZ)KCY
NYR)TKN
1L3)SKC
MND)S5K
17N)2D1
VL7)16Q
5FP)J5J
NBL)TLF
QDV)Z9Z
2S1)VXW
K22)G6R
DTL)9Z8
BXN)YXJ
VYW)LNX
WJ4)4LM
JTX)DPL
SLM)DNF
YM4)J37
4L5)BF4
2RT)8DD
FNB)KD8
PK8)9PD
RLR)4SY
TM2)661
PQ6)2FF
92J)XMC
GDM)21R
ZTG)VV5
X3B)ZZW
5XQ)H6J
WTL)W1N
PNM)H8Z
6MY)CG5
72V)RLR
3R6)WCZ
PF8)YRM
HPY)88T
X8P)MHN
7LY)6VJ
2FF)C9G
K4Q)4BS
X63)2WD
XM8)R8Q
G98)Z3X
44B)8DJ
PWZ)V1P
MDD)7ZX
RRD)T49
YBJ)DBX
GXQ)BXP
6FX)88P
WXZ)H22
18F)WTL
NQF)885
L4T)XM2
V82)4DY
6N6)TDX
172)QRK
N6L)23T
CSP)5MP
GLG)9TQ
9P6)R76
W1N)N3F
R8G)JZ1
H2L)F2D
TGT)C9S
7BS)2XQ
FWH)TVM
23T)K3L
WQX)37M
3ZW)QH7
BGP)FW6
YFC)FGH
JCF)94D
WRQ)9JN
GN5)6RM
V6H)QGG
C1S)XK9
FFY)9SF
WPX)HKT
7Y1)NBL
6RN)GPD
NYJ)YDT
934)FKL
P3N)FXS
ZQS)KXC
4KW)PVJ
FMS)V26
NT8)WM9
7Z7)G4J
NN9)2TT
VXW)DJB
C97)Q3Q
VVQ)486
CB6)DGN
DGQ)WMN
FM5)GJF
6YY)DPN
DDT)814
KF3)SFQ
G7S)TTB
R4C)S12
R8Q)35Y
GXN)JKH
J2H)K22
F5B)7LY
NCK)YSR
SMP)KY2
P4X)X3B
DR5)LZ8
8JQ)ZPV
WG9)SX3
NDQ)LM2
32S)LFV
K51)ND4
DP3)QDV
2W7)CCT
RSK)YH6
9NZ)NSB
K3L)RV3
HN8)414
92N)75S
61M)598
ZR6)53T
J8F)TB2
H22)5YX
HSJ)M63
4KW)5GB
HR7)89C
FQP)15J
TZ5)LYS
5FP)WG4
4HL)PBL
C8J)D1C
TS3)83C
C3G)1QT
GZD)DNK
2RB)KSF
BCD)SF9
327)9QZ
4FF)ZQS
6L9)82F
TJK)123
X9M)FMY
R92)TV1
CDL)2PW
7ZX)58Y
C2N)H5T
8MN)TH2
GN7)G44
HKL)61M
XD6)1C9
ZCS)NQ8
2L8)QGH
DFC)XX7
S5K)XM8
58J)8L9
PRR)4KM
6XT)N2T
FM2)7Y1
V26)HR7
2TT)91P
88F)ZW1
JBG)891
WZT)VZQ
PYG)NHS
2QT)P9H
FB2)9QT
MP3)PQ6
WZV)YBJ
H53)VYW
N2T)VQW
ZK2)6RT
SQS)5XQ
DPN)W8H
TSW)2L8
73H)R7Q
F1Z)B9Q
M8G)9KS
NSB)H41
WBJ)THR
KSF)KGS
PXH)3BS
4NT)ZRV
VNZ)8X5
98S)DR1
3XY)TJK
JNX)P9Z
F4P)6YY
VRT)VKV
TVM)92N
WLR)S62
D5X)ZVB
152)9P6
F2D)PN3
2R2)D6K
ZRV)2NX
67X)TS3
HWP)YBD
5FC)SRV
X92)CQR
8N9)DFC
Q5J)H8N
GGG)8LK
PPQ)841
6RT)WJ8
KQR)788
92N)Q62
W7S)98S
S19)NFK
VRM)ZVY
GL8)DCH
4YM)17N
F13)D3J
QVC)VGG
31S)J81
934)93X
Q21)R4C
TH2)HQ4
1C9)114
83N)X5B
S3L)T4L
SD2)ZWX
SC1)6K1
TXD)ZPF
3GY)NQ1
Z9M)9NS
D6R)7V5
WG4)C2N
SVH)JBT
TR8)K4V
MPD)7SR
Y36)DP3
LM2)K21
KD1)2S1
FC8)J3M
JFD)BWF
6Y3)88F
STT)GXN
KKC)DW9
52L)B3G
5BK)XWX
H6X)5BQ
YH6)FGK
VM3)6RN
BWH)1PZ
JK2)BCD
9QT)LTX
6WK)F5B
HYC)C97
GSP)3MR
6M3)B9P
7LD)9NZ
N31)7KS
GGG)9PB
6RM)SC1
83C)TGT
BWT)1TJ
7YH)CN7
JLD)PFT
W95)JNX
ZBT)Y2F
TPD)RSK
3BS)BR6
KG9)X9M
6HL)5D3
WMN)P9S
V32)V6H
RYJ)3HM
SF9)SBK
HQ4)19C
BWF)2W7
2LC)91R
VCD)3G2
VKJ)9BJ
NL2)VKJ
NLF)HY7
BR6)L6V
TQ4)8N9
QH7)W84
D6K)GGG
W1J)SLM
DB2)RQS
M18)RRD
DXS)32S
KL8)YRT
Q5W)VD4
SCC)WQX
GQW)VRT
52N)J47
Z9M)6FX
FKX)GSK
TKX)WRS
7YH)NFG
9Y6)WZV
7BS)48W
NT5)KSX
1Y8)RYV
814)J2F
GSK)KRQ
FBC)W1J
C9G)VRY
V2F)8TN
5M4)Y9S
X9Q)T71
Q7G)9HB
YLR)4YM
18L)29D
M1M)1BT
82F)MX3
9BJ)BBL
R55)HRQ
9RH)R5S
414)MHV
ML2)Y8T
179)7YH
GK3)5Q9
7CZ)VRM
QP3)LDF
YRM)D92
KXC)B8Y
2ZB)6X1
XWX)JCF
B3G)CG7
LHT)4NT
NNN)5LW
48W)R9S
91R)6J4
97T)BBF
1BT)KWW
B53)QL9
HBC)DB2
2D1)89S
BBL)WBG
7WK)LJP
TV1)SNJ
K5Q)L9Q
HNN)YKD
BXP)W81
64N)GLG
L9Q)M4Y
PTB)8JP
RHP)3FJ
657)PWZ
HZ7)T8S
YKD)W25
PYL)SMP
YSQ)HSJ
W84)9F8
HQS)NL2
3MC)6Y3
24K)MDD
G3D)B17
2QF)PXH
38D)FNB
614)SVH
34H)BWT
6L3)GZD
J1Y)QJ9
3XF)GG9
MHQ)25V
3PQ)L4T
57T)3W4
8KQ)614
PC9)P5K
C2Q)F4P
J37)FW2
TDC)M1F
YD4)LZV
4H5)HYC
VRY)5XG
3W4)YLR
JZ1)Q21
SBK)1XQ
RVK)6XT
9NS)LVG
JBT)612
J4R)QWT
YDT)31V
123)C78
PN3)PTM
6QM)5GN
CG7)PF8
HX4)T4H
6CK)HXR
PFT)H53
62W)FC8
KQ6)4X5
3CK)8Z5
RHS)Q7G
N13)2LT
D3J)NYJ
PFY)N44
N3F)4FF
P89)29Z
MHN)J49
S78)LCK
8DD)DR5
51H)VM3
1NF)MLL
S99)RKK
9PD)JLD
7Q5)7WK
7WK)MND
CQR)KG9
X5B)VTN
SCW)RPP
BF4)3XF
QL9)R8G
LDB)4X4
VDN)73Z
9KY)BXN
VTN)WJ1
8F4)PR2
F78)92J
7SX)GQW
GR7)22P
56B)7SX
876)HPY
HNP)V7Y
SNJ)HHH
KNR)W7S
HT2)T9W
PC2)B9Y
6VJ)HNP
4TS)32D
7FB)TZ5
5BQ)PRT
PRT)97T
G44)9DX
THR)81Z
25P)MCD
Z9L)R92
V1P)L1D
DSZ)TVC
VKV)TQ4
4LM)YM4
T8S)6N6
HPP)WV4
99X)8YW
B9P)HDV
T2Q)TNM
JVG)7XG
QGH)KTD
DBX)VNZ
DFM)RSW
29D)GYC
LYS)38D
MKK)WXZ
5B5)MQ5
BPX)2NW
5LR)PYY
VXL)6HL";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(162439, p1(IN));
    }

    #[test]
    fn test_p2() {
        assert_eq!(367, p2(IN));
    }
}

