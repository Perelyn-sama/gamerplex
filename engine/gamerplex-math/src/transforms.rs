use crate::vector::Vector3;
use crate::quaternion::Quaternion;

#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}

impl Transform {
    pub fn new(position: Vector3, rotation: Quaternion, scale: Vector3) -> Self {
        Self { position, rotation, scale }
    }
    
    pub fn identity() -> Self {
        Self {
            position: Vector3::zeros(),
            rotation: Quaternion::identity(),
            scale: Vector3::ones(),
        }
    }
    
}