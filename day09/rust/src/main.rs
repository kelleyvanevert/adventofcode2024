#![feature(let_chains)]

use std::time::Instant;

fn main() {
    let input = include_str!("../../../inputs/input_09.txt");

    time(|| {
        // <1ms
        println!("First part: {}", solve(input));
    });

    time(|| {
        // ±400ms
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

fn solve(input: &str) -> usize {
    #[derive(Debug, Clone)]
    enum DiskItem {
        File { id: usize, size: usize, moved: bool },
        FreeSpace { space: usize },
    }

    // Step 1. parse
    // ===
    let mut disk = vec![];
    let mut file = true;
    let mut file_id = 0;
    for c in input.trim().chars() {
        if file {
            disk.push(DiskItem::File {
                id: file_id,
                size: c.to_digit(10).unwrap() as usize,
                moved: false,
            });
            file_id += 1;
            file = false;
        } else {
            disk.push(DiskItem::FreeSpace {
                space: c.to_digit(10).unwrap() as usize,
            });
            file = true;
        }
    }

    // Step 2. compact + compute checksum at the same time
    // ===
    let mut le = 0;
    let mut ri = disk.len() - 1;
    let mut i = 0;
    let mut checksum = 0;
    loop {
        if le == ri {
            if let DiskItem::File { id, size, .. } = &disk[le] {
                for j in 0..*size {
                    checksum += (i + j) * *id;
                }
                i += *size;
            }

            break;
        }

        match get_mut2(&mut disk, le, ri) {
            (DiskItem::File { id, size, .. }, _) => {
                for j in 0..*size {
                    checksum += (i + j) * *id;
                }
                i += *size;

                le += 1;
            }
            (_, DiskItem::FreeSpace { .. }) => {
                ri -= 1;
            }
            (DiskItem::FreeSpace { space }, DiskItem::File { id, size, .. }) => {
                if *space >= *size {
                    for j in 0..*size {
                        checksum += (i + j) * *id;
                    }
                    i += *size;

                    *space -= *size;
                    *size = 0;
                    ri -= 1;
                } else {
                    for j in 0..*space {
                        checksum += (i + j) * *id;
                    }
                    i += *space;

                    *size -= *space;
                    *space = 0;
                    le += 1;
                }
            }
        }
    }

    checksum
}

fn bonus(input: &str) -> usize {
    #[derive(Debug)]
    struct File {
        file_id: i64,
        i: usize,
        size: i64,
    }

    // Step 1. parse
    // ===
    let mut disk = vec![];
    let mut file = true;
    let mut file_id: i64 = 0;
    let mut files = vec![];
    for c in input.trim().chars() {
        if file {
            let size = c.to_digit(10).unwrap() as i64;
            files.push(File {
                file_id,
                i: disk.len(),
                size,
            });
            for _ in 0..size {
                disk.push(file_id);
            }
            file_id += 1;
            file = false;
        } else {
            for _ in 0..c.to_digit(10).unwrap() {
                disk.push(-1);
            }
            file = true;
        }
    }

    // Step 2. compact
    for file in files.into_iter().rev() {
        if let Some(move_to) = (0..file.i).find(|&i| {
            (i + file.size as usize) < disk.len()
                && (i..(i + file.size as usize)).all(|j| disk[j] == -1)
        }) {
            for j in 0..file.size {
                disk[file.i + j as usize] = -1;
                disk[move_to + j as usize] = file.file_id;
            }
        }
    }

    // Step 3. compute checksum
    disk.into_iter()
        .enumerate()
        .map(|(i, d)| if d >= 0 { i as i64 * d } else { 0 })
        .sum::<i64>() as usize
}

fn get_mut2<T>(v: &mut [T], le: usize, ri: usize) -> (&mut T, &mut T) {
    if le >= ri || ri >= v.len() {
        panic!("Cannot call get_mut2 with le >= ri || ri >= len");
    }

    let (first, second) = v.split_at_mut(le + 1);
    (&mut first[le], &mut second[ri - le - 1])
}

#[test]
fn test() {
    assert_eq!(solve("2333133121414131402",), 1928);

    assert_eq!(bonus("2333133121414131402",), 2858);
}
