use bevy::prelude::Mesh;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use crate::gameplay::GameMap;

#[derive(Debug)]
pub struct GameFieldGround {
    pub width: f32,
    pub height: f32,
}

impl From<&GameMap> for Mesh {
    fn from(map: &GameMap) -> Self {
        let cell_size = map.grid_size as f32; //TODO: bring that in from the editor

        let mut vertices: Vec<([f32; 3], [f32; 3], [f32; 2])> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        //let x_offset = map.width / 2.;
        let x_offset = 0.;
        //let y_offset = map.height / 2.;
        let y_offset = 0.;

        for x in (0..map.width as i32).rev() {
            let pos_x = (x as f32 * cell_size) - x_offset;
            for y in (0..map.height as i32).rev() {
                let pos_y = (y as f32 * cell_size) - y_offset;
                // each cell is made of 4 vertices and 2 triangles
                // we are assuming the center of the plane will be 0,0
                // Creating counter clockwise
                let vert_index = vertices.len() as u32;

                let loc_vertices = [
                    ([pos_x + cell_size, 0.0, pos_y], [0.0, 1.0, 0.0], [1.0, 1.0]),
                    ([pos_x + cell_size, 0.0, pos_y + cell_size], [0.0, 1.0, 0.0], [1.0, 0.0]),
                    ([pos_x, 0.0, pos_y + cell_size], [0.0, 1.0, 0.0], [0.0, 0.0]),
                    ([pos_x, 0.0, pos_y], [0.0, 1.0, 0.0], [0.0, 1.0]),
                ];
                vertices.push(loc_vertices[0]);
                vertices.push(loc_vertices[1]);
                vertices.push(loc_vertices[2]);
                vertices.push(loc_vertices[3]);

                for i in &[1, 0, 2, 3, 2, 0] {
                    indices.push(*i + vert_index);
                }
            }
        }

        let indices = Indices::U32(indices);

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        for (position, normal, uv) in &vertices {
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