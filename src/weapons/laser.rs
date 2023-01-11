use bevy::prelude::*;
use bevy::render::mesh::{PrimitiveTopology};

#[derive(Component)]
pub struct Laser {
    pub thickness: f32,
    pub source: Option<Vec3>,
    pub target: Option<Vec3>,
}

impl Default for Laser {
    fn default() -> Self {
        Self {
            thickness: 1.0,
            source: None,
            target: None,
        }
    }
}


impl From<Laser> for Mesh {
    fn from(line: Laser) -> Self {
        // This tells wgpu that the positions are list of lines
        // where every pair is a start and end point
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);

        mesh
    }
}
