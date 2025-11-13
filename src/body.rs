use crate::geom::Vec3;

#[derive(Copy, Clone)]
pub enum BodyShader {
    Sun,
    Earth,
    Mars,
    Mercury,
}

pub struct CelestialBody {
    pub center: Vec3,
    pub radius: f32,
    pub shader: BodyShader,
}

impl CelestialBody {
    pub fn new(center: Vec3, radius: f32, shader: BodyShader) -> Self {
        Self { center, radius, shader }
    }
}
