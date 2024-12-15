use itertools::Itertools;
use std::time::Instant;

fn main() {
    let input = include_str!("../../../inputs/input_15.txt");

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

fn solve(input: &str) -> usize {
    let (grid, instructions) = input.trim().split_once("\n\n").unwrap();

    let mut grid = grid
        .lines()
        .map(str::chars)
        .map(Itertools::collect_vec)
        .collect_vec();

    macro_rules! at {
        ($v:expr) => {
            grid[$v.1 as usize][$v.0 as usize]
        };
    }

    let instructions = instructions
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| match c {
            '<' => (-1, 0),
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            _ => unreachable!(""),
        })
        .collect_vec();

    let mut robot = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|&c| c == '@')
                .map(|x| (x as i32, y as i32))
        })
        .unwrap();

    // remove starting marker (@)
    at!(robot) = '.';

    for (dx, dy) in instructions {
        let (mut x, mut y) = robot;

        // ########
        // #.@.O..#
        // ## .O..#
        // #...O..#
        // #.#.O..#
        // #...O..#
        // #......#
        // ########

        let mut swap = vec![];
        let mut can_move = false;

        loop {
            let n = (x + dx, y + dy);
            let c = at!(n);
            if c == '#' {
                break;
            } else {
                swap.push(((x, y), n));
                (x, y) = n;

                if c == '.' {
                    can_move = true;
                    break;
                }
            }
        }

        if can_move {
            robot = (robot.0 + dx, robot.1 + dy);
            while let Some((a, b)) = swap.pop() {
                (at!(a), at!(b)) = (at!(b), at!(a));
            }
        }
    }

    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &c)| if c == 'O' { 100 * y + x } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

fn bonus(input: &str) -> usize {
    let (grid, instructions) = input.trim().split_once("\n\n").unwrap();

    let mut grid = grid
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|c| match c {
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    '@' => ['@', '.'],
                    _ => unreachable!("unknown char in grid: {c}"),
                })
                .collect_vec()
        })
        .collect_vec();

    macro_rules! at {
        ($v:expr) => {
            grid[$v.1 as usize][$v.0 as usize]
        };
    }

    let instructions = instructions
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| match c {
            '<' => (-1, 0, false),
            '^' => (0, -1, true),
            '>' => (1, 0, false),
            'v' => (0, 1, true),
            _ => unreachable!(""),
        })
        .collect_vec();

    let mut robot = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|&c| c == '@')
                .map(|x| (x as i32, y as i32))
        })
        .unwrap();

    // remove starting marker (@)
    at!(robot) = '.';

    for (dx, dy, vertical) in instructions {
        let mut front = vec![robot];

        let mut swaps = vec![];
        let mut can_move = false;

        // walk through the 'ramifications' of what it would mean to move in the given direction
        loop {
            let next_front = front.iter().map(|&(x, y)| (x + dx, y + dy)).collect_vec();
            let next_front_chars = next_front.iter().map(|&p| at!(p)).collect_vec();

            if next_front_chars.contains(&'#') {
                // we won't be able to make this move because we're hitting a wall
                break;
            } else {
                // record the swaps we'll be making
                swaps.extend(front.iter().enumerate().map(|(i, &p)| (p, next_front[i])));

                // ...and continue to the next frontline
                front = next_front
                    .into_iter()
                    .flat_map(|(nx, ny)| match (vertical, at!((nx, ny))) {
                        (true, '[') => vec![(nx, ny), (nx + 1, ny)],
                        (true, ']') => vec![(nx, ny), (nx - 1, ny)],
                        (_, '[') => vec![(nx, ny)],
                        (_, ']') => vec![(nx, ny)],
                        _ => vec![],
                    })
                    .unique()
                    .collect_vec();

                if next_front_chars.iter().all(|&c| c == '.') {
                    // if ALL of the next front is empty, we can move!
                    can_move = true;
                    break;
                }
            }
        }

        // perform the move
        if can_move {
            robot = (robot.0 + dx, robot.1 + dy);
            while let Some((a, b)) = swaps.pop() {
                (at!(a), at!(b)) = (at!(b), at!(a));
            }
        }
    }

    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &c)| if c == '[' { 100 * y + x } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

#[test]
fn test() {
    assert_eq!(
        solve(
            "
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"
        ),
        2028
    );

    assert_eq!(
        solve(
            "
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"
        ),
        10092
    );

    assert_eq!(
        bonus(
            "
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
"
        ),
        618
    );

    assert_eq!(
        bonus(
            "
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"
        ),
        9021
    );
}
