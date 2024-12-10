# Advent of Code 2024 🎄

See https://adventofcode.com/2024

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
