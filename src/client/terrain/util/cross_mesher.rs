use terrain_util::{
    client_block::{block_properties, MeshRepresentation},
    create_cube_mesh_from_data, GeometryData, TextureManager, TextureName, Vertex,
};

use crate::prelude::*;

pub fn instance_mesh_for_repr(
    rep: MeshRepresentation,
    texture_manager: &TextureManager,
) -> Option<Mesh> {
    match rep {
        MeshRepresentation::None => None,
        MeshRepresentation::Cube(_) => None,
        MeshRepresentation::Cross(textures) => {
            let geometry_data = create_cross_geometry(textures, texture_manager);
            // TODO: refactor rename to create_mesh_from_data because it is not cube representation specific
            create_cube_mesh_from_data(geometry_data)
        }
    }
}

fn create_cross_geometry(
    textures: [TextureName; 2],
    texture_manager: &TextureManager,
) -> GeometryData {
    let mut position = vec![];
    let mut uv = vec![];
    let mut normal = vec![];
    let mut indices = vec![];

    let mut index_offset = 0;

    let cross_faces = [CrossFace::Face1, CrossFace::Face2];

    cross_faces.iter().for_each(|cross_face| {
        let face_verticies = cross_face_vertices(*cross_face);

        let face_uv = texture_manager
            .get_texture_uv(textures[0])
            .expect("Texture is not present in manager");

        for vertex in face_verticies {
            position.push([
                vertex.position[0] * 0.5 + 0.5,
                vertex.position[1] * 0.5 + 0.5,
                vertex.position[2] * 0.5 + 0.5,
            ]);

            uv.push([
                face_uv[0] + vertex.uv[0] * 0.25,
                face_uv[1] + vertex.uv[1] * 0.25,
            ]);
            normal.push(vertex.normal);
        }

        let offsets = [0, 1, 3, 1, 2, 3];
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

pub fn get_cross_block_positions(chunk: &Chunk) -> HashMap<MeshRepresentation, Vec<Vec3>> {
    let mut map: HashMap<MeshRepresentation, Vec<Vec3>> = HashMap::new();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let block_id = chunk.get(x, y, z);
                let pos = Vec3::new(x as f32, y as f32, z as f32);
                let mesh_repr = block_properties(block_id).mesh_representation;

                if let MeshRepresentation::Cross(_) = mesh_repr {
                    match map.get_mut(&mesh_repr) {
                        Some(positions) => positions.push(pos),
                        None => {
                            map.insert(mesh_repr, vec![pos]);
                        }
                    };
                }
            }
        }
    }

    map
}

#[derive(Debug, Clone, Copy)]
pub enum CrossFace {
    Face1,
    Face2,
}

#[rustfmt::skip]
fn cross_face_vertices(face: CrossFace) -> [Vertex; 4] {
    match face {
        CrossFace::Face1 => [
            Vertex{ position: [-1.0,  1.0, -1.0], normal: [FRAC_1_SQRT_2, 0.0, -FRAC_1_SQRT_2], uv: [0.0, 0.0] },
            Vertex{ position: [ 1.0,  1.0,  1.0], normal: [FRAC_1_SQRT_2, 0.0, -FRAC_1_SQRT_2], uv: [1.0, 0.0] },
            Vertex{ position: [ 1.0, -1.0,  1.0], normal: [FRAC_1_SQRT_2, 0.0, -FRAC_1_SQRT_2], uv: [1.0, 1.0] },
            Vertex{ position: [-1.0, -1.0, -1.0], normal: [FRAC_1_SQRT_2, 0.0, -FRAC_1_SQRT_2], uv: [0.0, 1.0] },
        ],
        CrossFace::Face2 => [
            Vertex{ position: [-1.0,  1.0,  1.0], normal: [-FRAC_1_SQRT_2, 0.0, -FRAC_1_SQRT_2], uv: [0.0, 0.0] },
            Vertex{ position: [ 1.0,  1.0, -1.0], normal: [-FRAC_1_SQRT_2, 0.0, -FRAC_1_SQRT_2], uv: [1.0, 0.0] },
            Vertex{ position: [ 1.0, -1.0, -1.0], normal: [-FRAC_1_SQRT_2, 0.0, -FRAC_1_SQRT_2], uv: [1.0, 1.0] },
            Vertex{ position: [-1.0, -1.0,  1.0], normal: [-FRAC_1_SQRT_2, 0.0, -FRAC_1_SQRT_2], uv: [0.0, 1.0] },
        ],
    }
}
