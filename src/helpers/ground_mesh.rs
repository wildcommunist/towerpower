use bevy::prelude::Mesh;
use bevy::render::mesh::{Indices, PrimitiveTopology};

#[derive(Debug)]
pub struct GameFieldGround {
    pub width: f32,
    pub height: f32,
}


impl From<GameFieldGround> for Mesh {
    fn from(value: GameFieldGround) -> Self {
        let width = value.width / 2.;
        let height = value.height / 2.;

        // we are assuming the center of the plane will be 0,0
        // Creating counter clockwise
        let vertices = [
            ([width, 0.0, -height], [0.0, 1.0, 0.0], [1.0, 1.0]),
            ([width, 0.0, height], [0.0, 1.0, 0.0], [1.0, 0.0]),
            ([-width, 0.0, height], [0.0, 1.0, 0.0], [0.0, 0.0]),
            ([-width, 0.0, -height], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ];

        let indices = Indices::U32(vec![0, 2, 1, 0, 3, 2]);

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        for (position, normal, uv) in vertices.iter() {
            positions.push(*position);
            normals.push(*normal);
            uvs.push(*uv);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}