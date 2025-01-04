use ast::Ast;
use itertools::Itertools;
use std::time::Instant;
use z3::*;

fn main() {
    let input = include_str!("../../../inputs/input_17.txt");

    time(|| {
        // <1ms
        println!("First part: {}", solve(input));
    });

    time(|| {
        // ±200ms (using Z3)
        println!("Bonus: {}", bonus_revisited(input));

        // println!("Bonus: {}", bonus_revisited_v2(input));
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

    let (_registers, output, _) = execute(registers, &instructions);

    // println!("registers: {_registers:?}");
    // println!("output: {output:?}");

    output.into_iter().map(|n| format!("{n}")).join(",")
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

fn bonus_revisited(input: &str) -> i64 {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Optimize::new(&ctx);

    let outputs = [2, 4, 1, 5, 7, 5, 4, 5, 0, 3, 1, 6, 5, 5, 3, 0];

    let zero = &ast::Int::from_i64(&ctx, 0);
    let one = &ast::Int::from_i64(&ctx, 1);
    let two = &ast::Int::from_i64(&ctx, 2);

    // t [i in 0..16] [0,1,2]
    let t = outputs
        .iter()
        .map(|&t| bits(t).map(|b| ast::Bool::from_bool(&ctx, b == 1)))
        .collect_vec();

    // a[i in 0..48]
    let mut a_bitarr = ast::Array::new_const(&ctx, "a", &Sort::int(&ctx), &Sort::int(&ctx));
    let a_bits = (0..90)
        .map(|i| {
            let bit = ast::Int::new_const(&ctx, format!("a_{i}"));
            if i < 48 {
                solver.assert(&ast::Bool::or(&ctx, &[&bit._eq(&zero), &bit._eq(&one)]));
            } else {
                solver.assert(&bit._eq(&zero));
            }
            a_bitarr = a_bitarr.store(&ast::Int::from_i64(&ctx, i as _), &bit); // assign ?
            bit
        })
        .collect_vec();

    // a
    let a = a_bits
        .iter()
        .enumerate()
        .fold(ast::Int::from_i64(&ctx, 0), |a, (i, bit)| {
            let n = ast::Int::from_u64(&ctx, 1 << (i + 1));
            ast::Int::add(&ctx, &[&a, &ast::Int::mul(&ctx, &[&n, bit])])
        });

    solver.minimize(&a);

    // s[i in 0..16] = 4*a_{3i+2} + 2*a_{3i+1} + a_{3i}
    let s = (0..16)
        .map(|i| {
            let neg_a_3i = ast::Int::sub(&ctx, &[&one, &a_bits[3 * i]]);
            let neg_a_3i2 = ast::Int::sub(&ctx, &[&one, &a_bits[3 * i + 2]]);

            let s = &neg_a_3i
                + &a_bits[3 * i + 1]
                + &a_bits[3 * i + 1]
                + &neg_a_3i2
                + &neg_a_3i2
                + &neg_a_3i2
                + &neg_a_3i2;

            // solver.assert()

            s
        })
        .collect_vec();

    for i in 0..16 {
        let k = ast::Int::from_i64(&ctx, i as i64 * 3);
        solver.assert(
            &a_bits[3 * i]
                ._eq(
                    &a_bitarr
                        .select(&ast::Int::add(&ctx, &[&k, &s[i], &zero]))
                        .as_int()
                        .unwrap(),
                )
                ._eq(&t[i][0]),
        );

        solver.assert(
            &a_bits[3 * i + 1]
                ._eq(
                    &a_bitarr
                        .select(&ast::Int::add(&ctx, &[&k, &s[i], &one]))
                        .as_int()
                        .unwrap(),
                )
                ._eq(&t[i][1]),
        );

        solver.assert(
            &a_bits[3 * i + 2]
                ._eq(
                    &a_bitarr
                        .select(&ast::Int::add(&ctx, &[&k, &s[i], &two]))
                        .as_int()
                        .unwrap(),
                )
                .not()
                ._eq(&t[i][2]),
        );
    }

    // println!();
    // println!("finding a model for:");
    // println!();
    // println!("{}", solver);

    let sol = solver.check(&[]);
    // println!("solvable? {:?}", sol);
    if sol == SatResult::Sat {
        let model = solver.get_model().unwrap();
        let num = a_bits
            .iter()
            .enumerate()
            .map(|(i, bit)| {
                let bitval = model.eval(bit, true).unwrap().as_i64().unwrap();
                // println!("a_{i} = {}", bitval);
                bitval << i
            })
            .sum::<i64>();

        // println!("final solution -> {num}");

        // check it
        // 109019930332937 is CORRECT but TOO HIGH
        // 109019930331546 is CORRECT and indeed the lowest! :)
        {
            // println!("let's check it...");
            let (registers, instructions) = parse(input);
            let mut registers = registers.clone();
            registers[0] = num;
            let (_, output, steps) = execute(registers, &instructions);
            // println!("  did it work? {output:?}");
            if output == instructions {
                return num;
                // println!("    YESYES!!");
            } else {
                panic!("found solution isn't correct :(");
            }
        }
    } else {
        panic!("can't find a solution")
    }
}

fn set_and_diff(bit: u8, check: u8) -> bool {
    bit < 2 && bit != check
}

fn bonus_revisited_v2(input: &str) -> i64 {
    type Bit = u8; // 0 | 1 | 2(unknown)
    type Pattern = Vec<u8>; // from least sign. to highest sign.

    let mut possible: Vec<Pattern> = vec![vec![]];

    let outputs = [2, 4, 1, 5, 7, 5, 4, 5, 0, 3, 1, 6, 5, 5, 3, 0];

    fn ensure(
        k: usize,
        pattern: &Pattern,
        a0: u8,
        a1: u8,
        a2: u8,
        t0: u8,
        t1: u8,
        t2: u8,
    ) -> Option<Pattern> {
        if set_and_diff(pattern[k + 0], a0)
            || set_and_diff(pattern[k + 1], a1)
            || set_and_diff(pattern[k + 2], a2)
        {
            return None;
        }

        let mut pattern = pattern.clone();

        pattern[k + 0] = a0;
        pattern[k + 1] = a1;
        pattern[k + 2] = a2;

        let s = (1 - a0) + 2 * a1 + 4 * (1 - a2);
        if t0 == 1 {
            // pattern[k + s]
            // todo
        }

        Some(pattern)
    }

    for (i, &n) in outputs[0..1].iter().enumerate() {
        let [t0, t1, t2] = bits(n);

        println!("Considering next output #{i} = {n} ...");
        println!("  atm there's {} possible patterns", possible.len());
        possible = possible
            .into_iter()
            .map(|mut pattern| {
                pattern.resize(3 * (i + 1), 2);
                pattern
            })
            .cartesian_product([0, 1, 2, 3, 4, 5, 6, 7])
            .flat_map(|(pattern, a_segment)| {
                let [a0, a1, a2] = bits(a_segment);
                ensure(i * 3, &pattern, a0, a1, a2, t0, t1, t2)
            })
            // .filter_map(|b| b)
            .collect_vec();
        println!("  -> now there's {} possible patterns", possible.len());
        for pattern in &possible {
            println!("    {pattern:?}");
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
}
