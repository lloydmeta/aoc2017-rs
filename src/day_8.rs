use std::collections::HashMap;
use std::hash::Hash;
use combine::char::*;
use combine::primitives::*;
use combine::*;
use combine::easy::*;

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    register: String,
    op: Op,
    amount: i64,
    check_register: String,
    cond: Cond,
    cond_amount: i64,
}

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Inc,
    Dec,
}

#[derive(Debug, PartialEq, Eq)]
enum Cond {
    GT,
    LT,
    GTE,
    LTE,
    E,
    NE,
}

impl Cond {
    fn compare(&self, target: i64, amount: i64) -> bool {
        match self {
            &Cond::GT => target > amount,
            &Cond::LT => target < amount,
            &Cond::GTE => target >= amount,
            &Cond::LTE => target <= amount,
            &Cond::E => target == amount,
            &Cond::NE => target != amount,
        }
    }
}

struct Simulation<'a> {
    instructions: &'a Vec<Instruction>,
    registers: HashMap<&'a str, i64>,
    historical_highest_reg_value: Option<i64>,
    current_highest_reg_value: Option<i64>,
}

impl<'a> Simulation<'a> {
    fn new(instructions: &'a Vec<Instruction>) -> Simulation<'a> {
        Simulation {
            instructions: instructions,
            registers: HashMap::new(),
            historical_highest_reg_value: None,
            current_highest_reg_value: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SimulationResult {
    pub historical_highest_reg_value: Option<i64>,
    pub current_highest_reg_value: Option<i64>,
}

fn run(s: &mut Simulation) -> () {
    s.instructions.iter().fold(s, |simulation, i| {
        // arg borrow checker..
        let should_proceed = {
            let check_register_value = simulation
                .registers
                .get(i.check_register.as_str())
                .unwrap_or(&0);
            i.cond.compare(*check_register_value, i.cond_amount)
        };
        if should_proceed {
            match i.op {
                Op::Inc => {
                    *simulation.registers.entry(i.register.as_str()).or_insert(0) += i.amount
                }
                Op::Dec => {
                    *simulation.registers.entry(i.register.as_str()).or_insert(0) -= i.amount
                }
            }
        }
        let max_value_in_reg = max_value(&simulation.registers);
        simulation.current_highest_reg_value = max_value_in_reg;
        match max_value_in_reg {
            None => (),
            Some(v) => match simulation.historical_highest_reg_value {
                Some(old_highest_reg_value) => if v > old_highest_reg_value {
                    simulation.historical_highest_reg_value = Some(v)
                },
                None => simulation.historical_highest_reg_value = Some(v),
            },
        }
        simulation
    });
}

/// Parses and runs register instructions
///
/// # Example
/// ```
/// # use aoc_2017::day_8::*;
/// const TEST_INPUT: &str = r#"
/// b inc 5 if a > 1
/// a inc 1 if b < 5
/// c dec -10 if a >= 1
/// c inc -20 if c == 10"#;
///
/// let results = simualate_instructions(TEST_INPUT).unwrap();
/// assert_eq!(results.current_highest_reg_value, Some(1));
/// assert_eq!(results.historical_highest_reg_value, Some(10));
/// ```
pub fn simualate_instructions(
    s: &str,
) -> Result<SimulationResult, Errors<PointerOffset, char, &str>> {
    let (instructions, _) = Instruction::parse(s)?;
    let mut simulation = Simulation::new(&instructions);
    run(&mut simulation);
    Ok(SimulationResult {
        current_highest_reg_value: simulation.current_highest_reg_value,
        historical_highest_reg_value: simulation.historical_highest_reg_value,
    })
}

fn max_value<'a, K, V>(hash: &'a HashMap<K, V>) -> Option<V>
where
    K: Eq + Hash,
    V: Copy + Ord,
{
    hash.iter().fold(None, |acc, (_, v)| match acc {
        None => Some(*v),
        Some(old_v) => if *v > old_v {
            Some(*v)
        } else {
            acc
        },
    })
}

// Can't be arsed to figure out the return types for these horrors.
macro_rules! number_parser {
    ($t: ty) => {
        token('-')
        .with(many1::<String, _>(digit()).and_then(|s| s.parse::<$t>().map(|v| -v)))
        .or(many1::<String, _>(digit()).and_then(|s| s.parse::<$t>()));
    }
}
macro_rules! tabs_or_spaces {
    () => {
        many::<Vec<char>, _>(try(char(' ')).or(char('\t')))
    }
}

macro_rules! instruction_parser {
    () => {
        {
        let identifier_parser = many1::<String, _>(letter());
        let op_parser = try(string("inc"))
            .map(|_| Op::Inc)
            .or(string("dec").map(|_| Op::Dec));
        let amount_parser = number_parser!(i64);
        let cond_statement_parser = string("if");
        let cond_target_parser = many1::<String, _>(letter());
        let cond_parser = (try(string(">=")).map(|_| Cond::GTE))
            .or(try(string("<=")).map(|_| Cond::LTE))
            .or(try(string("==")).map(|_| Cond::E))
            .or(try(string("!=")).map(|_| Cond::NE))
            .or(try(string(">")).map(|_| Cond::GT))
            .or(string("<").map(|_| Cond::LT));
        let cond_amount_parser = number_parser!(i64);
        identifier_parser
            .skip(tabs_or_spaces!())
            .and(op_parser.skip(tabs_or_spaces!()))
            .and(amount_parser.skip(tabs_or_spaces!()))
            .skip(cond_statement_parser.skip(tabs_or_spaces!()))
            .and(cond_target_parser.skip(tabs_or_spaces!()))
            .and(cond_parser.skip(tabs_or_spaces!()))
            .and(cond_amount_parser)
            .map(
                |(
                    (
                        (((parsed_identifier, parsed_op), parsed_amount), parsed_cond_target),
                        parsed_cond,
                    ),
                    parsed_cond_amount,
                )| {
                    Instruction {
                        register: parsed_identifier,
                        op: parsed_op,
                        amount: parsed_amount,
                        check_register: parsed_cond_target,
                        cond: parsed_cond,
                        cond_amount: parsed_cond_amount,
                    }
                },
            )
        }
    }
}

impl Instruction {
    // Wish I could separate this out into functions, but the whole impl trait story in Rust
    // makes that __reaally__ difficult
    pub fn parse(s: &str) -> Result<(Vec<Instruction>, &str), Errors<PointerOffset, char, &str>> {
        let mut instructions_parser =
            skip_many(newline()).with(sep_by(instruction_parser!(), spaces()));
        instructions_parser.easy_parse(s)
    }
}

#[cfg(test)]
mod tests {
    use day_8::*;

    const TEST_INPUT: &str = r#"
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10"#;

    #[test]
    fn parse_test() {
        let (parsed, _) = Instruction::parse(TEST_INPUT).unwrap();
        assert_eq!(parsed.len(), 4);
    }

    #[test]
    fn parse_real_test() {
        let (parsed, _) = Instruction::parse(DAY_8_INPUT).unwrap();
        assert!(parsed.len() > 1);
    }

    #[test]
    fn simualate_instructions_test() {
        let results = simualate_instructions(TEST_INPUT);
        assert_eq!(
            results,
            Ok(SimulationResult {
                historical_highest_reg_value: Some(10),
                current_highest_reg_value: Some(1),
            })
        );
    }

    #[test]
    fn simualate_instructions_real_test() {
        let results = simualate_instructions(DAY_8_INPUT);
        assert_eq!(
            results,
            Ok(SimulationResult {
                historical_highest_reg_value: Some(7037),
                current_highest_reg_value: Some(4902),
            })
        )
    }

}

pub const DAY_8_INPUT: &str = r#"
ioe dec 890 if qk > -10
gif inc -533 if qt <= 7
itw dec 894 if t != 0
nwe inc 486 if hfh < -2
xly inc 616 if js >= -3
j inc 396 if b != -5
nwe dec -637 if uoc > 0
b inc 869 if yg >= -3
gif dec -221 if iyu < 0
tc dec -508 if gy >= -7
x dec 637 if gif < -526
nwe dec -185 if nwe != -8
x inc 638 if b != 869
ih dec -722 if itw > 9
xly inc -38 if ih >= 8
hm dec 910 if t == 0
uoc dec -585 if qt == 0
js dec -325 if hm == -910
yr dec -922 if cp != 0
qt dec 316 if itw != 2
bi dec -422 if iyu <= -1
uoc inc -862 if itw <= 3
itw dec -301 if x < -632
gif inc -492 if fi != 5
uoc inc -745 if x < -631
xly inc 21 if js > 331
hm inc 44 if js > 334
js dec 503 if tc > 503
t inc -216 if j == 396
yg inc 559 if nwe > 189
bhp dec -214 if x >= -646
hm dec 366 if fi == 0
t dec -658 if nwe == 185
hm inc -432 if qt <= -307
xly dec 695 if uoc >= -1031
cp inc -438 if x != -647
yg dec 211 if x >= -628
bi inc 829 if ih > -8
yg dec 540 if tc >= 503
hm dec -913 if qt > -310
qk inc -406 if itw < 309
uoc dec -716 if iyu >= -1
ih inc -655 if qt != -316
ih inc 6 if xly > -80
cp inc 795 if xly > -88
bhp dec 59 if yr < 1
yr dec 952 if x >= -628
xly dec -867 if j > 393
fi inc 720 if ioe >= -892
gif inc 454 if ioe > -886
j dec 547 if fi != 720
qk inc 665 if bi > 819
hm dec -174 if cp != 357
hm dec -795 if uoc <= -314
uoc inc 273 if itw <= 307
gy dec 212 if xly >= 783
tc inc 918 if ih != 9
tc inc -43 if js >= -186
gif inc -615 if b == 869
bhp inc -335 if fi > 724
ih inc 747 if hm >= -1711
ih inc -515 if ioe != -881
yg dec 967 if cp == 357
yr inc -23 if qt < -309
gif dec -16 if cp == 357
itw inc 353 if uoc <= -41
cp dec -788 if b <= 869
bi dec 510 if itw < 306
yg inc 321 if qk > 265
itw inc -194 if gif == -1624
yr dec 484 if b == 869
yr dec 828 if yg != -1515
cp dec -700 if gy != -212
ioe dec -238 if iyu >= -5
xly inc -334 if bi != 316
js dec 642 if uoc < -27
cp inc 131 if x >= -633
cp dec 693 if iyu >= -2
bi inc 671 if hm >= -1712
fi dec 781 if nwe >= 176
ioe inc 770 if qk > 253
nwe dec 381 if j < 406
qt inc -599 if hm != -1715
yg inc 277 if qk >= 268
hm dec -656 if ioe < 117
uoc dec -875 if ih < 243
js dec 297 if yg >= -1514
hfh dec 821 if iyu >= 8
ioe inc -133 if iyu > -9
x dec -623 if iyu != 0
gy inc 240 if cp >= 451
gy inc 937 if hfh != -10
tc dec 476 if tc != 1376
iyu dec -35 if hm == -1706
nwe dec 86 if yr >= -1344
cp dec 96 if qk != 259
x dec 864 if b >= 865
hm dec -965 if bhp != 161
bhp dec 402 if b == 859
b inc -19 if hm <= -746
xly dec 24 if uoc >= 840
ih inc 816 if js >= -1117
xly dec 511 if iyu != 5
t dec -214 if iyu != -8
yr inc -81 if tc <= 898
js inc -187 if yr < -1331
x inc -970 if cp < 462
hm dec 668 if tc > 904
uoc dec -488 if yg <= -1502
uoc inc -974 if j != 394
qk inc -58 if qt > -913
j dec -741 if js == -1304
j dec 58 if nwe != -276
fi dec 772 if qt != -910
yg dec -271 if bi == 990
uoc dec 983 if gy != 966
t inc -355 if uoc >= -636
hfh dec -486 if yr == -1335
itw inc -102 if tc < 905
gif inc 663 if b > 862
fi dec -546 if js < -1294
bhp dec -955 if qt > -914
qt dec -927 if ih == 1054
gy inc -272 if uoc <= -620
yg inc -30 if nwe != -291
nwe dec 393 if ih != 1048
ih dec 904 if tc != 915
nwe inc -211 if j != 1084
qt dec 249 if tc <= 913
ioe dec -703 if gy <= 695
hfh inc -836 if j <= 1079
tc inc -153 if qk <= 255
bhp dec -697 if uoc == -627
hfh dec -762 if qk <= 263
xly inc -369 if qk >= 269
xly inc -135 if xly > -80
j dec 773 if x <= -2468
iyu inc -541 if iyu < 9
itw inc 870 if xly < -77
x inc 646 if x >= -2474
qk dec 447 if fi >= -287
qk dec -37 if yr >= -1344
nwe inc 406 if gy >= 703
j inc 72 if j > 299
qt inc 946 if t < 306
hm dec -406 if cp == 451
ioe inc -906 if bhp < 849
xly inc -924 if b <= 871
xly inc -645 if qt == 709
b dec -578 if bi <= 999
js inc 237 if t <= 304
gif inc 425 if bi <= 989
uoc dec 318 if x < -1821
fi inc 455 if bhp < 859
fi inc -282 if iyu <= -537
bhp dec 980 if cp > 451
t dec 204 if gif > -967
ioe inc -273 if fi == -114
iyu inc 940 if yr != -1335
t inc -593 if uoc == -945
tc inc -34 if yr >= -1335
yg inc 204 if gif < -969
nwe dec -128 if iyu == -541
qk dec 949 if yr >= -1335
nwe dec -582 if js >= -1063
tc dec -550 if cp <= 452
js inc 427 if ioe <= 407
bhp inc -672 if gif == -961
b dec 890 if hm > -1419
qk dec -801 if iyu > -538
t inc 789 if tc != 1433
tc dec 134 if hm >= -1413
cp inc -287 if b > 552
b inc -131 if bhp == -800
iyu inc 991 if b <= 429
itw inc 3 if hm > -1411
ih inc 263 if b < 428
j dec 848 if t <= 294
xly inc 94 if ioe != 406
j inc 116 if qk < -1094
t inc -779 if yr >= -1338
nwe dec 910 if tc >= 1281
uoc inc -945 if itw <= 975
xly inc -675 if hm <= -1410
xly inc -58 if x == -1825
hfh inc -701 if bhp < -798
nwe inc -586 if uoc > -952
x inc -164 if b < 435
hfh inc -641 if j != -354
yg inc 827 if qt > 699
iyu dec -512 if t < -478
j inc 651 if itw < 987
yg dec 537 if hfh >= -289
ih inc -289 if qk <= -1094
itw dec 158 if nwe > -2260
xly dec 704 if hfh == -289
itw dec -564 if j <= 305
nwe inc -820 if hfh >= -290
bhp dec -252 if nwe < -3065
bhp dec 680 if itw <= 1385
cp inc 811 if t == -486
x inc 423 if b > 419
itw inc 131 if gy <= 693
qk inc 500 if nwe == -3065
gif dec 583 if yg <= -981
x dec -261 if fi > -115
itw dec 314 if cp >= 977
itw inc 197 if b < 429
ioe inc -666 if x <= -1314
hfh inc 345 if hm < -1402
b inc 653 if itw >= 1719
qk dec 298 if itw != 1701
t dec 943 if qt > 703
gif inc 430 if qt > 715
bi inc 60 if gy > 687
j inc -816 if gy > 685
xly inc 334 if js != -1075
qk inc -388 if xly != -2656
t inc 475 if fi <= -113
t dec -842 if nwe > -3078
fi inc 284 if cp == 976
xly inc 265 if ih >= 127
qt dec -481 if hfh < 58
xly dec -484 if ioe <= 415
ioe inc 525 if xly >= -2183
t inc 658 if tc >= 1281
nwe dec -680 if t != 555
qk inc 395 if tc <= 1293
bi dec 422 if gy > 683
itw inc 713 if t == 546
bhp dec 285 if iyu < 967
gif inc -186 if tc <= 1297
iyu inc -858 if iyu <= 954
gy inc -320 if nwe < -2384
yg inc 691 if qt < 1199
hm inc -19 if j <= -528
bhp dec 403 if xly == -2175
gy inc -886 if qk >= -1399
yg inc 404 if js < -1065
gy inc -255 if fi > 160
bhp inc 740 if bi <= 620
b inc -548 if nwe != -2398
t dec 255 if nwe >= -2397
t inc -678 if js >= -1064
js inc -497 if js >= -1071
iyu dec -463 if hfh >= 52
js inc -537 if b != -122
t inc -518 if yg != 129
ih dec 208 if yr >= -1341
qt dec -566 if yr > -1340
itw inc 113 if nwe == -2395
ioe dec -471 if itw != 2424
cp inc 222 if tc > 1283
ih inc 28 if bhp != -1915
yg inc -923 if xly <= -2169
bi inc 893 if yr == -1335
cp inc -459 if gy > -771
b inc 958 if bhp != -1922
yg dec 896 if tc < 1298
ih inc 385 if hfh != 48
j dec -663 if iyu <= 1419
fi dec 89 if x != -1297
tc inc -294 if gif != -1147
uoc inc -232 if qk < -1381
bi inc 743 if hfh >= 48
gif inc 317 if yr == -1335
hm dec 938 if xly <= -2172
iyu dec 961 if xly > -2173
qk dec -69 if qk == -1390
ioe dec -450 if hfh != 52
j inc 944 if qt != 1748
x dec 48 if gy <= -764
yr dec 505 if iyu <= 1422
x dec 316 if yg < -1695
bi inc -386 if nwe != -2392
xly inc 363 if fi < 87
uoc dec 629 if nwe < -2384
ioe dec 816 if bhp < -1909
uoc inc 196 if fi == 81
gif dec -196 if iyu > 1419
bi dec -652 if nwe != -2402
gy dec -416 if yg > -1701
bhp dec 78 if qk < -1384
bi inc -660 if ih != 328
ih dec 352 if fi >= 79
ioe inc 288 if itw == 2414
b dec -461 if xly < -1814
t inc -291 if js > -1557
tc dec 502 if j == 432
gif inc -800 if qk >= -1384
yr dec -59 if gif > -641
yg dec 360 if yr <= -1275
nwe dec 790 if gy >= -358
b dec 544 if yr == -1268
tc inc 743 if ih <= -22
bhp inc 314 if cp <= 744
x dec -708 if tc <= 2039
iyu inc 968 if yr != -1274
x dec 454 if hfh <= 61
qk dec -41 if bhp >= -1685
ih inc 302 if tc != 2032
cp inc -167 if yr == -1281
ih inc 349 if qt != 1760
gif inc -367 if uoc <= -1603
xly inc 229 if gy <= -344
gif dec 459 if fi >= 76
hm dec -297 if gy <= -347
j dec -320 if iyu <= 2397
qt dec 634 if b <= 837
cp dec 342 if ih > 324
ioe dec -119 if qt > 1120
x dec 564 if hfh != 56
tc dec 779 if qt < 1130
hm dec 586 if ioe > 695
yg dec 373 if ioe > 683
xly inc 344 if iyu <= 2397
hm inc 91 if qt < 1121
ih inc -219 if ioe <= 699
ih inc -937 if ioe <= 702
hfh dec -805 if t != -221
itw dec -385 if ioe == 693
nwe inc 946 if js != -1564
qt dec 685 if hm == -2052
itw dec -625 if bhp <= -1674
ih inc 933 if qt < 442
js inc 971 if itw != 3437
hfh dec 444 if uoc >= -1617
itw inc 182 if itw >= 3433
hfh inc 509 if gy > -353
ioe dec 952 if bhp > -1688
b inc 210 if uoc > -1615
itw inc -579 if hm < -2044
hm inc -427 if j <= 739
uoc inc 90 if nwe > -3190
ih inc -678 if x <= -1407
js dec -238 if x >= -1419
ih inc -263 if tc > 1244
ioe dec 30 if gy == -352
t inc -76 if ih < -837
yr dec -442 if cp < 400
iyu inc -425 if j < 751
qk inc -189 if js <= -350
yg inc 282 if t >= -311
b inc 11 if j > 737
qt dec -839 if bi == 1863
tc inc -40 if bhp >= -1688
b inc 439 if yg >= -2142
bhp dec 144 if b == 1057
gy dec 460 if b <= 1065
qk inc -681 if itw >= 3035
iyu dec 0 if j > 740
ih inc -189 if uoc <= -1520
bi inc 360 if hm > -2058
yr inc 603 if xly <= -1232
j dec 68 if fi < 90
yg dec -425 if gy <= -804
x dec 295 if x != -1405
nwe inc -355 if j <= 679
ioe dec -506 if xly < -1229
bhp inc -489 if j >= 671
hm inc 730 if ioe > 209
gy inc -464 if iyu < 1968
bi dec 191 if yg > -1731
j dec 707 if gy < -804
ioe dec -180 if qt == 437
ioe inc 909 if yr != -231
yr dec -627 if ih == -1027
itw dec -145 if gy == -818
hm inc -156 if gif == -1460
hm inc 301 if yg < -1735
gif dec 137 if bi == 2039
cp dec 777 if yg <= -1717
fi dec -90 if yg != -1726
cp dec 256 if tc <= 1217
gy dec -401 if ioe >= 393
js inc 850 if tc <= 1219
iyu inc 124 if hm >= -1478
tc dec 341 if nwe > -3534
yr dec -13 if tc <= 1218
x inc -44 if cp != -634
t inc 591 if qk >= -2229
t dec -38 if itw <= 3045
xly inc -996 if ioe >= 396
cp inc -21 if x > -1757
t dec -989 if itw >= 3033
uoc inc 193 if fi >= 74
x inc 234 if uoc > -1333
hm dec -353 if gif > -1597
cp inc 684 if t == 1315
bhp dec -437 if iyu < 2095
xly inc 59 if j == -30
nwe inc 176 if fi >= 81
b inc 573 if iyu > 2086
gif inc 526 if gy > -418
ioe dec 261 if b >= 1625
qk inc -976 if bi != 2040
qt inc -648 if cp >= 31
x dec -490 if iyu <= 2099
bi dec 450 if x <= -1021
hm dec -317 if hm >= -1485
x inc 462 if ih != -1032
qt dec 436 if yr == 409
uoc inc 289 if b <= 1638
qk inc -998 if uoc > -1045
j dec 387 if b < 1622
gy dec 13 if cp >= 24
bi dec -822 if js != 489
ioe dec -140 if xly < -2170
uoc dec -15 if ih < -1021
x dec -906 if hfh < 923
itw dec -986 if yr > 402
cp inc 474 if gy < -421
gy dec 530 if fi == 72
fi inc -277 if tc < 1214
ioe inc -26 if gif < -1066
itw dec -900 if ih < -1025
fi dec 443 if yg < -1719
hfh inc 602 if uoc == -1023
yg inc 409 if ih == -1027
t dec -366 if t != 1323
gif inc 158 if yr == 409
iyu inc 464 if hfh >= 1525
xly inc -71 if yg > -1319
cp dec -247 if qk > -4201
cp dec -278 if cp > 745
tc inc -903 if ioe < 257
itw inc 221 if bhp < -1870
yg dec -352 if j >= -31
ih dec -77 if qk < -4188
itw inc -921 if gy < -414
tc dec 565 if iyu != 2561
uoc inc -564 if cp <= 1035
ih dec -962 if qt == 1
hfh dec 646 if itw < 4222
js dec 604 if qk != -4186
hfh dec 389 if itw != 4214
cp inc -819 if bhp == -1876
tc inc 121 if qk <= -4185
cp dec 330 if t == 1677
nwe inc -864 if gy >= -417
uoc dec -459 if hm == -1155
x inc 886 if tc >= -124
tc inc 954 if yg != -973
qk inc 408 if x <= -563
hfh inc 37 if uoc == -1587
bhp dec -426 if cp == 197
nwe inc -46 if itw < 4233
j dec -546 if fi <= -637
bi dec 421 if xly > -2253
cp inc -991 if nwe >= -3417
gif inc -9 if hfh >= 1169
yg dec 397 if tc >= 816
gif inc 258 if js > -115
bhp dec -594 if tc != 822
t inc -975 if ioe != 253
tc inc 999 if ioe >= 242
gif inc 746 if yr < 419
gy inc -980 if yr > 407
qt inc 892 if cp > -789
itw inc 806 if yr != 402
hm dec 977 if iyu <= 2559
gy inc 848 if gy >= -1405
bi dec -310 if qt == 893
hfh inc -40 if yg == -1353
js inc -626 if j == 516
iyu dec 863 if yr < 408
bi inc 321 if b < 1633
qt inc 895 if hfh <= 1178
hfh dec -782 if js == -735
cp inc -443 if yr < 410
yg inc -446 if bi >= 2626
uoc inc -251 if tc < 1824
ioe dec 549 if yg < -1360
x dec -961 if yg <= -1362
gif inc -184 if js > -743
js dec 929 if x <= 397
yg inc 994 if yg == -1362
js dec -246 if bi < 2629
yg dec 875 if j <= 522
ih inc 287 if yg >= -1240
itw inc 615 if hfh == 1958
ih dec -665 if qt <= 1790
itw inc -747 if qt == 1788
iyu inc -631 if tc > 1810
j inc 178 if bhp >= -1273
nwe dec 85 if itw >= 4897
tc inc 530 if cp > -1222
fi dec 329 if tc >= 1819
uoc inc 1 if iyu != 1932
t dec 905 if nwe > -3497
j inc -836 if ioe >= -299
xly dec 185 if nwe > -3499
b inc 41 if gif >= -92
iyu dec 315 if t > -196
nwe dec 676 if uoc != -1839
bi dec 380 if tc != 1819
ih dec -397 if gif != -102
yr dec 449 if js > -1420
hfh inc 584 if bhp <= -1282
ih inc -524 if ioe >= -304
gif dec 688 if hfh == 2542
hm dec -370 if bi == 2621
js dec 575 if hm != -1768
qk dec -85 if itw != 4893
xly inc 961 if tc == 1819
itw inc 913 if qk <= -3699
bi inc -858 if js != -1420
t dec 622 if bi != 1764
qt inc -980 if fi >= -964
fi dec -380 if js == -1418
qt dec -446 if bi != 1772
gy inc -414 if yr != -50
qt dec 628 if itw < 5813
bi inc -649 if fi >= -588
uoc dec 821 if iyu >= 1916
gy dec 19 if xly > -1472
qt dec 137 if qt <= 1608
bhp dec -850 if t >= -830
bhp inc 19 if iyu != 1928
yr dec 272 if ih == 153
js inc 628 if hm > -1775
qk inc 893 if b != 1630
fi dec -959 if gif < -783
x inc 848 if bhp <= -405
bhp dec -555 if j != -318
gif dec -906 if hm > -1772
itw dec -722 if hfh == 2542
bhp dec 960 if j != -327
js dec 670 if gif >= 123
itw inc 153 if yr > -313
t dec -712 if ioe == -299
ioe inc 345 if itw == 6693
hfh dec -411 if bi <= 1123
hfh dec 368 if qt == 1469
ioe dec -688 if itw != 6684
cp inc 888 if bhp != -818
iyu dec 184 if fi != 378
xly inc 143 if fi == 371
cp dec 691 if ih <= 151
bi dec -712 if xly != -1333
nwe inc 231 if bhp < -808
qt dec -736 if fi != 361
hm inc 536 if gy < -988
yr inc -539 if j < -311
xly inc -311 if bi <= 1833
qk inc 271 if yg != -1237
uoc dec -40 if gif < 119
nwe inc 223 if yg >= -1239
qt inc 501 if itw <= 6685
xly inc 11 if t >= -109
hm dec 147 if j != -328
js inc -15 if hm == -1379
bhp inc -841 if fi > 362
fi inc -26 if ioe != 397
fi dec 283 if tc > 1817
uoc inc 105 if j < -314
ioe inc -885 if gif < 113
xly dec -849 if iyu == 1746
j inc -614 if yg != -1243
yg inc 115 if fi <= 62
itw inc -405 if cp != -1222
hm inc 854 if gy >= -988
tc dec 143 if cp >= -1232
gy dec -111 if b < 1635
ioe inc -939 if hfh == 2585
gif dec 43 if bi < 1831
gif dec 519 if itw < 6284
nwe inc 197 if gif != -449
bi dec -71 if bi != 1835
ih inc 479 if hfh >= 2595
ih inc -979 if qk <= -3438
iyu dec 653 if uoc == -2513
yr dec -346 if hfh <= 2585
gif inc 572 if xly != -1638
qk dec -891 if j > -322
qk dec -911 if ih == 150
tc inc 995 if iyu <= 1094
hfh inc -129 if x == 1241
xly dec -533 if gy > -884
gif dec 806 if b == 1630
hfh dec -805 if hfh >= 2453
iyu inc 242 if qt < 2714
fi dec 878 if bhp <= -1661
t dec 166 if hm == -1379
ioe inc -456 if hm == -1379
hm dec 225 if cp >= -1228
xly dec 820 if qt <= 2707
tc dec -786 if bi != 1898
uoc inc 751 if qk != -2547
b dec 922 if x >= 1238
j dec -179 if b <= 715
hfh dec -590 if qk == -2539
x dec 187 if itw == 6280
bi dec -967 if js < -804
tc inc 133 if qt <= 2713
t dec -411 if tc <= 3596
uoc inc 264 if fi < 61
gy inc 223 if ioe <= -1000
gif dec -942 if ioe >= -1008
hfh inc 999 if gy > -662
fi inc -485 if ih <= 153
nwe dec 733 if ioe > -1015
x inc -753 if bhp <= -1653
js dec 784 if gif < 263
bhp dec -307 if b != 703
itw dec -757 if ih == 153
iyu dec 785 if bi >= 2874
nwe inc -450 if t == 136
bi dec -77 if yg > -1132
js dec 990 if yr > -515
iyu inc 337 if bi > 2933
uoc dec 241 if ih != 153
bi dec 459 if hm == -1604
iyu inc -85 if bhp > -1353
qk inc -437 if hm != -1597
x inc -852 if qt <= 2708
gy dec 311 if hm != -1604
iyu dec 876 if t <= 143
x dec 779 if yr <= -513
hfh inc -843 if qt < 2707
bi inc 536 if yg < -1120
itw inc -886 if hfh != 4003
js inc 107 if hfh <= 4009
x dec 156 if yr >= -504
hfh dec 971 if j < -140
tc inc -480 if hm != -1595
x inc 993 if j > -142
tc inc 910 if yg != -1120
tc dec 571 if x != 432
x dec -194 if nwe > -4935
yr inc 288 if b <= 717
fi inc 970 if tc >= 3454
xly dec -554 if fi <= -431
ih inc -380 if iyu <= 706
qk dec 162 if nwe == -4925
fi dec 396 if gif != 261
xly inc 698 if yr >= -224
qt dec 57 if hm >= -1607
tc inc 555 if x <= 644
x inc -20 if xly == -1217
ih dec -766 if ioe <= -1005
tc dec -101 if gy != -660
hm dec 989 if ioe == -1006
yr inc 995 if gy != -651
cp inc -70 if cp < -1232
gy inc -408 if hfh > 3027
yg dec 360 if bhp >= -1352
hm inc 263 if qk < -3137
qt dec -603 if yg > -1495
xly inc 516 if js <= -2466
ioe inc -442 if uoc >= -1764
ih inc 898 if gy < -1053
gif dec 863 if tc >= 4114
hfh dec 478 if hfh != 3030
js dec -623 if hm > -2328
itw dec 792 if qt > 3244
yg inc -367 if tc == 4105
gy inc -330 if b <= 709
x inc 821 if t == 136
bi inc -979 if cp >= -1235
nwe dec -608 if qt == 3252
cp inc 885 if yr == 778
js dec 197 if b > 717
ih inc -737 if qt != 3248
iyu dec -570 if tc < 4107
x dec 63 if bi == 2039
t dec 323 if j < -133
tc inc -962 if tc > 4098
ih inc 663 if xly == -701
ih inc 489 if ih >= 1363
uoc inc -393 if js >= -2480
j inc -62 if cp == -342
gy dec -29 if bhp < -1348
yg inc -202 if nwe >= -4322
fi dec -628 if bi == 2039
itw inc -338 if itw > 5350
qt dec 24 if qt > 3242
tc inc 877 if t == -188
nwe inc 521 if ioe != -1448
gif inc -328 if gif > 261
ih inc -277 if bhp != -1351
tc inc -61 if qk > -3148
nwe inc 588 if xly != -702
qk dec -762 if qk >= -3141
gif inc 937 if b <= 708
t inc -749 if fi == -191
js inc 814 if qk == -2376
j inc -479 if bhp < -1346
iyu inc -941 if gif == 876
js dec 411 if hm > -2326
itw inc -832 if gy == -1364
iyu dec 537 if xly != -706
yg inc 687 if js == -1658
b inc 741 if tc > 3073
x dec 86 if ih < 1584
fi inc 851 if t > -940
x dec 680 if qk >= -2373
gy inc 882 if itw != 4183
tc dec 711 if fi < 658
yg dec -166 if qt == 3228
yg dec 684 if fi > 661
bi inc -715 if gif < 875
gy dec 168 if nwe != -3719
gif inc 301 if yg != -1204
ioe inc 293 if cp >= -344
yr dec -395 if bhp <= -1351
qk inc 97 if js > -1659
hm inc 371 if gif > 865
gif inc 443 if uoc >= -2151
nwe dec -677 if cp > -351
qk dec -845 if yg != -1201
ih dec -890 if cp != -337
nwe dec -932 if hfh > 2549
xly dec -281 if ih <= 2473
cp dec 76 if j == -682
hfh dec 236 if gy == -650
uoc inc 804 if b != 1448
iyu dec -791 if yg <= -1205
yr inc -720 if bi == 1324
b inc -234 if yg == -1204
bhp dec 728 if t != -938
qk dec 703 if itw == 4194
hfh dec 944 if uoc <= -1360
yg dec -384 if hfh < 2324
hm inc -765 if fi == 660
b inc -769 if hfh == 2322
ioe dec 681 if b >= 455
iyu inc -795 if itw > 4180
bhp dec -533 if t < -933
j inc 861 if ih < 2475
ioe inc 136 if itw < 4189
b dec 400 if fi >= 663
fi inc -483 if gif < 868
bhp dec 815 if qt == 3228
x inc -236 if yr > 456
xly inc 516 if b > 443
bhp inc -95 if x != 1295
uoc inc 174 if js > -1668
bi inc 126 if b <= 454
uoc inc -742 if gy >= -655
ih dec -381 if qt > 3235
itw inc -493 if bi <= 1449
fi inc -673 if fi <= 653
ih inc 55 if x >= 1284
tc inc 55 if fi == 657
ioe inc -715 if yg <= -820
nwe inc -665 if qk != -1432
qt inc -129 if b <= 443
fi inc 984 if yg <= -819
cp dec 112 if bi < 1454
j dec -268 if hm <= -2729
fi inc -620 if itw <= 4191
nwe dec 330 if tc != 3077
xly dec -490 if bi == 1450
cp dec 922 if yg >= -825
itw inc -752 if uoc < -1914
yg inc 502 if tc == 3082
hm inc 987 if qt > 3226
qk inc -234 if bi <= 1451
ioe inc 201 if iyu > -64
fi dec 730 if xly <= 591
js inc 506 if qk <= -1661
x dec 453 if yr <= 455
t inc -941 if uoc > -1923
hm inc 30 if bi < 1458
gy inc 947 if fi > 292
hm dec 976 if iyu == -56
tc inc -420 if qk <= -1667
tc dec -779 if gy < 302
bi inc -814 if x < 845
qt dec 484 if bi > 643
yr inc 363 if fi >= 290
ioe inc -314 if gif > 866
ioe inc 84 if yg < -312
bhp dec -941 if itw >= 3447
gif inc -791 if ioe >= -1898
yg dec -440 if ioe != -1904
b dec -673 if x != 835
tc dec -547 if qt <= 3233
t inc 641 if yg < 120
xly dec 548 if fi < 293
iyu dec -947 if hm >= -2687
gif inc 643 if yg > 119
iyu dec 681 if j >= 174
ioe dec 326 if cp == -1452
cp dec 489 if js == -1151
bi dec -814 if js != -1152
tc inc -803 if yr >= 812
qk inc 155 if itw >= 3429
bhp inc -445 if tc < 3188
itw inc -280 if js > -1147
js inc -921 if ioe > -2227
gy inc 200 if b == 446
js inc -943 if nwe > -3125
gif inc -412 if hfh == 2322
t inc 240 if ioe == -2225
iyu inc 774 if b != 440
yr dec -305 if ih != 2522
ih dec -24 if uoc > -1928
yr inc 770 if gif != 1102
iyu inc 440 if yg != 115
gy inc 148 if qt >= 3226
ih inc 534 if tc != 3182
yr inc -140 if js > -3019
ih dec 369 if bhp < -2894
x dec -155 if uoc <= -1915
x inc -627 if nwe < -3114
yr dec -446 if tc != 3180
cp dec -638 if xly <= 593
gy inc -97 if x <= 370
qk inc -769 if fi == 298
hfh dec 365 if bhp < -2906
j dec -533 if xly != 586
x dec 24 if uoc < -1910
fi inc 391 if tc <= 3182
yg inc -329 if b >= 440
ih dec -722 if t >= -1633
qk dec -517 if x >= 340
gif inc 404 if qt != 3225
x inc -52 if ih >= 2716
hm dec 759 if js < -3008
nwe dec -356 if yg >= -212
ioe inc -337 if x == 332
xly inc 610 if cp < -811
j dec 778 if j >= 178
qk dec -477 if ioe != -2231
gy inc 61 if b < 451
hfh inc -726 if gy <= 615
iyu inc -899 if itw < 3441
ih dec 980 if qt >= 3232
js inc 165 if yr != 1425
js dec -668 if yg == -207
b dec 584 if x < 339
qk dec -606 if cp < -806
gif inc 269 if hm <= -3437
js dec -472 if t > -1638
fi inc -853 if xly < 1197
gif inc -716 if iyu != 522
cp inc -536 if itw < 3440
hfh dec -848 if hfh <= 1594
hfh inc -135 if iyu <= 525
fi dec -789 if hfh > 1459
bhp dec -689 if uoc == -1919
hfh inc -507 if ioe <= -2221
yg dec 440 if ih > 2714
qk inc 99 if bhp != -2213
qk dec -330 if hfh >= 958
j inc -946 if ih != 2717
bhp dec -107 if gy < 619
cp dec -797 if t >= -1641
cp dec 615 if fi <= 228
nwe dec 502 if t != -1641
hm dec -58 if hm < -3433
t inc -324 if nwe < -3257
hfh dec 754 if gif > 1067
hm dec 2 if ih == 2709
bi dec -992 if fi == 230
t dec 646 if ih == 2709
qk dec -549 if nwe == -3261
tc dec -772 if nwe <= -3260
fi dec 640 if fi == 230
ioe dec 921 if tc >= 3957
tc dec 79 if tc <= 3964
tc inc 541 if yr == 1427
qk dec 331 if cp >= -546
ih dec 561 if itw >= 3438
itw inc 558 if qk != 119
hfh inc 809 if hfh >= 948
itw dec 981 if xly < 1197
ih dec -431 if js > -1708
yr dec -737 if hfh != 1754
tc inc 873 if ih != 2709
bhp dec -53 if hm != -3395
fi inc -207 if j >= -1541
bi inc -484 if itw >= 2449
js inc 548 if j > -1549
tc inc 336 if nwe != -3257
gy dec 39 if hm == -3386
gif inc 186 if itw > 2450
hm dec 671 if fi > -404
bi inc 950 if qt > 3227
gif inc 667 if fi > -414
x inc 380 if qk != 111
t dec -194 if js > -1172
yr inc -954 if itw < 2449
bhp inc 61 if j <= -1539
x inc -893 if bhp > -1997
ih dec -169 if iyu > 522
b dec 211 if cp >= -548
gif inc -278 if fi < -406
js dec 22 if t < -2404
ih dec -350 if nwe <= -3255
yg dec 198 if qk != 116
fi inc -466 if bi != 2091
yr inc -28 if yg == -405
cp dec 165 if xly < 1201
hm inc 875 if bhp >= -1995
qk inc 856 if gy > 565
hm inc -20 if iyu == 525
hfh dec -771 if itw > 2448
ioe dec 162 if fi == -874
yg inc -662 if ih < 3233
yr inc -214 if t == -2413
xly dec -251 if yg != -1065
itw inc 361 if ih < 3234
qk inc 206 if yg <= -1060
t inc 398 if ioe == -3146
qk dec 35 if b > 437
itw inc -841 if xly != 1445
hfh inc -461 if yr == 1922
hm dec -698 if itw != 1968
fi dec -98 if bhp < -1986
t inc 224 if hfh < 2080
j inc -767 if hm == -1833
uoc dec -817 if j > -2320
qt inc -801 if tc < 4761
hfh dec 597 if iyu == 525
itw dec 670 if cp != -709
ih inc 173 if bhp <= -1983
js inc 904 if hm == -1833
ih dec 913 if bi >= 2091
x inc 841 if itw < 1297
t dec 74 if gif == 1634
j dec 302 if yr != 1916
xly dec -815 if bi > 2092
qk inc -349 if x <= -168
xly inc 974 if fi >= -783
ih inc -251 if t == -1865
ioe dec 314 if nwe != -3253
fi dec -245 if ih < 2239
js dec -69 if js <= -272
yg inc -33 if j != -2623
xly dec -146 if ioe < -3451
bhp dec 757 if itw < 1309
qt inc -472 if yr >= 1918
xly dec 662 if hfh < 1477
nwe dec 66 if iyu <= 528
gy inc 47 if j > -2623
bhp dec -567 if bhp < -2746
gy inc 968 if nwe >= -3333
t dec -731 if iyu >= 519
gif dec -522 if j > -2621
qk inc -855 if tc > 4754
gif inc 125 if gif <= 2159
iyu dec -400 if hfh >= 1476
nwe dec -751 if qk >= -67
hm inc -645 if ioe != -3466
bi inc -235 if nwe >= -2571
b dec 981 if yr >= 1922
gy dec -452 if hm <= -2475
fi dec -532 if qt <= 1959
cp inc 612 if hm >= -2487
fi dec -455 if bi < 2097
t dec 80 if iyu <= 934
xly inc -694 if hm == -2478
uoc dec -759 if qk >= -59
ih dec 244 if yr >= 1918
tc inc 638 if ih > 1998
cp inc 1 if fi < 458
cp inc 454 if nwe > -2573
gy inc -134 if hm < -2471
yg dec 1 if iyu >= 919
js inc 91 if yg < -1108
yg inc 173 if ioe <= -3458
gy dec -760 if t != -1214
qk dec 816 if iyu < 920
gy dec 226 if itw < 1301
bi dec 704 if tc == 4755
iyu inc -661 if nwe != -2572
nwe inc -425 if qt < 1963
gy inc 197 if qk == -58
t inc 853 if nwe == -3001
js dec 603 if yg >= -928
j dec 708 if xly == 2026
hm dec 836 if b == -535
x dec -132 if itw > 1299
cp inc 587 if gif > 2278
t dec -715 if nwe <= -2994
b inc 63 if yr == 1922
tc inc 147 if bi > 1383
yg dec -24 if cp <= 476
gy dec -11 if j == -3322
bhp inc -488 if tc >= 4903
iyu inc 890 if hfh >= 1476
qk inc 224 if yr <= 1928
bhp dec 845 if itw > 1303
uoc inc 965 if qk != 166
fi dec -186 if cp == 482
t dec 388 if js >= -814
hm dec -157 if t < 357
hfh inc -353 if nwe == -3008
x dec 846 if cp >= 491
fi inc -436 if iyu <= 1154
qk dec 465 if bhp == -3027
yg inc -489 if t <= 354
js inc 809 if gif >= 2284
iyu inc 757 if hfh != 1479
uoc inc -765 if yg != -1415
bi inc 637 if nwe == -3005
ih dec 369 if ih == 1993"#;
