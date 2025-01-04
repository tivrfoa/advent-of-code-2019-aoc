function day24(input, part2) {
	let [init, gates] = input.split("\n\n")
    // console.log(init);
    // console.log(gates);

	let wires = {}
	let numBits = 0

	init.split("\n").forEach(line => {
		let [wire, bit] = line.split(": ")
		wires[wire] = +bit
		numBits = +wire.slice(1) + 1
	})
	console.log(numBits)
	let gatesMap = new Map()

	gates.split("\n").forEach(line => {
		let [in0, op, in1, _, out] = line.split(" ")

		let gates0 = gatesMap.get(in0)
		if (!gates0) {
			gatesMap.set(in0, gates0 = new Map())
		}

		let gates1 = gatesMap.get(in1)
		if (!gates1) {
			gatesMap.set(in1, gates1 = new Map())
		}

		gates0.set(op, { other: in1, out })
		gates1.set(op, { other: in0, out })
	})
    // console.log(gatesMap);

	let z = []
	let swaps = new Set()

	function expect(in0, op, in1) {
		console.log(in0, op, in1);
		let res = gatesMap.get(in0)?.get(op)

		if (!res) {
			res = gatesMap.get(in1).get(op)
			swaps.add(in0)
			swaps.add(res.other)
			in0 = res.other
		} else if (res.other != in1) {
			swaps.add(in1)
			swaps.add(res.other)
			in1 = res.other
		}

		let out = res.out
		wires[out] =
			op == "AND" ? wires[in0] & wires[in1] :
			op == "XOR" ? wires[in0] ^ wires[in1] : wires[in0] | wires[in1]

		if (out[0] == "z") {
			z.unshift(wires[out])
		}

		return { in0, in1, out }
	}

	let carry

	for (let bit = 0; bit < numBits; bit++) {
		let x = "x" + bit.toString().padStart(2, "0")
		let y = "y" + bit.toString().padStart(2, "0")
		console.log(x, y);

		let low = expect(x, "XOR", y).out
		let newCarry = expect(x, "AND", y).out

		if (bit > 0) {
			({ in0: carry, in1: low } = expect(carry, "XOR", low))

			let cont
			({ in0: carry, in1: low, out: cont } = expect(carry, "AND", low));
			({ in0: newCarry, in1: cont, out: newCarry } = expect(newCarry, "OR", cont))
		}

		carry = newCarry
	}

	return part2 ? [...swaps].sort().join(",") : z.reduce((a, b) => a * 2 + b)
}

if (typeof window == "undefined") {
	module.exports = day24
}

