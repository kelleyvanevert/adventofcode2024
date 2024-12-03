# Advent of Code 2024 🎄

See https://adventofcode.com/2024

## Day 01

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

## Day 02

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
