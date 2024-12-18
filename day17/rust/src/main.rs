use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::time::Instant;

fn main() {
    let input = include_str!("../../../inputs/input_17.txt");

    time(|| {
        // <1ms
        println!("First part: {}", solve(input));
    });

    time(|| {
        // ±3ms
        println!("Bonus: {}", bonus_smart(input));
    });
}

// macro_rules! vprintln {
//     ($c:expr, $($x:tt)*) => { if $c { println!($($x)*); } }
// }

fn time<F>(mut f: F)
where
    F: FnMut(),
{
    let t0 = Instant::now();
    f();
    println!("  took {:?}", t0.elapsed());
}

fn execute(mut registers: Vec<i64>, instructions: &Vec<i64>) -> (Vec<i64>, Vec<i64>, usize) {
    macro_rules! combo {
        ($k:expr) => {
            match $k {
                0 | 1 | 2 | 3 => $k,
                4 => registers[0],
                5 => registers[1],
                6 => registers[2],
                _ => unreachable!("{} will not happen", $k),
            }
        };
    }

    let mut ip = 0;
    let mut steps = 0;

    let mut output = vec![];

    while ip < instructions.len() {
        steps += 1;
        match instructions[ip].clone() {
            // adv
            0 => {
                registers[0] = registers[0] / (1 << combo!(instructions[ip + 1]));
                ip += 2;
            }
            // bxl
            1 => {
                registers[1] = registers[1] ^ instructions[ip + 1];
                ip += 2;
            }
            // bst
            2 => {
                registers[1] = combo!(instructions[ip + 1]) % 8;
                ip += 2;
            }
            // jnz
            3 => {
                if registers[0] != 0 {
                    ip = instructions[ip + 1] as usize;
                } else {
                    ip += 2;
                }
            }
            // bxc
            4 => {
                registers[1] = registers[1] ^ registers[2];
                ip += 2;
            }
            // out
            5 => {
                output.push(combo!(instructions[ip + 1]) % 8);
                ip += 2;
            }
            // bdv
            6 => {
                registers[1] = registers[0] / (1 << combo!(instructions[ip + 1]));
                ip += 2;
            }
            // cdv
            7 => {
                registers[2] = registers[0] / (1 << combo!(instructions[ip + 1]));
                ip += 2;
            }
            _ => unreachable!(""),
        };
    }

    (registers, output, steps)
}

fn parse(input: &str) -> (Vec<i64>, Vec<i64>) {
    let lines = input.trim().lines().collect_vec();

    let registers = lines[0..3]
        .into_iter()
        .map(|line| line.split_once(": ").unwrap().1.parse::<i64>().unwrap())
        .collect_vec();

    let instructions = lines[4]
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect_vec();

    (registers, instructions)
}

fn solve(input: &str) -> String {
    let (registers, instructions) = parse(input);

    let (registers, output, _) = execute(registers, &instructions);

    println!("registers: {registers:?}");
    println!("output: {output:?}");

    output.into_iter().map(|n| format!("{n}")).join(",")
}

fn bonus_brute_parallel(input: &str) -> i64 {
    let (registers, instructions) = parse(input);

    let a = (0..)
        .par_bridge()
        .find_first(|&a| {
            let mut registers = registers.clone();
            registers[0] = a;
            let (_, output, _) = execute(registers, &instructions);
            return output == instructions;
        })
        .unwrap();

    // For some reason `find_first` doesn't REALLY give the first, it's off by about the amount of processors I have (XD), and all those answers seem kinda possible..

    // Proof that the answer, whether lowest or not, is indeed valid
    {
        let mut registers = registers.clone();
        registers[0] = a;

        let (_, output, _) = execute(registers, &instructions);
        assert_eq!(output, instructions);
    }

    // But then, let's find the actually lowest one
    let num_processors_in_kelleys_laptop = 8;
    for a in (a - num_processors_in_kelleys_laptop)..(a + 1) {
        let mut registers = registers.clone();
        registers[0] = a;
        let (_, output, _) = execute(registers, &instructions);
        if output == instructions {
            return a;
        }
    }

    unreachable!("should have been found by now")
}

fn bonus_brute(input: &str) -> i64 {
    let (registers, instructions) = parse(input);

    for a in 0.. {
        let mut registers = registers.clone();
        registers[0] = a;
        let (_, output, steps) = execute(registers, &instructions);
        // println!("{a} ({steps}) -> {output:?}");
        if output == instructions {
            return a;
        }
    }

    unreachable!("sdf")
}

