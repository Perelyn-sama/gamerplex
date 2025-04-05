// This file provides conversion between gamerplex math types and other libraries

#[cfg(feature = "rapier3d")]
pub mod rapier {
    use crate::vector::Vector3;
    use crate::quaternion::Quaternion;
    use rapier3d::math as rapier_math;
    
    // Vector3 conversions
    impl From<Vector3> for rapier_math::Vector<f32> {
        fn from(v: Vector3) -> Self {
            rapier_math::Vector::new(v.x, v.y, v.z)
        }
    }
    
    impl From<rapier_math::Vector<f32>> for Vector3 {
        fn from(v: rapier_math::Vector<f32>) -> Self {
            Vector3::new(v.x, v.y, v.z)
        }
    }
    
    // Quaternion conversions
    impl From<Quaternion> for rapier_math::Rotation<f32> {
        fn from(q: Quaternion) -> Self {
            rapier_math::Rotation::from_quaternion(
                rapier_math::Quaternion::new(q.w, q.x, q.y, q.z)
            )
        }
    }
    
}