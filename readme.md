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
