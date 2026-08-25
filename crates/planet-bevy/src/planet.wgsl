// The planet, drawn on the GPU.
//
// This is a direct port of `planet-render/src/raster.rs`, and deliberately so: it keeps
// the per-pixel algorithm rather than switching to meshes. Every fragment independently
// asks the sphere which region it lands in, exactly as the CPU version does.
//
// Keeping it per-pixel is what avoids the whole class of problem that mesh rendering
// would reintroduce — no antimeridian split, no clipping at the rim, no instancing for
// the repeating rings, and no trouble at one or two regions. Fragments do not consult
// their neighbours, so there are no seams to get wrong.
//
// The CPU path stays in the tree as the reference. When this and that disagree, that
// one is right.

#import bevy_sprite::mesh2d_vertex_output::VertexOutput

const PI: f32 = 3.14159265358979323846;
const MAX_REGIONS: u32 = 512u;

// Matches PlanetUniform on the Rust side.
struct Planet {
    // Rows of the orientation matrix (world -> view). Only xyz are used.
    row0: vec4<f32>,
    row1: vec4<f32>,
    row2: vec4<f32>,
    // x radius, y width, z height, w region count
    view: vec4<f32>,
    // x projection (0 fanned, 1 globe), y hovered region (-1 none), z flags, w border px
    params: vec4<f32>,
    // xyz unit direction of the seed, w packed as colour + 8 * (owner + 1)
    seeds: array<vec4<f32>, MAX_REGIONS>,
};

@group(2) @binding(0) var<uniform> planet: Planet;

const FLAG_BORDERS: u32 = 1u;
const FLAG_DIM_REPEATS: u32 = 2u;

// Palette, matching planet-render/src/palette.rs. Values are sRGB, converted below.
fn region_colour(index: u32) -> vec3<f32> {
    switch index {
        case 0u: { return vec3<f32>(0.106, 0.227, 0.361); } // dark blue
        case 1u: { return vec3<f32>(0.545, 0.290, 0.612); } // purple
        case 2u: { return vec3<f32>(0.310, 0.659, 0.561); } // teal green
        case 3u: { return vec3<f32>(0.910, 0.851, 0.541); } // pale gold
        case 4u: { return vec3<f32>(0.769, 0.341, 0.235); } // burnt orange
        default: { return vec3<f32>(0.949, 0.949, 0.925); } // off white
    }
}

fn player_colour(index: u32) -> vec3<f32> {
    switch index {
        case 0u: { return vec3<f32>(0.898, 0.282, 0.302); } // red
        case 1u: { return vec3<f32>(0.243, 0.608, 1.000); } // blue
        case 2u: { return vec3<f32>(0.188, 0.643, 0.424); } // green
        case 3u: { return vec3<f32>(0.961, 0.851, 0.039); } // yellow
        case 4u: { return vec3<f32>(0.898, 0.549, 0.173); } // orange
        default: { return vec3<f32>(0.690, 0.490, 0.910); } // violet
    }
}

// The palettes above are written as sRGB, because that is how they are written in
// planet-render and how a human reads a hex colour. The pipeline wants linear, so the
// conversion happens once at the end rather than in every constant.
fn srgb_to_linear(colour: vec3<f32>) -> vec3<f32> {
    let higher = pow((colour + vec3<f32>(0.055)) / vec3<f32>(1.055), vec3<f32>(2.4));
    let lower = colour / vec3<f32>(12.92);
    return select(higher, lower, colour <= vec3<f32>(0.04045));
}

const BACKGROUND: vec3<f32> = vec3<f32>(0.047, 0.055, 0.071);
const BORDER: vec3<f32> = vec3<f32>(0.020, 0.027, 0.039);
const DUPLICATE_STRENGTH: f32 = 0.34;
const OWNER_TINT: f32 = 0.55;

