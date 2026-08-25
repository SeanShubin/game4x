# Theory

[Documentation map](../README.md) · [Root README](../../README.md)

Background research the design depends on. These documents describe the problem space,
not our code — each should still be correct and useful if this project is abandoned
tomorrow. Implementation decisions appear here only as a labeled conclusion at the end of
a survey.

| Document                                               | Question it answers                                                                                           |
| ------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| [Splitting a sphere into regions](region-splitting.md) | How do you divide a sphere into hex-like regions, for any count from 1 to hundreds, without visible symmetry? |
| [Coloring regions](region-coloring.md)                 | How few colors are needed so no two adjacent regions match, and which actual colors should we use?            |

## How they fit together

The two are a pipeline, and they meet at a single interface. The wider survey they were
picked out of is [comparing region schemes](../notes/region-schemes.md), which is a
derived note rather than theory:

```
region splitting  --->  adjacency graph  --->  region coloring
   (geometry)          (integers only)         (graph algorithm)
```

Region splitting produces the adjacency graph. Region coloring consumes it and knows
nothing about geometry. The game logic sees only that same graph.

They also share a punchline. Exactly twelve pentagons are unavoidable on a sphere, which
is why a perfect hex grid is impossible — and those same twelve defects introduce the odd
cycles that push the minimum color count from three to four.
