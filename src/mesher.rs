use bevy::render::{
    mesh::{Indices, Mesh, PrimitiveTopology},
    render_asset::RenderAssetUsages,
};

use crate::chunk::{Chunk, CHUNK_SIZE};

pub fn create_cube_mesh_from_data(geometry_data: GeometryData) -> Mesh {
    let GeometryData {
        position,
        uv,
        normal,
        indices,
    } = geometry_data;

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, position)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uv)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normal)
    .with_inserted_indices(Indices::U32(indices))
}

pub fn create_cube_geometry_data(x: f32, y: f32, z: f32, faces: u8) -> GeometryData {
    let mut position = Vec::new();
    let mut uv = Vec::new();
    let mut normal = Vec::new();
    let mut indices = Vec::new();
    let mut index_offset = 0;

    CUBE_FACES.iter().enumerate().for_each(|(i, face)| {
        if faces & (1 << i) != 0 {
            let face_vertices = face_vertices(*face);
            for vertex in face_vertices.iter() {
                position.push([
                    vertex.position[0] + x,
                    vertex.position[1] + y,
                    vertex.position[2] + z,
                ]);
                uv.push(vertex.uv);
                normal.push(vertex.normal);
            }

            let offsets = [0, 1, 2, 2, 1, 3];
            offsets.iter().for_each(|offset| {
                indices.push(index_offset + offset);
            });
            index_offset += 4;
        }
    });

    GeometryData {
        position,
        uv,
        normal,
        indices,
    }
}

pub fn create_chunk_mesh(chunk: Chunk) -> Mesh {
    let mut geometry_data = GeometryData {
        position: Vec::new(),
        uv: Vec::new(),
        normal: Vec::new(),
        indices: Vec::new(),
    };

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                if (chunk.get(x as usize, y as usize, z as usize) & 0b111111) == 0 {
                    continue;
                }

                let cube_data = create_cube_geometry_data(x as f32, y as f32, z as f32, 0b111111);

                geometry_data.position.extend(cube_data.position);
                geometry_data.uv.extend(cube_data.uv);
                geometry_data.normal.extend(cube_data.normal);
                geometry_data.indices.extend(
                    cube_data
                        .indices
                        .iter()
                        .map(|i| i + geometry_data.position.len() as u32),
                );
            }
        }
    }

    create_cube_mesh_from_data(geometry_data)
}

#[derive(Debug, Clone, Copy)]
enum CubeFace {
    Top,
    Bottom,
    Right,
    Left,
    Back,
    Forward,
}

const CUBE_FACES: [CubeFace; 6] = [
    CubeFace::Top,
    CubeFace::Bottom,
    CubeFace::Right,
    CubeFace::Left,
    CubeFace::Back,
    CubeFace::Forward,
];

struct Vertex {
    position: [f32; 3],
    uv: [f32; 2],
    normal: [f32; 3],
}

pub struct GeometryData {
    pub position: Vec<[f32; 3]>,
    pub uv: Vec<[f32; 2]>,
    pub normal: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

#[rustfmt::skip]
fn face_vertices(face_index: CubeFace) -> [Vertex; 4] {
    match face_index {
        CubeFace::Left => [
            Vertex{ position: [-1.0, -1.0, -1.0], normal: [-1.0, 0.0, 0.0], uv: [0.0, 0.0] },
            Vertex{ position: [-1.0, -1.0, 1.0], normal: [-1.0, 0.0, 0.0], uv: [1.0, 0.0] },
            Vertex{ position: [-1.0, 1.0, -1.0], normal: [-1.0, 0.0, 0.0], uv: [0.0, 1.0] },
            Vertex{ position: [-1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0], uv: [1.0, 1.0] },
        ],
        CubeFace::Right => [
            Vertex{ position: [1.0, -1.0, 1.0], normal: [1.0, 0.0, 0.0], uv: [0.0, 0.0] },
            Vertex{ position: [1.0, -1.0, -1.0], normal: [1.0, 0.0, 0.0], uv: [1.0, 0.0] },
            Vertex{ position: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 0.0], uv: [0.0, 1.0] },
            Vertex{ position: [1.0, 1.0, -1.0], normal: [1.0, 0.0, 0.0], uv: [1.0, 1.0] },
        ],
        CubeFace::Bottom => [
            Vertex{ position: [1.0, -1.0, 1.0], normal: [0.0, -1.0, 0.0], uv: [0.0, 0.0] },
            Vertex{ position: [-1.0, -1.0, 1.0], normal: [0.0, -1.0, 0.0], uv: [1.0, 0.0] },
            Vertex{ position: [1.0, -1.0, -1.0], normal: [0.0, -1.0, 0.0], uv: [0.0, 1.0] },
            Vertex{ position: [-1.0, -1.0, -1.0], normal: [0.0, -1.0, 0.0], uv: [1.0, 1.0] },
        ],
        CubeFace::Top => [
            Vertex{ position: [1.0, 1.0, -1.0], normal: [0.0, 1.0, 0.0], uv: [0.0, 0.0] },
            Vertex{ position: [-1.0, 1.0, -1.0], normal: [0.0, 1.0, 0.0], uv: [1.0, 0.0] },
            Vertex{ position: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 0.0], uv: [0.0, 1.0] },
            Vertex{ position: [-1.0, 1.0, 1.0], normal: [0.0, 1.0, 0.0], uv: [1.0, 1.0] },
        ],
        CubeFace::Back => [
            Vertex{ position: [1.0, -1.0, -1.0], normal: [0.0, 0.0, -1.0], uv: [0.0, 0.0] },
            Vertex{ position: [-1.0, -1.0, -1.0], normal: [0.0, 0.0, -1.0], uv: [1.0, 0.0] },
            Vertex{ position: [1.0, 1.0, -1.0], normal: [0.0, 0.0, -1.0], uv: [0.0, 1.0] },
            Vertex{ position: [-1.0, 1.0, -1.0], normal: [0.0, 0.0, -1.0], uv: [1.0, 1.0] },
        ],
        CubeFace::Forward => [
            Vertex{ position: [-1.0, -1.0, 1.0], normal: [0.0, 0.0, 1.0], uv: [0.0, 0.0] },
            Vertex{ position: [1.0, -1.0, 1.0], normal: [0.0, 0.0, 1.0], uv: [1.0, 0.0] },
            Vertex{ position: [-1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0], uv: [0.0, 1.0] },
            Vertex{ position: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0], uv: [1.0, 1.0] }
        ],
    }
}
