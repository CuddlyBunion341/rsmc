use bevy::{
    log::info,
    render::{
        mesh::{Indices, Mesh, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
};

use crate::{
    blocks::get_block_face_uvs,
    chunk::{Chunk, CHUNK_SIZE},
};

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

pub fn create_cube_geometry_data(x: f32, y: f32, z: f32, faces: u8, block_id: u8) -> GeometryData {
    let mut position = Vec::new();
    let mut uv = Vec::new();
    let mut normal = Vec::new();
    let mut indices = Vec::new();
    let mut index_offset = 0;

    CUBE_FACES.iter().enumerate().for_each(|(i, face)| {
        if faces & (1 << i) == 0 {
            return;
        }

        let face_vertices = face_vertices(*face);
        for vertex in face_vertices.iter() {
            position.push([
                vertex.position[0] * 0.5 + x + 0.5,
                vertex.position[1] * 0.5 + y + 0.5,
                vertex.position[2] * 0.5 + z + 0.5,
            ]);

            let block_uvs = get_block_face_uvs(block_id, *face).unwrap();
            uv.push([
                block_uvs[0] + vertex.uv[0] * 0.25,
                block_uvs[1] + (1.0 - vertex.uv[1]) * 0.25,
            ]);
            normal.push(vertex.normal);
        }

        let offsets = [0, 1, 2, 2, 1, 3];
        offsets.iter().for_each(|offset| {
            indices.push(index_offset + offset);
        });
        index_offset += 4;
    });

    GeometryData {
        position,
        uv,
        normal,
        indices,
    }
}

pub fn create_chunk_mesh(chunk: &Chunk) -> Mesh {
    let mut geometry_data = GeometryData {
        position: Vec::new(),
        uv: Vec::new(),
        normal: Vec::new(),
        indices: Vec::new(),
    };

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let block_id = chunk.get(x as usize, y as usize, z as usize);

                if block_id == 0 {
                    continue;
                }

                fn update_mask(chunk: &Chunk, mask: &mut u8, value: u8, x: i32, y: i32, z: i32) {
                    if chunk.get(x as usize, y as usize, z as usize) == 1 {
                        *mask ^= value;
                    }
                }

                let mut mask = 0b111111;

                // update_mask(&chunk, &mut mask, 0b000001, x, y + 1, z);
                // update_mask(&chunk, &mut mask, 0b000010, x, y - 1, z);

                // update_mask(&chunk, &mut mask, 0b000100, x + 1, y, z);
                // update_mask(&chunk, &mut mask, 0b001000, x - 1, y, z);

                // update_mask(&chunk, &mut mask, 0b010000, x, y, z + 1);
                // update_mask(&chunk, &mut mask, 0b100000, x, y, z - 1);

                let cube_data =
                    create_cube_geometry_data(x as f32, y as f32, z as f32, mask, block_id);

                geometry_data.indices.extend(
                    cube_data
                        .indices
                        .iter()
                        .map(|i| i + geometry_data.position.len() as u32),
                );
                geometry_data.position.extend(cube_data.position);
                geometry_data.uv.extend(cube_data.uv);
                geometry_data.normal.extend(cube_data.normal);
            }
        }
    }

    create_cube_mesh_from_data(geometry_data)
}

#[derive(Debug, Clone, Copy)]
pub enum CubeFace {
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
