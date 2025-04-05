use gamerplex_math::{Quaternion, Vector3};


#[derive(Clone, Debug, PartialEq)]
pub enum BodyType {
    Dynamic, // laws of physics aplies to these body types
    Static, // immovable objs like mountains and terrains
    Kinematic // programatic movements not the laws of phyysics
}

#[derive(Clone, Debug)]
pub struct Body {
    pub body_type: BodyType,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub linear_velocity: Vector3,
    pub angular_velocity: Vector3,
    pub mass: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub can_sleep: bool,
    pub ccd_enabled: bool, // Continuous collision detection
}

impl Default for Body {
    fn default() -> Self {
        Self {
            body_type: BodyType::Dynamic,
            position: Vector3::zeros(),
            rotation: Quaternion::identity(),
            linear_velocity: Vector3::zeros(),
            angular_velocity: Vector3::zeros(),
            mass: 1.0,
            linear_damping: 0.0,
            angular_damping: 0.0,
            can_sleep: true,
            ccd_enabled: false,
        }
    }
}