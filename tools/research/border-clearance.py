#!/usr/bin/env python3
"""Does the chord pass under the drawn surface, and over which borders?

`X-38` measured that a straight chord between two territory centres, floated at `ABOVE`, passes
under radius 1.0 on every step of `GP(2, 0)`. It said plainly what it had not measured - whether
the chord also passes under the **drawn** surface - and named `PlanetMesh::deepest()` as the
instrument.

The code lane answered at `fec232b` and reported that `deepest()` is the wrong instrument: it is
the deepest point *anywhere*, which is the middle of the widest panel, while a step's low point
is over the **border** between two territories. This re-derives that answer by a second route,
because a claim that arrives finished is the one to re-derive.

The panels are a triangle fan from each centre out to its corners, all of which sit at radius 1,
so the drawn surface over a border is the straight segment joining that border's two corners.
The corners of `GP(2, 0)` are the centroids of the geodesic's eighty triangles, normalised; the
border between two neighbouring territories is the segment joining the two corners whose
triangles both carry those territories.

Run: python tools/research/border-clearance.py
"""

import itertools
import math

# `prototypes/goldberg-move/src/main.rs`, `ABOVE`.
ABOVE = 1.035

PHI = (1 + 5 ** 0.5) / 2


def normalise(v):
    length = math.sqrt(sum(c * c for c in v))
    return tuple(c / length for c in v)


def add(a, b):
    return tuple(x + y for x, y in zip(a, b))


def scale(v, k):
    return tuple(x * k for x in v)


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def length(v):
    return math.sqrt(dot(v, v))


def between(a, b):
    return math.degrees(math.acos(max(-1.0, min(1.0, dot(a, b)))))


def icosahedron():
    points = set()
    for one in (1, -1):
        for two in (1, -1):
            points.add(normalise((0.0, one * 1.0, two * PHI)))
            points.add(normalise((one * 1.0, two * PHI, 0.0)))
            points.add(normalise((two * PHI, 0.0, one * 1.0)))
    points = sorted(points)
    assert len(points) == 12, f"twelve vertices, not {len(points)}"
    return points


def edges_and_faces(points):
    """The thirty edges and twenty faces, found by the one shortest separation."""
    pairs = sorted(
        itertools.combinations(range(len(points)), 2),
        key=lambda pair: between(points[pair[0]], points[pair[1]]),
    )
    side = between(points[pairs[0][0]], points[pairs[0][1]])
    edges = [p for p in pairs if abs(between(points[p[0]], points[p[1]]) - side) < 1e-9]
    assert len(edges) == 30, f"thirty edges, not {len(edges)}"

    joined = set(edges) | {(b, a) for a, b in edges}
    faces = [
        triple
        for triple in itertools.combinations(range(len(points)), 3)
        if all(pair in joined for pair in itertools.combinations(triple, 2))
    ]
    assert len(faces) == 20, f"twenty faces, not {len(faces)}"
    return edges, faces


def geodesic():
    """The 2-frequency class-I geodesic: 42 vertices and its 80 triangles.

    Its vertices are `GP(2, 0)`'s territory centres; its triangles' centroids are that solid's
    corners.
    """
    corners = icosahedron()
    edges, faces = edges_and_faces(corners)

    points = list(corners)
    middle = {}
    for one, two in edges:
        middle[frozenset((one, two))] = len(points)
        points.append(normalise(add(corners[one], corners[two])))
    assert len(points) == 42, f"forty-two centres, not {len(points)}"

    triangles = []
    for a, b, c in faces:
        ab = middle[frozenset((a, b))]
        bc = middle[frozenset((b, c))]
        ca = middle[frozenset((c, a))]
        triangles += [(a, ab, ca), (b, bc, ab), (c, ca, bc), (ab, bc, ca)]
    assert len(triangles) == 80, f"twenty faces times four, not {len(triangles)}"
    return points, triangles


