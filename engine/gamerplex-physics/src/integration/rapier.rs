use rapier3d::prelude::*;
use crate::body::{BodyType, Body};
use crate::collider::{ColliderDef, ColliderShape};
use crate::handles::BodyHandle;
use gamerplex_math::{Vector3, Quaternion};

// Conversion utilities between your math types and nalgebra
pub fn to_na_vector(v: &Vector3) -> nalgebra::Vector3<f32> {
    nalgebra::Vector3::new(v.x, v.y, v.z)
}

pub fn from_na_vector(v: &nalgebra::Vector3<f32>) -> Vector3 {
    Vector3::new(v.x, v.y, v.z)
}

pub fn to_na_quaternion(q: &Quaternion) -> nalgebra::UnitQuaternion<f32> {
    nalgebra::UnitQuaternion::new_normalize(
        nalgebra::Quaternion::new(q.w, q.x, q.y, q.z)
    )
}

pub fn from_na_quaternion(q: &nalgebra::UnitQuaternion<f32>) -> Quaternion {
    Quaternion::new(q.i, q.j, q.k, q.w)
}

// Convert our body type to Rapier's
pub fn convert_body_type(body_type: &BodyType) -> RigidBodyType {
    match body_type {
        BodyType::Dynamic => RigidBodyType::Dynamic,
        BodyType::Static => RigidBodyType::Fixed,
        BodyType::Kinematic => RigidBodyType::KinematicPositionBased,
    }
}

// Create a Rapier rigid body from our definition
pub fn create_rigid_body(def: &Body) -> RigidBodyBuilder {
    let mut builder = RigidBodyBuilder::new(convert_body_type(&def.body_type))
        .translation(to_na_vector(&def.position))
        .rotation(to_na_vector(&def.rotation.to_euler_angles()))
        .linear_damping(def.linear_damping)
        .angular_damping(def.angular_damping)
        .can_sleep(def.can_sleep)
        .ccd_enabled(def.ccd_enabled);
    
    if def.body_type == BodyType::Dynamic {
        builder = builder
            .additional_mass(def.mass)
            .linvel(to_na_vector(&def.linear_velocity))
            .angvel(to_na_vector(&def.angular_velocity));
    }
    
    builder
}

// Create a Rapier collider from our definition
pub fn create_collider(def: &ColliderDef) -> ColliderBuilder {
    // Convert shape to Rapier's shape
    let shape = match &def.shape {
        ColliderShape::Sphere { radius } => {
            SharedShape::ball(*radius)
        },
        ColliderShape::Box { half_extents } => {
            SharedShape::cuboid(
                half_extents.x, 
                half_extents.y, 
                half_extents.z
            )
        },
        ColliderShape::Capsule { height, radius } => {
            SharedShape::capsule_y(height / 2.0, *radius)
        },
        ColliderShape::Cylinder { height, radius } => {
            SharedShape::cylinder(*height / 2.0, *radius)
        },
        ColliderShape::ConvexHull { points } => {
            let na_points: Vec<nalgebra::Point3<f32>> = points.iter()
                .map(|p| nalgebra::Point3::new(p.x, p.y, p.z))
                .collect();
            
            SharedShape::convex_hull(&na_points).unwrap_or(
                // Fallback to a sphere if convex hull fails
                SharedShape::ball(1.0)
            )
        },
    };
    
    let mut builder = ColliderBuilder::new(shape)
        .density(def.density)
        .friction(def.friction)
        .restitution(def.restitution)
        .sensor(def.is_sensor);
    
    // Add position/rotation offset if not at origin
    if def.position != Vector3::zeros() || def.rotation != Quaternion::identity() {
        builder = builder
            .translation(to_na_vector(&def.position))
            .rotation(to_na_vector(&def.rotation.to_euler_angles()));
    }
    
    // Add collision groups
    if def.collision_groups != 0xFFFFFFFF {
        let groups = InteractionGroups::new(
            Group::from_bits(def.collision_groups as u32).unwrap_or(Group::all()),
            Group::from_bits(def.collision_groups as u32).unwrap_or(Group::all())
        );
        builder = builder.collision_groups(groups);
    }
    
    builder
}
