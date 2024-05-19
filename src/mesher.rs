pub fn create_cube_mesh_from_data(
    position: Vec<[f32; 3]>,
    uv: Vec<[f32; 2]>,
    normal: Vec<[f32; 3]>,
    indices: Vec<u32>,
) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, position)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uv)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normal)
    .with_inserted_indices(Indices::U32(indices))
}

#[rustfmt::skip]
pub fn create_cube_mesh(x: f32, y: f32, z: f32) -> Mesh {
    let positions = CUBE_POSITIONS.iter().map(|[px, py, pz]| [px + x, py + y, pz + z]).collect();
    let uvs = CUBE_UVS.iter().copied().collect();
    let normals = CUBE_NORMALS.iter().copied().collect();
    let indices = CUBE_INDICES.iter().copied().collect();
    create_cube_mesh_from_data(positions, uvs, normals, indices)
}

enum CubeFace {
    Top,
    Bottom,
    Right,
    Left,
    Back,
    Forward,
}

#[rustfmt::skip]
const CUBE_POSITIONS: Vec<[f32]> = vec![
    // top (facing towards +y)
    [-0.5, 0.5, -0.5], // vertex with index 0
    [0.5, 0.5, -0.5], // vertex with index 1
    [0.5, 0.5, 0.5], // etc. until 23
    [-0.5, 0.5, 0.5],
    // bottom   (-y)
    [-0.5, -0.5, -0.5],
    [0.5, -0.5, -0.5],
    [0.5, -0.5, 0.5],
    [-0.5, -0.5, 0.5],
    // right    (+x)
    [0.5, -0.5, -0.5],
    [0.5, -0.5, 0.5],
    [0.5, 0.5, 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
    [0.5, 0.5, -0.5],
    // left     (-x)
    [-0.5, -0.5, -0.5],
    [-0.5, -0.5, 0.5],
    [-0.5, 0.5, 0.5],
    [-0.5, 0.5, -0.5],
    // back     (+z)
    [-0.5, -0.5, 0.5],
    [-0.5, 0.5, 0.5],
    [0.5, 0.5, 0.5],
    [0.5, -0.5, 0.5],
    // forward  (-z)
    [-0.5, -0.5, -0.5],
    [-0.5, 0.5, -0.5],
    [0.5, 0.5, -0.5],
    [0.5, -0.5, -0.5],
];
const CUBE_UVS: Vec<[f32]> = vec![
    // Assigning the UV coords for the top side.
    [0.0, 0.2],
    [0.0, 0.0],
    [1.0, 0.0],
    [1.0, 0.25],
    // Assigning the UV coords for the bottom side.
    [0.0, 0.45],
    [0.0, 0.25],
    [1.0, 0.25],
    [1.0, 0.45],
    // Assigning the UV coords for the right side.
    [1.0, 0.45],
    [0.0, 0.45],
    [0.0, 0.2],
    [1.0, 0.2],
    // Assigning the UV coords for the left side.
    [1.0, 0.45],
    [0.0, 0.45],
    [0.0, 0.2],
    [1.0, 0.2],
    // Assigning the UV coords for the back side.
    [0.0, 0.45],
    [0.0, 0.2],
    [1.0, 0.2],
    [1.0, 0.45],
    // Assigning the UV coords for the forward side.
    [0.0, 0.45],
    [0.0, 0.2],
    [1.0, 0.2],
    [1.0, 0.45],
];
const CUBE_NORMALS: Vec<[f32]> = vec![
    // Normals for the top side (towards +y)
    [0.0, 1.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 1.0, 0.0],
    // Normals for the bottom side (towards -y)
    [0.0, -1.0, 0.0],
    [0.0, -1.0, 0.0],
    [0.0, -1.0, 0.0],
    [0.0, -1.0, 0.0],
    // Normals for the right side (towards +x)
    [1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    // Normals for the left side (towards -x)
    [-1.0, 0.0, 0.0],
    [-1.0, 0.0, 0.0],
    [-1.0, 0.0, 0.0],
    [-1.0, 0.0, 0.0],
    // Normals for the back side (towards +z)
    [0.0, 0.0, 1.0],
    [0.0, 0.0, 1.0],
    [0.0, 0.0, 1.0],
    [0.0, 0.0, 1.0],
    // Normals for the forward side (towards -z)
    [0.0, 0.0, -1.0],
    [0.0, 0.0, -1.0],
    [0.0, 0.0, -1.0],
    [0.0, 0.0, -1.0],
];
const CUBE_INDICES: Vec<u32> = vec![
    0, 3, 1, 1, 3, 2, // triangles making up the top (+y) facing side.
    4, 5, 7, 5, 6, 7, // bottom (-y)
    8, 11, 9, 9, 11, 10, // right (+x)
    12, 13, 15, 13, 14, 15, // left (-x)
    16, 19, 17, 17, 19, 18, // back (+z)
    20, 21, 23, 21, 22, 23, // forward (-z)
];

struct Vertex {
    position: [f32; 3],
    uv: [f32; 2],
    normal: [f32; 3],
}

#[rustfmt::skip]
fn face_vertices(face_index: CubeFace) -> [Vertex; 4] {
    match face_index {
        CubeFace::Top => [
            Vertex { position: [-0.5, 0.5, -0.5], uv: [0.0, 0.2], normal: [0.0, 1.0, 0.0] },
            Vertex { position: [0.5, 0.5, -0.5], uv: [0.0, 0.0], normal: [0.0, 1.0, 0.0], },
            Vertex { position: [0.5, 0.5, 0.5], uv: [1.0, 0.0], normal: [0.0, 1.0, 0.0], },
            Vertex { position: [-0.5, 0.5, 0.5], uv: [1.0, 0.25], normal: [0.0, 1.0, 0.0], },
        ],
        CubeFace::Bottom => [
            Vertex { position: [-0.5, -0.5, -0.5], uv: [0.0, 0.45], normal: [0.0, -1.0, 0.0] },
            Vertex { position: [0.5, -0.5, -0.5], uv: [0.0, 0.25], normal: [0.0, -1.0, 0.0] },
            Vertex { position: [0.5, -0.5, 0.5], uv: [1.0, 0.25], normal: [0.0, -1.0, 0.0] },
            Vertex { position: [-0.5, -0.5, 0.5], uv: [1.0, 0.45], normal: [0.0, -1.0, 0.0] },
        ],
        CubeFace::Right => [
            Vertex { position: [0.5, -0.5, -0.5], uv: [1.0, 0.45], normal: [1.0, 0.0, 0.0] },
            Vertex { position: [0.5, -0.5, 0.5], uv: [0.0, 0.45], normal: [1.0, 0.0, 0.0] },
            Vertex { position: [0.5, 0.5, 0.5], uv: [0.0, 0.2], normal: [1.0, 0.0, 0.0] },
            Vertex { position: [0.5, 0.5, -0.5], uv: [1.0, 0.2], normal: [1.0, 0.0, 0.0] },
        ],
        CubeFace::Left => [
            Vertex { position: [-0.5, -0.5, -0.5], uv: [1.0, 0.45], normal: [-1.0, 0.0, 0.0] },
            Vertex { position: [-0.5, -0.5, 0.5], uv: [0.0, 0.45], normal: [-1.0, 0.0, 0.0] },
            Vertex { position: [-0.5, 0.5, 0.5], uv: [0.0, 0.2], normal: [-1.0, 0.0, 0.0] },
            Vertex { position: [-0.5, 0.5, -0.5], uv: [1.0, 0.2], normal: [-1.0, 0.0, 0.0] },
        ],
        CubeFace::Back => [
            Vertex { position: [-0.5, -0.5, 0.5], uv: [0.0, 0.45], normal: [0.0, 0.0, 1.0] },
            Vertex { position: [-0.5, 0.5, 0.5], uv: [0.0, 0.2], normal: [0.0, 0.0, 1.0] },
            Vertex { position: [0.5, 0.5, 0.5], uv: [1.0, 0.2], normal: [0.0, 0.0, 1.0] },
            Vertex { position: [0.5, -0.5, 0.5], uv: [1.0, 0.45], normal: [0.0, 0.0, 1.0] },
        ],
        CubeFace::Forward => [
            Vertex { position: [-0.5, -0.5, -0.5], uv: [0.0, 0.45], normal: [0.0, 0.0, -1.0] },
            Vertex { position: [-0.5, 0.5, -0.5], uv: [0.0, 0.2], normal: [0.0, 0.0, -1.0] },
            Vertex { position: [0.5, 0.5, -0.5], uv: [1.0, 0.2], normal: [0.0, 0.0, -1.0] },
            Vertex { position: [0.5, -0.5, -0.5], uv: [1.0, 0.45], normal: [0.0, 0.0, -1.0] },
        ],
    }
}
