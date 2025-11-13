use nalgebra_glm as glm;

pub type Vec3 = glm::Vec3;

pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    glm::vec3(x, y, z)
}
