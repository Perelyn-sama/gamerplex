use gamerplex_math::{Quaternion, Vector3};

#[derive(Clone, Debug)]
pub enum ColliderShape {
    Sphere { radius: f32 },
    Box { half_extents: Vector3 },
    Capsule { height: f32, radius: f32 },
    Cylinder { height: f32, radius: f32 },
    ConvexHull { points: Vec<Vector3> },
}

#[derive(Clone, Debug)]
pub struct ColliderDef {
    pub shape: ColliderShape,
    pub position: Vector3,      //offset from rigid body
    pub rotation: Quaternion,  // rotation relative to rigid body
    pub density: f32,   // used to calculate mass
    pub friction: f32,  // 0.0 to 1.0 -> standard
    pub restitution: f32, // bounciness, 0.0 to 1.0
    pub is_sensor: bool,  // detects but doesn't collide
    pub collision_groups: u32, // filtering
}

impl Default for ColliderDef {
    fn default() -> Self {
        Self {
            shape: ColliderShape::Box { half_extents: Vector3::new(0.5, 0.5, 0.5) },
            position: Vector3::zeros(),
            rotation: Quaternion::identity(),
            density: 1.0,
            friction: 0.5,
            restitution: 0.0,
            is_sensor: false,
            collision_groups: 0xFFFFFFFF, // All groups by default
        }
    }
}