fn bits(n: i64) -> [u8; 3] {
    match n {
        0 => [0, 0, 0],
        1 => [1, 0, 0],
        2 => [0, 1, 0],
        3 => [1, 1, 0],
        4 => [0, 0, 1],
        5 => [1, 0, 1],
        6 => [0, 1, 1],
        7 => [1, 1, 1],
        _ => unreachable!("won't get bits of {n}"),
    }
}

// All too high
// Trying 0...
// Trying 1...
// Trying 2...
// found num 824750618133311
// Trying 3...
// found num 805081683494790
// Trying 4...
// found num 1196890644943729
// Trying 5...
// found num 3112607301359880
// Trying 6...
// found num 6225173949989835
// Trying 7...
// found num 23146398892914690
fn bonus_smart(input: &str) -> i64 {
    let (registers, instructions) = parse(input);

    enum Op {
        Same,
        Diff,
    }

    type Constraint = (i64, i64, Op);

    'find_k: for k in 0..8 {
        println!("\nTrying {k}...");

        // !a2  a1  !a0  =  k  =  k2  k1  k0
        let [k0, k1, k2] = bits(k);

        let mut constraints: Vec<Constraint> = instructions
            .iter()
            .enumerate()
            .flat_map(|(i, &instr)| {
                let [s0, s1, s2] = bits(instr);
                [
                    (
                        i as i64 * 3,
                        k + 0 + i as i64 * 3,
                        if s0 == 1 { Op::Diff } else { Op::Same },
                    ),
                    (
                        i as i64 * 3,
                        k + 1 + i as i64 * 3,
                        if s1 == 1 { Op::Same } else { Op::Diff },
                    ),
                    (
                        i as i64 * 3,
                        k + 2 + i as i64 * 3,
                        if s2 == 1 { Op::Diff } else { Op::Same },
                    ),
                ]
            })
            .collect_vec();

        // !a2  a1  !a0  =  k  =  k2  k1  k0
        constraints.push((0, 1, if k0 == k1 { Op::Diff } else { Op::Same }));
        constraints.push((1, 2, if k1 == k2 { Op::Diff } else { Op::Same }));
        constraints.push((0, 2, if k0 == k2 { Op::Same } else { Op::Diff }));

        let num_bits = (constraints
            .iter()
            .map(|c| c.0)
            .max()
            .unwrap()
            .max(constraints.iter().map(|c| c.1).max().unwrap())
            + 1) as usize;

        let mut a = vec![None; num_bits];
        a[0] = Some(1 - k0);

        let mut todo = vec![0i64];
        while let Some(i) = todo.pop() {
            let bit = a[i as usize].unwrap();
            while let Some(r) = constraints.iter().position(|c| c.0 == i || c.1 == i) {
                let c = constraints.remove(r);
                let j = if i == c.0 { c.1 } else { c.0 };
                match (a[j as usize], c.2) {
                    (None, Op::Same) => {
                        a[j as usize] = Some(bit);
                        if !todo.contains(&j) {
                            todo.push(j);
                        }
                    }
                    (None, Op::Diff) => {
                        a[j as usize] = Some(1 - bit);
                        if !todo.contains(&j) {
                            todo.push(j);
                        }
                    }
                    (Some(other_bit), Op::Same) if other_bit == bit => {}
                    (Some(other_bit), Op::Diff) if other_bit != bit => {}
                    _ => {
                        continue 'find_k;
                    }
                }
            }

            if todo.len() == 0 {
                if let Some(i) =
                    a.iter()
                        .enumerate()
                        .rev()
                        .find_map(|(i, &b)| if b == None { Some(i) } else { None })
                {
                    a[i] = Some(0); // lower is better
                    todo.push(i as i64);
                }
            }
        }

        // there can't still be unapplied constraints, right ?
        assert_eq!(constraints.len(), 0);

        let num_binary = a
            .iter()
            .rev()
            .map(|&b| if b.unwrap() == 0 { '0' } else { '1' })
            .collect::<String>();

        let num = i64::from_str_radix(
            &a.into_iter()
                .rev()
                .map(|b| if b.unwrap() == 0 { '0' } else { '1' })
                .collect::<String>(),
            2,
        )
        .unwrap();

        println!("found num `{}` = {}", num_binary, num);

        // check it
        {
            let mut registers = registers.clone();
            registers[0] = num;
            let (_, output, steps) = execute(registers, &instructions);
            println!("  did it work? {output:?}");
            if output == instructions {
                println!("    YESYES!!");
            } else {
                println!("    no :(");
            }
        }
    }

    0
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"
        ),
        "4,6,3,5,6,3,5,2,1,0"
    );

    assert_eq!(
        solve(
            "
Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"
        ),
        "0,3,5,4,3,0"
    );

    assert_eq!(
        solve(
            "
Register A: 117444
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"
        ),
        "0,3,5,4,3,0"
    );

    assert_eq!(
        bonus_brute(
            "
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"
        ),
        117440
    );

    assert_eq!(
        bonus_brute_parallel(
            "
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"
        ),
        117440
    );
}

