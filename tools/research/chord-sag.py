#!/usr/bin/env python3
"""How far under the planet the route in `goldberg-move` is drawn.

The prototype joins two territory centres with `gizmos.line`, which is a straight chord in
three dimensions, and floats both ends at `ABOVE` times the radius. A chord between two points
at radius `r` separated by an angle `theta` passes closest to the centre at `r * cos(theta / 2)`,
so it stays outside the unit sphere only while `theta` is small enough.

This computes both halves for `GP(2, 0)`: the neighbour separations the board actually has, and
the separation at which the chord breaks even.

The forty-two centres are derived rather than read from the crate, so this is a second
derivation and not a copy of the one it is checking. `sphere_tessellation::goldberg::seeds`
builds a Goldberg solid as the dual of a geodesic polyhedron, so its face centres are that
geodesic's vertices; for `GP(2, 0)` the geodesic is the 2-frequency class-I subdivision, whose
vertices are the icosahedron's twelve plus its thirty edge midpoints, all normalised.

Run: python tools/research/chord-sag.py
"""

import itertools
import math

# How far above the surface `goldberg-move` floats a disk and a route, as a multiple of the
# radius. `prototypes/goldberg-move/src/main.rs`, `ABOVE`.
ABOVE = 1.035

PHI = (1 + 5 ** 0.5) / 2


def normalise(v):
    length = math.sqrt(sum(c * c for c in v))
    return tuple(c / length for c in v)


def between(a, b):
    """The angle between two unit vectors, in degrees."""
    dot = max(-1.0, min(1.0, sum(x * y for x, y in zip(a, b))))
    return math.degrees(math.acos(dot))


def icosahedron():
    """The twelve vertices: cyclic permutations of (0, +-1, +-phi), normalised."""
    points = set()
    for one in (1, -1):
        for two in (1, -1):
            points.add(normalise((0.0, one * 1.0, two * PHI)))
            points.add(normalise((one * 1.0, two * PHI, 0.0)))
            points.add(normalise((two * PHI, 0.0, one * 1.0)))
    points = sorted(points)
    assert len(points) == 12, f"an icosahedron has twelve vertices, not {len(points)}"
    return points


def edges_of(points):
    """The thirty edges: the pairs at the one shortest separation."""
    pairs = sorted(
        itertools.combinations(range(len(points)), 2),
        key=lambda pair: between(points[pair[0]], points[pair[1]]),
    )
    shortest = between(points[pairs[0][0]], points[pairs[0][1]])
    edges = [
        pair
        for pair in pairs
        if abs(between(points[pair[0]], points[pair[1]]) - shortest) < 1e-9
    ]
    assert len(edges) == 30, f"an icosahedron has thirty edges, not {len(edges)}"
    return edges, shortest


def seeds():
    """The forty-two centres of `GP(2, 0)`."""
    corners = icosahedron()
    edges, _ = edges_of(corners)
    points = list(corners)
    for one, two in edges:
        points.append(
            normalise(tuple((x + y) / 2 for x, y in zip(corners[one], corners[two])))
        )
    assert len(points) == 42, f"GP(2, 0) has forty-two faces, not {len(points)}"
    return points


def separations(points):
    """Every neighbour separation on the board, in degrees.

    A `GP(2, 0)` centre has five neighbours if it came from an icosahedron vertex and six if it
    came from an edge midpoint, so the neighbour set is read off the gap in each point's sorted
    distances rather than by taking a fixed count.
    """
    found = []
    for at, point in enumerate(points):
        others = sorted(
            between(point, other) for elsewhere, other in enumerate(points) if elsewhere != at
        )
        # The gap: neighbours are close together and the next ring is far. Cut where the step
        # between successive distances is largest within the first ten.
        steps = [(others[i + 1] - others[i], i) for i in range(9)]
        _, last = max(steps)
        found.extend(others[: last + 1])
    return found


def main():
    points = seeds()
    found = separations(points)
    closest, furthest = min(found), max(found)

    # A chord between two points at radius ABOVE clears the unit sphere while
    # ABOVE * cos(theta / 2) >= 1.
    breaks_even = 2 * math.degrees(math.acos(1.0 / ABOVE))

    def midpoint(theta):
        return ABOVE * math.cos(math.radians(theta / 2))

    print(f"territories                     {len(points)}")
    print(f"neighbour separations counted   {len(found)}")
    print(f"neighbour separation, smallest  {closest:.3f} deg")
    print(f"neighbour separation, largest   {furthest:.3f} deg")
    print(f"chord at {ABOVE} breaks even at  {breaks_even:.3f} deg")
    print(f"chord midpoint radius, best     {midpoint(closest):.4f}")
    print(f"chord midpoint radius, worst    {midpoint(furthest):.4f}")
    print(f"deepest below the arc it draws  {ABOVE - midpoint(furthest):.4f} of a radius")

    under = [theta for theta in found if midpoint(theta) < 1.0]
    print()
    print(
        f"{len(under)} of {len(found)} neighbour separations put the chord under the sphere"
    )

    # The population, asserted rather than trusted: a geodesic polyhedron of triangulation
    # number T has 30T edges, and T = 4 for GP(2, 0) - so 120 edges, each counted from both
    # ends. If the gap heuristic above picked the wrong ring this number would not land.
    assert len(found) == 240, f"30 * 4 edges, counted from both ends, not {len(found)}"
    assert len(under) == len(found), "the finding is that it is all of them"


if __name__ == "__main__":
    main()
