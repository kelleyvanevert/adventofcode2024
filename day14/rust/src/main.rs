use image::{ImageBuffer, Rgb};
use itertools::Itertools;
use std::{
    thread::sleep,
    time::{Duration, Instant},
};
use tuple::Map;

fn main() {
    let input = include_str!("../../../inputs/input_14.txt");

    time(|| {
        // <1ms
        println!("First part: {}", solve(input, 101, 103, 100));
    });

    // time(|| {
    //     // ±20ms
    //     println!("Bonus: {}", bonus(input));
    // });

    bonus(input);
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

fn solve(input: &str, w: i64, h: i64, t: i64) -> usize {
    let mut quadrants = [[0, 0], [0, 0]];

    input
        .trim()
        .lines()
        .map(|line| {
            // initial position, velocity
            let ((px, py), (vx, vy)) = line.split_once(" ").unwrap().map(|v| {
                v[2..]
                    .split_once(",")
                    .unwrap()
                    .map(|s| s.parse::<i64>().unwrap())
            });

            // final position
            let (fx, fy) = ((((px + vx * t) % w) + w) % w, (((py + vy * t) % h) + h) % h);

            // add to quadrant, if not in center row or column
            if (fx + 1) * 2 != w + 1 && (fy + 1) * 2 != h + 1 {
                let (qx, qy) = (fx / ((w + 1) / 2), fy / ((h + 1) / 2));
                quadrants[qy as usize][qx as usize] += 1;
            }
        })
        .collect_vec();

    quadrants[0][0] * quadrants[0][1] * quadrants[1][0] * quadrants[1][1]
}

fn bonus(input: &str) -> usize {
    const W: i64 = 101;
    const H: i64 = 103;
    const S: u32 = 120;
    const N: u32 = 10;
    const P: u32 = N * N;

    let robots = input
        .trim()
        .lines()
        .map(|line| {
            // initial position, velocity
            line.split_once(" ").unwrap().map(|v| {
                v[2..]
                    .split_once(",")
                    .unwrap()
                    .map(|s| s.parse::<i64>().unwrap())
            })
        })
        .collect_vec();

    for i in 0.. {
        let mut image = ImageBuffer::new(S as u32 * N, S as u32 * N);

        for t in (i * P)..((i + 1) * P) {
            let gx = t % N;
            let gy = (t / N) % N;

            for &((px, py), (vx, vy)) in robots.iter() {
                let (fx, fy) = (
                    (((px + vx * (t as i64)) % W) + W) % W,
                    (((py + vy * (t as i64)) % H) + H) % H,
                )
                    .map(|v| v as u32);

                *image.get_pixel_mut(gx * S + fx, gy * S + fy) = Rgb([255u8, 255u8, 255u8]);
            }
        }

        image.save(format!("aoc2024_day14_{}.png", i)).unwrap();
    }

    0
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
",
            11,
            7,
            100
        ),
        12
    );
}