/*

[
    a,
    _,
    _
]
2,4,1,5,7,5,4,5,0,3,1,6,5,5,3,0
^

r[1] <- r[0] % 8

[
    a,
    a%8,
    _
]
2,4,1,5,7,5,4,5,0,3,1,6,5,5,3,0
    ^

r[1] <- r[1] ^ `101`

[
    a,
    (a%8) ^ `101`,
    _
]
2,4,1,5,7,5,4,5,0,3,1,6,5,5,3,0
        ^

r[2] <- r[0] / (1 << r[1])

[
    a,
    (a%8) ^ `101`,
    a / (1 << ((a%8) ^ `101`))
]
2,4,1,5,7,5,4,5,0,3,1,6,5,5,3,0
            ^

r[1] <- r[1] ^ r[2]

[
    a,
    ((a%8) ^ `101`) ^ (a / (1 << ((a%8) ^ `101`))),
    a / (1 << ((a%8) ^ `101`))
]
2,4,1,5,7,5,4,5,0,3,1,6,5,5,3,0
                ^

r[0] <- r[0] / (1 << 3) = r[0] / 8

[
    a / 8,
    ((a%8) ^ `101`) ^ (a / (1 << ((a%8) ^ `101`))),
    a / (1 << ((a%8) ^ `101`))
]
2,4,1,5,7,5,4,5,0,3,1,6,5,5,3,0
                    ^

r[1] <- r[1] ^ `110`

[
    a / 8,
    (((a%8) ^ `101`) ^ (a / (1 << ((a%8) ^ `101`)))) ^ `110`,
    a / (1 << ((a%8) ^ `101`))
]
2,4,1,5,7,5,4,5,0,3,1,6,5,5,3,0
                        ^

output.push(r[2] % 8)

[
    a / 8,
    (((a%8) ^ `101`) ^ (a / (1 << ((a%8) ^ `101`)))) ^ `110`,
    a / (1 << ((a%8) ^ `101`))
]
OUTPUT ((((a%8) ^ `101`) ^ (a / (1 << ((a%8) ^ `101`)))) ^ `110`) % 8
2,4,1,5,7,5,4,5,0,3,1,6,5,5,3,0
                            ^

jump to start if r0 != 0, else DONE


===


WHILE a != 0 {
    OUTPUT `!(a(k+2) ^ !a2)  !(a(k+1) ^ a1)  (ak ^ !a0)`
      where k = `!a2  a1  !a0`

    a = a/8     (same as)    a = a >> 3
}


                                a                             =                     ... a2              a1            a0
                                a%8                           =                         a2              a1            a0
                              ((a%8) ^ `101`))                =                        !a2              a1           !a0      =  k
                         1 << ((a%8) ^ `101`))                =  1 ~~~~~~~~~~~~~~~~~~~~~~~ k x 000's ~~~~~~~~~~~~~~~~~~~~~~~
                   (a / (1 << ((a%8) ^ `101`)))               =    ...   a(k+3)      a(k+2)           a(k+1)         ak
 ((a%8) ^ `101`) ^ (a / (1 << ((a%8) ^ `101`))))              =          ...      (a(k+2) ^ !a2)   (a(k+1) ^ a1)  (ak ^ !a0)
 ((a%8) ^ `101`) ^ (a / (1 << ((a%8) ^ `101`)))) ^ `110`      =          ...     !(a(k+2) ^ !a2)  !(a(k+1) ^ a1)  (ak ^ !a0)
(((a%8) ^ `101`) ^ (a / (1 << ((a%8) ^ `101`)))) ^ `110`) % 8 =                  !(a(k+2) ^ !a2)  !(a(k+1) ^ a1)  (ak ^ !a0)
                                                              =                   (a(k+2) <> a2)   (a(k+1) = a1)  (ak =  a0)


k=0   k=`000`   a = `..........101`
k=1   k=`001`   a = `..........100`
k=2   k=`010`   a = `..........111`
k=3   k=`011`   a = `..........110`
k=4   k=`100`   a = `..........001`
k=5   k=`101`   a = `..........000`
k=6   k=`110`   a = `..........011`
k=7   k=`111`   a = `..........010`


                    // generally:

                    enum Op { Same, Diff };

                    type Constraint = (i64, i64, Op);

                    'find_k: for k in 0..8 {
                        let kb = bits(k); // !a2  a1  !a0  =  k

                        let mut constraints = instructions.enumerate().flat_map(|(i, instr)| {
                            let [s0, s1, s2] = bits(instr);
                            [
                                (i*3, k + 0 + i*3, if s0 { Diff } else { Same }),
                                (i*3, k + 1 + i*3, if s0 { Same } else { Diff }),
                                (i*3, k + 2 + i*3, if s0 { Diff } else { Same }),
                            ]
                        }).collect();

                        // `!a2  a1  !a0`  =  k
                        constraints.push((0, 1, if kb[0] == kb[1] { Diff } else { Same }));
                        constraints.push((1, 2, if kb[1] == kb[2] { Diff } else { Same }));
                        constraints.push((0, 2, if kb[0] == kb[2] { Same } else { Diff }));

                        let num = [...constraints.map(|c| c.0), ...constraints.map(|c| c.1)].max().unwrap() + 1;

                        let mut a = vec![None; num];
                        a[0] = Some(!kb[0]);

                        let mut todo = vec![0];
                        while let Some(i) = todo.pop() {
                            let bit = a[i].unwrap();
                            let apply = constraints.retain(|c| c.0 != i && c.1 != i);
                            for c in apply {
                                let j = if i == c.0 { c.1 } else { c.0 };
                                match (a.get(j), c.2) {
                                    (None, Op::Same) => {
                                        a[j] = Some(bit)
                                        todo.push(j)
                                    }
                                    (None, Op::Diff) => {
                                        a[j] = Some(1-bit)
                                        todo.push(j)
                                    }
                                    (Some(other_bit), Op::Same) if other_bit == bit => {}
                                    (Some(other_bit), Op::Diff) if other_bit != bit => {}
                                    _ => {
                                        continue 'find_k;
                                    }
                                }
                            }

                            if todo.len() == 0 {
                                if let Some(i) = a.max_position(|b| b == None) {
                                    a[i] = 0; // lower is better
                                    todo.push(i);
                                }
                            }
                        }

                        // there can't still be unapplied constraints, right ?
                        assert_eq!(constraints.len(), 0);

                        println!("found num {}", a.reverse().join(""));
                    }


suppose `(a(k+2) <> a2)  (a(k+1) = a1)  (ak = a0)` == `010` == 2
   then a(k+0)  = !a0
        a(k+1)  =  a1
        a(k+2)  =  a2


===2,4,1,5,7,5,4,5,0,3,1,6,5,5,3,0

suppose `(a(k+2) <> a2)  (a(k+1) = a1)  (ak = a0)` == `010` == 2
   then a(k+0)  = !a0
        a(k+1)  =  a1
        a(k+2)  =  a2

             2         1
    987654321098765432109876543210
k=0   k=`000`   a = `..........101` -> impossible
k=1   k=`001`   a = `..........100` -> impossible
k=2   k=`010`   a = `..........111` -> impossible
k=3   k=`011`   a = `.......111110` ok
k=4   k=`100`   a = `......000.001` ok
k=5   k=`101`   a = `.....001..000` ok
k=6   k=`110`   a = `....010...011` ok
k=7   k=`111`   a = `...011....010` ok



===4,1,5,7,5,4,5,0,3,1,6,5,5,3,0

suppose `(a(k+5) <> a5)  (a(k+4) = a4)  (a(k+3) = a3)` == `100` == 4
   then a(k+3)   = !a3
        a(k+4)   = !a4
        a(k+5)   = !a5

             2         1
    987654321098765432109876543210
k=3   k=`011`   a = `....000111110` ok
k=4   k=`100`   a = `...11.000.001` ok+   a3 <> a7
k=5   k=`101`   a = `..0..001..000` ok+   a3 <> a8, a4 <> a9
k=6   k=`110`   a = `....010...011` ok+   a3 <> a9, a4 <> a10, a5 <> a11
k=7   k=`111`   a = `...011....010` ok+   a3 <> a10, a4 <> a11, a5 <> a12



===1,5,7,5,4,5,0,3,1,6,5,5,3,0

suppose `(a(k+8) <> a8)  (a(k+7) = a7)  (a(k+6) = a6)` == `001` == 1
   then a(k+6)   =  a6
        a(k+7)   = !a7
        a(k+8)   =  a8

                  2         1
         987654321098765432109876543210
k=3   k=`011`   a = `......010000111110` ok
k=4   k=`100`   a = `.....1.011.000.001` ok+   a3 <> a7, a7 <> a11
k=5   k=`101`   a = `.....100..001..000` ok+   a3 <> a8, a4 <> a9, a8 = a13
k=6   k=`110`   a = `...000...010...011` ok+   a3 <> a9, a4 <> a10, a5 <> a11
k=7   k=`111`   a = `..10....011....010` ok+   a3 <> a10, a4 <> a11, a5 <> a12, a6 = a13



===5,7,5,4,5,0,3,1,6,5,5,3,0

suppose `(a(k+11) <> a11)  (a(k+10) = a10)  (a(k+9) = a9)` == `101` == 5
   then a(k+9)   =   a9
        a(k+10)  = !a10
        a(k+11)  = !a11

             3         2         1
    9876543210987654321098765432109876543210
k=3   k=`011`   a = `........100010000111110` ok
k=4   k=`100`   a = `........111.011.000.001` ok+   a3 <> a7, a7 <> a11, a11 <> a15
k=5   k=`101`   a = `......11..100..001..000` ok+   a3 <> a8, a4 <> a9, a8 = a13, a9 = a14
k=6   k=`110`   a = `........000...010...011` ok+   a3 <> a9, a4 <> a10, a5 <> a11, a9 = a15, a10 <> a16, a11 <> a17
k=7   k=`111`   a = `......010....011....010` ok+   a3 <> a10, a4 <> a11, a5 <> a12, a6 = a13, a10 <> a17, a11 <> a18



===7,5,4,5,0,3,1,6,5,5,3,0

suppose `(a(k+14) <> a14)  (a(k+13) = a13)  (a(k+12) = a12)` == `111` == 7
   then a(k+12)   =  a12
        a(k+13)   =  a13
        a(k+14)   = !a14

...



===5,4,5,0,3,1,6,5,5,3,0

suppose `(a(k+17) <> a17)  (a(k+16) = a16)  (a(k+15) = a15)` == `101` == 5
   then a(k+15)   =  a15
        a(k+16)   = !a16
        a(k+17)   = !a17



===4,5,0,3,1,6,5,5,3,0

suppose `(a(k+20) <> a20)  (a(k+19) = a19)  (a(k+18) = a18)` == `100` == 4
   then a(k+18)   = !a18
        a(k+19)   = !a19
        a(k+20)   = !a20



===5,0,3,1,6,5,5,3,0

suppose `(a(k+23) <> a23)  (a(k+22) = a22)  (a(k+21) = a21)` == `101` == 5
   then a(k+21)   =  a21
        a(k+22)   = !a22
        a(k+23)   = !a23



===0,3,1,6,5,5,3,0

suppose `(a(k+26) <> a26)  (a(k+25) = a25)  (a(k+24) = a24)` == `000` == 0
   then a(k+24)   = !a24
        a(k+25)   = !a25
        a(k+26)   =  a26



===3,1,6,5,5,3,0

suppose `(a(k+29) <> a29)  (a(k+28) = a28)  (a(k+27) = a27)` == `011` == 3
   then a(k+27)   =  a27
        a(k+28)   =  a28
        a(k+29)   =  a29



===1,6,5,5,3,0

suppose `(a(k+32) <> a32)  (a(k+31) = a31)  (a(k+30) = a30)` == `001` == 1
   then a(k+30)   =  a30
        a(k+31)   = !a31
        a(k+32)   =  a32



===6,5,5,3,0

suppose `(a(k+35) <> a35)  (a(k+34) = a34)  (a(k+33) = a33)` == `110` == 6
   then a(k+33)   = !a33
        a(k+34)   =  a34
        a(k+35)   = !a35



===5,5,3,0

suppose `(a(k+38) <> a38)  (a(k+37) = a37)  (a(k+36) = a36)` == `101` == 5
   then a(k+36)   =  a36
        a(k+37)   = !a37
        a(k+38)   = !a38



===5,3,0

suppose `(a(k+41) <> a41)  (a(k+40) = a40)  (a(k+39) = a39)` == `101` == 5
   then a(k+39)   =  a39
        a(k+40)   = !a40
        a(k+41)   = !a41



===3,0

suppose `(a(k+44) <> a44)  (a(k+43) = a43)  (a(k+42) = a42)` == `011` == 3
   then a(k+42)   =  a42
        a(k+43)   =  a43
        a(k+44)   =  a44



===0

suppose `(a(k+47) <> a47)  (a(k+46) = a46)  (a(k+45) = a45)` == `000` == 0
   then a(k+45)   = !a45
        a(k+46)   = !a46
        a(k+47)   =  a47



*/
