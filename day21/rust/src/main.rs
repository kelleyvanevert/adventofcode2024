mod bonus;
mod first_part;

use std::time::Instant;

fn main() {
    let input = include_str!("../../../inputs/input_21.txt");

    time(|| {
        // ±9s
        // println!("First part: {}", first_part::solve(input));

        // <1ms
        println!("First part: {}", bonus::solve(input, 2));
    });

    time(|| {
        // <1ms
        println!("Bonus: {}", bonus::solve(input, 25));
    });
}

fn time<F>(mut f: F)
where
    F: FnMut(),
{
    let t0 = Instant::now();
    f();
    println!("  took {:?}", t0.elapsed());
}
