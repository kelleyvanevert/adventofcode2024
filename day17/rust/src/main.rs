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
        println!("Bonus: {}", bonus(input));
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

fn execute(mut registers: Vec<i32>, instructions: &Vec<i32>) -> (Vec<i32>, Vec<i32>, usize) {
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

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lines = input.trim().lines().collect_vec();

    let registers = lines[0..3]
        .into_iter()
        .map(|line| line.split_once(": ").unwrap().1.parse::<i32>().unwrap())
        .collect_vec();

    let instructions = lines[4]
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
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

// fn bonus_par(input: &str) -> i32 {
//     let (registers, instructions) = parse(input);

//     let a = (0..)
//         .par_bridge()
//         .find_first(|&a| {
//             let mut registers = registers.clone();
//             registers[0] = a;
//             let (_, output) = execute(registers, &instructions);
//             return output == instructions;
//         })
//         .unwrap();

//     let mut registers = registers.clone();
//     registers[0] = a;

//     let (_, output) = execute(registers, &instructions);
//     assert_eq!(output, instructions);

//     return a;
// }

fn bonus(input: &str) -> i32 {
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
        bonus(
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
