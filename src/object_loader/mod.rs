pub mod object;
use crate::math::Vec3;
use std::hash::Hash;
use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};

#[derive(Debug, Clone)]
pub struct Object {
    pub vertex: Vec<Vertexxx>,
    pub indice: Vec<u32>,
    pub size: Vec3,
    pub center: Vec3,
}

#[derive(BufferContents, Vertex, Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Vertexxx {
    #[format(R32G32B32_SFLOAT)]
    #[name("in_position")]
    pub position: [f32; 3],

    #[format(R32G32B32_SFLOAT)]
    #[name("in_normal")]
    pub normal: [f32; 3],

    #[format(R32G32B32_SFLOAT)]
    #[name("in_color")]
    pub color: [f32; 3],

    #[format(R32G32_SFLOAT)]
    #[name("in_texture")]
    pub texture: [f32; 2],
}

impl PartialEq for Vertexxx {
    fn eq(&self, other: &Self) -> bool {
        let iter_self = self
            .position
            .iter()
            .chain(self.normal.iter())
            .chain(self.texture.iter());
        let iter_other = other
            .position
            .iter()
            .chain(other.normal.iter())
            .chain(other.texture.iter());
        for (s, o) in iter_self.zip(iter_other) {
            if s.to_bits() != o.to_bits() {
                return false;
            }
        }
        true
    }
}

impl Eq for Vertexxx {}

impl Hash for Vertexxx {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for v in self
            .position
            .iter()
            .chain(self.normal.iter())
            .chain(self.texture.iter())
        {
            v.to_bits().hash(state);
        }
    }
}