let input = `x00: 1
x01: 1
x02: 0
x03: 0
x04: 0
x05: 1
x06: 0
x07: 1
x08: 1
x09: 0
x10: 1
x11: 0
x12: 0
x13: 0
x14: 1
x15: 0
x16: 1
x17: 1
x18: 0
x19: 1
x20: 0
x21: 0
x22: 1
x23: 1
x24: 1
x25: 1
x26: 0
x27: 1
x28: 1
x29: 0
x30: 1
x31: 0
x32: 0
x33: 1
x34: 1
x35: 0
x36: 1
x37: 1
x38: 1
x39: 1
x40: 1
x41: 1
x42: 1
x43: 1
x44: 1
y00: 1
y01: 0
y02: 1
y03: 1
y04: 0
y05: 0
y06: 1
y07: 1
y08: 0
y09: 1
y10: 1
y11: 1
y12: 1
y13: 0
y14: 1
y15: 1
y16: 1
y17: 0
y18: 1
y19: 0
y20: 0
y21: 1
y22: 0
y23: 1
y24: 0
y25: 1
y26: 0
y27: 1
y28: 0
y29: 0
y30: 1
y31: 1
y32: 0
y33: 1
y34: 0
y35: 0
y36: 1
y37: 0
y38: 1
y39: 0
y40: 0
y41: 0
y42: 1
y43: 0
y44: 1

y08 AND x08 -> pkh
grg AND twt -> bbk
vvt OR wwt -> vgs
x10 XOR y10 -> pmq
pmq XOR hkf -> z10
vmw OR bfb -> hkf
twp OR qbq -> kmj
qns AND mwj -> qhk
dqm XOR cqp -> z02
snr AND crb -> htp
jwv XOR dgj -> z09
sjf OR rwf -> wkq
y02 XOR x02 -> dqm
msw AND qqp -> rss
fgv XOR bhw -> z26
y03 AND x03 -> ftb
kmj XOR fnh -> z24
jhv XOR bkq -> wtt
x27 XOR y27 -> knf
y40 XOR x40 -> jhf
gmq OR nrs -> kkq
y03 XOR x03 -> ndn
x25 XOR y25 -> hfj
x33 AND y33 -> rvf
wkh AND pmn -> vrq
mgg OR hbf -> fhw
nrr OR rtk -> wqr
x00 AND y00 -> mtk
crb XOR snr -> z30
y37 XOR x37 -> qns
kgw OR wms -> snr
y09 XOR x09 -> jwv
mtk AND kvc -> jjp
x01 XOR y01 -> kvc
jgg AND bsn -> fpm
kvt OR ftb -> drd
x15 AND y15 -> tfs
x34 XOR y34 -> jgv
y44 XOR x44 -> vjg
x21 XOR y21 -> fqr
x36 XOR y36 -> rwh
y30 AND x30 -> kgr
sqg OR hfb -> rfq
x05 XOR y05 -> kbj
rjq OR skn -> jhv
y17 AND x17 -> bmp
x28 AND y28 -> jwb
x44 AND y44 -> vrj
wwn XOR gwm -> z23
y39 AND x39 -> gsr
wkq XOR tfv -> z15
x31 AND y31 -> sst
x08 XOR y08 -> twt
wnf AND rsk -> pjp
mmr AND qdc -> rwf
y22 XOR x22 -> jsr
pnj OR bcc -> hhn
fpm OR wtv -> kth
kkq XOR jsr -> z22
srn XOR kth -> z33
vrq OR mrt -> tkc
wwn AND gwm -> qbq
jhf XOR mdn -> z40
y13 XOR x13 -> tqf
kvc XOR mtk -> z01
stg XOR kdv -> z35
grg XOR twt -> z08
kkq AND jsr -> fjh
tmd AND shm -> rjm
nvq AND chk -> gnp
twb OR ptq -> qqp
x32 XOR y32 -> bsn
y39 XOR x39 -> bkq
mph OR vbp -> stg
x02 AND y02 -> rks
dtj AND hfj -> frh
y43 XOR x43 -> qmr
chk XOR nvq -> z28
x16 AND y16 -> wnf
y06 XOR x06 -> ghf
rhr OR ccq -> qjq
x38 AND y38 -> rjq
tsf AND dtb -> twb
x15 XOR y15 -> tfv
vds AND gdk -> skn
hhn XOR rwh -> z36
x40 AND y40 -> bjf
wtd OR cks -> nvq
jpk OR qhk -> vds
x14 XOR y14 -> qdc
y19 AND x19 -> ptq
fnh AND kmj -> gqj
pjp OR vtj -> smg
x11 XOR y11 -> tmd
bbk OR pkh -> dgj
bkq AND jhv -> z39
y07 AND x07 -> krp
hkf AND pmq -> crj
y22 AND x22 -> cdk
knb OR vkk -> mmr
vds XOR gdk -> z38
jhf AND mdn -> qkq
vrj OR gkc -> z45
x41 AND y41 -> mrt
jgv XOR dkh -> z34
y16 XOR x16 -> vtj
rcf OR gqj -> dtj
rvf OR bqr -> dkh
x25 AND y25 -> swn
dbv XOR vhs -> z31
y12 XOR x12 -> fgq
y20 AND x20 -> kcf
rss OR kcf -> mjj
hfj XOR dtj -> z25
x14 AND y14 -> sjf
fpd XOR smg -> z17
x21 AND y21 -> z21
x43 AND y43 -> prt
x31 XOR y31 -> vhs
grv OR tfs -> rsk
y33 XOR x33 -> srn
msw XOR qqp -> z20
gsr OR wtt -> mdn
x30 XOR y30 -> crb
y24 AND x24 -> rcf
frh OR swn -> bhw
htm AND ndn -> kvt
x06 AND y06 -> hbf
x35 XOR y35 -> kdv
srn AND kth -> bqr
dgj AND jwv -> bfb
bsn XOR jgg -> z32
rjm OR skj -> tff
gcf AND fhw -> mpv
vfb OR jjp -> cqp
x00 XOR y00 -> z00
crj OR fgc -> shm
tff XOR fgq -> z12
mwj XOR qns -> z37
vjg AND kmg -> gkc
x19 XOR y19 -> dtb
hvs XOR tsn -> z18
dtb XOR tsf -> z19
y05 AND x05 -> srp
kkh AND bsb -> kgw
rks OR trv -> htm
frn AND ghf -> mgg
qjq XOR kbj -> frn
knf XOR vgs -> z27
y09 AND x09 -> vmw
x36 AND y36 -> dbc
knf AND vgs -> wtd
y23 XOR x23 -> wwn
wnf XOR rsk -> z16
y29 XOR x29 -> kkh
vjg XOR kmg -> z44
htp OR kgr -> dbv
htm XOR ndn -> z03
rfq XOR tqf -> z13
mjj AND fqr -> nrs
gcf XOR fhw -> z07
qmr AND wqr -> frs
x32 AND y32 -> wtv
x07 XOR y07 -> gcf
gmf OR pvh -> tsf
mjj XOR fqr -> gmq
qqs OR dbc -> mwj
krp OR mpv -> grg
x34 AND y34 -> vbp
tff AND fgq -> hfb
drd AND wkb -> rhr
tqf AND rfq -> vkk
x13 AND y13 -> knb
bhw AND fgv -> vvt
hhn AND rwh -> qqs
kbj AND qjq -> jcf
y41 XOR x41 -> wkh
y01 AND x01 -> vfb
jgv AND dkh -> mph
gnp OR jwb -> bsb
cdk OR fjh -> gwm
prt OR frs -> kmg
x17 XOR y17 -> fpd
y42 AND x42 -> rtk
fpd AND smg -> tqm
dbv AND vhs -> gdw
y29 AND x29 -> wms
kkh XOR bsb -> z29
cqp AND dqm -> trv
hvs AND tsn -> gmf
wkq AND tfv -> grv
x04 AND y04 -> ccq
x20 XOR y20 -> msw
y26 XOR x26 -> fgv
mmr XOR qdc -> z14
ghf XOR frn -> z06
y26 AND x26 -> wwt
sst OR gdw -> jgg
pmn XOR wkh -> z41
y04 XOR x04 -> wkb
y28 XOR x28 -> chk
wqr XOR qmr -> z43
kdv AND stg -> bcc
y18 AND x18 -> pvh
mvk XOR tkc -> z42
x27 AND y27 -> cks
bjf OR qkq -> pmn
tkc AND mvk -> nrr
y38 XOR x38 -> gdk
y37 AND x37 -> jpk
x23 AND y23 -> twp
tqm OR bmp -> tsn
wkb XOR drd -> z04
x42 XOR y42 -> mvk
y35 AND x35 -> pnj
shm XOR tmd -> z11
y24 XOR x24 -> fnh
x11 AND y11 -> skj
x10 AND y10 -> fgc
srp OR jcf -> z05
y12 AND x12 -> sqg
y18 XOR x18 -> hvs`;


console.log(day24(input, true));