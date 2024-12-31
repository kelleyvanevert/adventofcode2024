use fxhash::FxHashMap;
use itertools::Itertools;
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let input = include_str!("../../../inputs/input_24.txt");

    time(|| {
        // <1ms
        println!("First part: {}", solve(input));
    });

    // time(|| {
    //     // ±2s
    //     println!("Bonus: {}", bonus(input));
    // });
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

fn solve(input: &'static str) -> usize {
    let (init, wires) = input.trim().split_once("\n\n").unwrap();

    let mut values = FxHashMap::default();

    for line in init.lines() {
        let (node, val) = line.split_once(": ").unwrap();
        values.insert(node, val == "1");
    }

    type Equation = (&'static str, &'static str, &'static str);

    let mut equations = FxHashMap::default();

    for wire in wires.lines() {
        let (a, op, b, _, out) = wire.split(" ").collect_tuple().unwrap();
        equations.insert(out, (a, op, b));
    }

    fn eval(
        key: &'static str,
        values: &mut FxHashMap<&'static str, bool>,
        equations: &FxHashMap<&'static str, Equation>,
    ) -> bool {
        match values.get(key) {
            Some(val) => *val,
            None => {
                let &(a, op, b) = equations.get(key).unwrap();
                let av = eval(a, values, equations);
                let bv = eval(b, values, equations);
                let val = match op {
                    "AND" => av && bv,
                    "OR" => av || bv,
                    "XOR" => av != bv,
                    _ => unreachable!(),
                };
                values.insert(key, val);
                val
            }
        }
    }

    equations
        .keys()
        .filter(|k| k.starts_with("z"))
        .sorted()
        .enumerate()
        .map(|(i, &z)| (eval(&z, &mut values, &equations) as usize) << i)
        .sum()
}

fn bonus(input: &str) -> usize {
    0
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"
        ),
        2024
    );
}