// Where a pixel lands on the sphere. Returns xyz = world direction, w = ring number,
// or w < 0 for a pixel that shows nothing (only possible when holding the ball
// together rather than fanning it out).
fn screen_to_sample(across: f32, up: f32) -> vec4<f32> {
    let radius = planet.view.x;
    let pixels = sqrt(across * across + up * up);
    var in_view: vec3<f32>;
    var ring: f32 = 0.0;

    if planet.params.x > 0.5 {
        // Globe: one hemisphere, space beyond the limb.
        let distance = pixels / radius;
        if distance > 1.0 {
            return vec4<f32>(0.0, 0.0, 0.0, -1.0);
        }
        in_view = vec3<f32>(
            across / radius,
            up / radius,
            sqrt(max(0.0, 1.0 - distance * distance)),
        );
    } else {
        // Fanned: the whole world inside a disc of radius pi, repeating outward.
        let pixels_per_radian = radius / PI;
        let raw = pixels / pixels_per_radian;
        ring = floor(raw / PI);
        let into_ring = raw - ring * PI;

        var angle: f32;
        var side: f32;
        // Odd rings are turned inside out: the geodesic has passed the far point and
        // is on its way back, so it arrives from the opposite direction.
        if (u32(ring) % 2u) == 0u {
            angle = into_ring;
            side = 1.0;
        } else {
            angle = PI - into_ring;
            side = -1.0;
        }

        if pixels < 1e-6 {
            in_view = vec3<f32>(0.0, 0.0, 1.0);
        } else {
            let s = sin(angle);
            in_view = vec3<f32>(
                s * side * across / pixels,
                s * side * up / pixels,
                cos(angle),
            );
        }
    }

    // Inverse of the orientation, which for a rotation is its transpose.
    let world = planet.row0.xyz * in_view.x
        + planet.row1.xyz * in_view.y
        + planet.row2.xyz * in_view.z;
    return vec4<f32>(world, ring);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let width = planet.view.y;
    let height = planet.view.z;
    let count = u32(planet.view.w);

    // The quad covers the window exactly, so uv maps straight onto pixels.
    let x = mesh.uv.x * width;
    let y = mesh.uv.y * height;
    let across = x - width * 0.5;
    let up = -(y - height * 0.5);

    let sample = screen_to_sample(across, up);
    if sample.w < 0.0 {
        return vec4<f32>(srgb_to_linear(BACKGROUND), 1.0);
    }
    let direction = sample.xyz;

    // Nearest and second nearest seed, in one pass.
    var best: f32 = -2.0;
    var second: f32 = -2.0;
    var best_index: u32 = 0u;
    for (var i: u32 = 0u; i < count; i = i + 1u) {
        let towards = dot(planet.seeds[i].xyz, direction);
        if towards > best {
            second = best;
            best = towards;
            best_index = i;
        } else if towards > second {
            second = towards;
        }
    }

    // w packs the region's colour index and its owner.
    let packed = u32(planet.seeds[best_index].w + 0.5);
    let owner_plus_one = packed / 8u;
    var colour = region_colour(packed % 8u);
    if owner_plus_one > 0u {
        colour = mix(colour, player_colour(owner_plus_one - 1u), OWNER_TINT);
    }

    // The region under the cursor.
    if planet.params.y >= 0.0 && u32(planet.params.y + 0.5) == best_index {
        colour = mix(colour, vec3<f32>(1.0), 0.30);
    }

    let flags = u32(planet.params.z + 0.5);

    // Borders. The angular gap to the second nearest seed is a distance field; dividing
    // by its own screen-space derivative turns it into a distance in pixels, which is
    // how the border keeps an even width at any zoom under either projection without
    // knowing anything about the projection. The CPU path gets the same result by
    // looking at neighbouring pixels, which a fragment cannot do.
    if (flags & FLAG_BORDERS) != 0u && count > 1u {
        // The gap between the best and second-best score is zero exactly on a border
        // and grows away from it. Dividing by its own screen-space derivative converts
        // it to a distance in pixels, which keeps the border an even width at any zoom
        // under either projection.
        //
        // Note there is no acos here. Seeds are `n / h` rather than unit vectors, so a
        // dot product with one is not a cosine and taking its arc cosine would be
        // meaningless. It does not matter: any field that vanishes on the border works,
        // because fwidth normalises the scale away.
        let gap = best - second;
        let in_pixels = gap / max(fwidth(gap), 1e-8);
        if in_pixels < planet.params.w {
            colour = BORDER;
        }
    }

    // Every ring past the first is a repeat of the world, and is dimmed.
    if (flags & FLAG_DIM_REPEATS) != 0u && sample.w > 0.5 {
        colour = mix(BACKGROUND, colour, DUPLICATE_STRENGTH);
    }

    return vec4<f32>(srgb_to_linear(colour), 1.0);
}