def where_the_ray_meets(segment_a, segment_b, direction):
    """Where a ray from the origin along `direction` crosses the segment, as a radius.

    The segment and the ray are coplanar through the origin, so this solves
    `a + t (b - a) = k * direction` for `t` in the plane those three span.
    """
    edge = tuple(y - x for x, y in zip(segment_a, segment_b))
    # Normal of the plane containing the origin and the ray, taken against the segment.
    # Solve for t such that (a + t*edge) is parallel to direction: cross(a + t*edge, d) = 0.
    # Use two independent components of the cross product.
    def cross(u, v):
        return (
            u[1] * v[2] - u[2] * v[1],
            u[2] * v[0] - u[0] * v[2],
            u[0] * v[1] - u[1] * v[0],
        )

    numerator = cross(segment_a, direction)
    denominator = cross(edge, direction)
    # Pick the component with the largest magnitude in the denominator, for conditioning.
    at = max(range(3), key=lambda i: abs(denominator[i]))
    assert abs(denominator[at]) > 1e-12, "the segment is parallel to the ray"
    t = -numerator[at] / denominator[at]
    meeting = add(segment_a, scale(edge, t))
    return t, length(meeting)


def circumcentre(a, b, c):
    """The point of the sphere equidistant from three sites - a Voronoi vertex.

    Not the centroid. The two agree only for an equilateral triangle, and the 2-frequency
    geodesic has two shapes: the corner triangles touching an icosahedron vertex are isoceles.
    Using centroids put this script's numbers 0.003 away from the code lane's on the first run,
    with the counts already agreeing - which is what said the shape was right and the points
    were not.
    """
    edge_one = tuple(y - x for x, y in zip(a, b))
    edge_two = tuple(y - x for x, y in zip(a, c))
    normal = (
        edge_one[1] * edge_two[2] - edge_one[2] * edge_two[1],
        edge_one[2] * edge_two[0] - edge_one[0] * edge_two[2],
        edge_one[0] * edge_two[1] - edge_one[1] * edge_two[0],
    )
    normal = normalise(normal)
    return normal if dot(normal, a) > 0 else scale(normal, -1.0)


def main():
    centres, triangles = geodesic()

    corners = [circumcentre(centres[a], centres[b], centres[c]) for a, b, c in triangles]
    assert len(corners) == 80, f"eighty corners, not {len(corners)}"
    for corner, (a, b, c) in zip(corners, triangles):
        one, two, three = (between(corner, centres[at]) for at in (a, b, c))
        assert abs(one - two) < 1e-9 and abs(two - three) < 1e-9, "a Voronoi vertex is equidistant"

    # Which triangles carry each territory.
    carried = {at: set() for at in range(len(centres))}
    for face, triple in enumerate(triangles):
        for at in triple:
            carried[at].add(face)

    # A border is the pair of corners whose triangles both carry the two territories.
    borders = []
    for one, two in itertools.combinations(range(len(centres)), 2):
        shared = sorted(carried[one] & carried[two])
        if len(shared) != 2:
            continue
        borders.append((one, two, corners[shared[0]], corners[shared[1]]))
    assert len(borders) == 120, f"30T edges with T = 4 is 120, not {len(borders)}"

    rows = []
    for one, two, corner_a, corner_b in borders:
        separation = between(centres[one], centres[two])
        # The chord's low point, and the direction it lies along.
        low = ABOVE * math.cos(math.radians(separation / 2))
        direction = normalise(add(centres[one], centres[two]))
        t, surface = where_the_ray_meets(corner_a, corner_b, direction)
        assert -1e-9 <= t <= 1 + 1e-9, f"the crossing left the border at t = {t}"
        rows.append((separation, low, surface, low - surface))

    under = [row for row in rows if row[3] < 0]
    over = [row for row in rows if row[3] > 0]

    print(f"borders                          {len(rows)}   (directed: {2 * len(rows)})")
    print(f"lowest surface over any border   {min(row[2] for row in rows):.4f}")
    print(f"chord low, best / worst          {max(r[1] for r in rows):.4f} / {min(r[1] for r in rows):.4f}")
    print()
    print(f"chord UNDER its border           {len(under)}   (directed: {2 * len(under)})")
    print(f"chord OVER its border            {len(over)}   (directed: {2 * len(over)})")
    if under:
        print(f"  deepest under by              {-min(row[3] for row in under):.4f}")
    if over:
        print(f"  highest over by               {max(row[3] for row in over):.4f}")

    print()
    print("by neighbour separation:")
    for separation in sorted({row[0] for row in rows}):
        group = [row for row in rows if abs(row[0] - separation) < 1e-9]
        beneath = sum(1 for row in group if row[3] < 0)
        print(
            f"  {separation:7.3f} deg  {len(group):3d} borders  "
            f"{beneath:3d} under, {len(group) - beneath:3d} over"
        )

    assert rows, "a claim of zero needs a population that is not also zero"


if __name__ == "__main__":
    main()
