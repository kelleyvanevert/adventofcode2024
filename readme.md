# Advent of Code 2024 🎄

See https://adventofcode.com/2024

## Links

- [Previous year's repo](https://github.com/kelleyvanevert/adventofcode2023)
- Friends also participating:
  - [Auke](https://github.com/Fadarrizz/advent-of-code/tree/main/2024/)

## Day 1

Solved in Google Sheets, because Chen said "Oh, I don't need to learn Python for this, I can just do it in Excel!"

Impression of the first part:

```
=ABS(A1-B1)
=SUM(C:C)
```

Impression of the second part:

```
=countif(B:B,A1)
=A1*C1
=SUM(D:D)
```

![](./impressions/day01b.png)

## Day 2

Solved the first part in Google Sheets, because I wanted to see how far I'd get with it :P But then quickly realized I'd already hit that end when I saw the bonus challenge..

Impression of the first part:

```
=AND(MAP($A11:$G11, $B11:$H11, LAMBDA(a, b, IF(b, a<b, true))))
=AND(MAP($A11:$G11, $B11:$H11, LAMBDA(a, b, IF(b, a>b, true))))
=AND(MAP($A11:$G11, $B11:$H11, LAMBDA(a, b, IF(b, AND(abs(a-b) >= 1, abs(a-b) <= 3), true))))
=and(or(J11:K11),L11)
=COUNTIF(M:M,true)
```

![](./impressions/day02a.png)

For the bonus part I turned to Gleam, because that seemed fun. However, while using the language, I started to get turned off, because they seem to have an inclination to follow "ergonomic functional-style" syntax ... and .. let's be honest, it's simply the same but less ergonomic than .. Adventlang! :D So then afterwards I also coded the first part in Adventlang for comparison. (The bonus would be a bit harder, I might need to add more stuff to the stdlib.)

Side-by-side:

![](./impressions/day02_gleam_vs_adventlang.png)

## Day 3, 4, 5

Fun and not too easy -- except day 5's bonus, which was actually quite the surpise for me. I don't know whether I was juist brainfarting, or it was actually so hard, but it took me quite some iterations :P

## Day 6

Today was a bad day for Adventlang..

- A found a bug that must have been in AL for a long time already:

  ```al
  let a = 1
  let u = (4, a)
  b = 2
  // u is now (4, 2) but should still be (4, 1)
  ```

  the simple solution is to sprinkle some `:clone`s throughout the code for now 😓

- There's a syntax / parsing bug, where it doesn't recognize this code:

  ```al
  if (a, b) == (1, 5) {
    ...
  }
  ```

- And, to put the nail in the coffin, it was suuper slow in computing, and eventually ate too much memory to be able to complete, the bonus part of today's challenge.

..so I took to Rust to solve the bonus part. My brute force solution is of course not the most beautiful way of solving it, and there'll most definitely be a way to solve the whole thing way faster, and then also in AL, but .. oh well..

_Update: I realized the kinda obvious first optimization that makes it a whole lot faster: only check obstruction placements along the path that the guard is going to walk anyway. This way, AL can get the job done in 6 minutes, and Rust in 400ms ☺️_
w

## Day 7

Fun and not too hard :)

## Day 8

Also fun and not too hard, except .. I did it in Rust, because Adventlang as yet another syntax problem 😓 — this apparently doesn't work:

```al
antennas[a] []= (x,y)
```

## Day 9

I'm embarrassed how long today took me! XD Let's maybe just say my brain was fried today... I don't know why I decided to start off the first part encoding the list of disk items semantically, instead of a just an offset-based vector like in the (eventual) bonus solution .., but it definitely led me into a whole world of imperative algorithmic pain. It worked though, and although it's ugly, I was kinda proud, and then continued to try the bonus that way as well, but then ran into various problems, including having read the problem statement wrong, before just codifying the disk vector more straightforwardly like in the example's ASCII art.. which then immediately led me to a working solution way faster. Ugh, ok, well ...

## Day 10

Nice :)

## Day 11

Simple, because I know by know, having done Advent of Code before, that this is a dynamic programming problem that can be super easily expressed with the `#[cached]` macro from the `cached` crate 😉 ±20 minutes to complete, and ±20 milliseconds to compute (the bonus).

## Day 12

The first part was a fun little exercise of the "flood gathering" algorithm (just named it that way myself). And then the bonus was a fun challenge that turned out just a bit more tricky than I though, because of my encoding of edges and the existence of diagonally adjacent regions.. See below, the most interestingly formatted + commented function I might have ever made in Rust :P

![](./impressions/day12.png)

## Day 13

Today was all about whether I can still muster up enough linear algebra maths 😅 I managed, though

![](./impressions/day_13.jpg)

## Day 14

Today's challenge started with a little speed-run, trying to solve it in 30 minutes before leaving with Chen to Markus + Priyanka to play Warhammer :P I was able to get the first part just in time, and then saw taht the bonus would require by-eye verification and probably using images and whatnot .. so I postponed to the next day :) The next day, I just went ahead and generated lots of png grids of each next 100 steps, and then verified them by eye, until we found it in the 75th grid! So cute!

![](./day14/rust/aoc2024_day14_75.png)

## Day 15

Today was pretty hard! The first part as OK, but then I got stuck on an algorithmic detail in the bonus part for an hour or two :O The generalization of the bonus part was fun. The way I set it up, instead of looking each subsequent next grid cell, while walking through the 'ramifications' of what it would mean to move in a certain direction, we need to now look at each subsequent next 'frontier' of grid cells. But I forgot to remove empty gird cells from the frontier, and then had to manually verify a lot of the moves taken to finally find the move where it went wrong, in the test example. Which was here:

```
[504] MOVING robot (7, 4) dir v (0, 1)...

Grid before:
####################
##[]...[].....[][]##
##[]...........[].##
##............[][]##
##.....@....[].[].##
##..##[]....[][]..##
##...[]...[]..[]..##
##.....[]..[].[][]##
##........[]......##
####################

Grid after:
####################
##[]...[].....[][]##
##[]...........[].##
##............[][]##
##..........[].[].##
##..##.@....[][]..##
##....[]..[]..[]..##
##...[]....[].[][]##
##.....[].[]......##
####################
```

Indeed, 504 moves in 😅 You can see it going wrong. As soon as I found this little bug, the algorithm worked and I was able to compute the answer, in about 3ms 💪
