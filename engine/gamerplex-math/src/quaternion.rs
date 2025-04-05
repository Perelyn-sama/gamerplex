use crate::Vector3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    
    pub fn identity() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }
    
    pub fn from_axis_angle(axis: &Vector3, angle: f32) -> Self {
        let half_angle = angle * 0.5;
        let s = half_angle.sin();
        
        Self {
            x: axis.x * s,
            y: axis.y * s,
            z: axis.z * s,
            w: half_angle.cos(),
        }
    }
    
    pub fn from_euler(pitch: f32, yaw: f32, roll: f32) -> Self {
        // Convert Euler angles to quaternion
        let cy = (yaw * 0.5).cos();
        let sy = (yaw * 0.5).sin();
        let cp = (pitch * 0.5).cos();
        let sp = (pitch * 0.5).sin();
        let cr = (roll * 0.5).cos();
        let sr = (roll * 0.5).sin();
        
        Self {
            x: sr * cp * cy - cr * sp * sy,
            y: cr * sp * cy + sr * cp * sy,
            z: cr * cp * sy - sr * sp * cy,
            w: cr * cp * cy + sr * sp * sy,
        }
    }
    
    pub fn normalize(&self) -> Self {
        let len_sq = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
        if len_sq > 0.0 {
            let inv_len = 1.0 / len_sq.sqrt();
            Self {
                x: self.x * inv_len,
                y: self.y * inv_len,
                z: self.z * inv_len,
                w: self.w * inv_len,
            }
        } else {
            Self::identity()
        }
    }
    
    pub fn to_euler_angles(&self) -> Vector3 {
        // Extract the components for readability
        let x = self.x;
        let y = self.y; 
        let z = self.z;
        let w = self.w;
        
        // Calculate pitch (x-axis rotation)
        let pitch = f32::atan2(2.0 * (w * x + y * z), 1.0 - 2.0 * (x * x + y * y));
        
        // Calculate yaw (y-axis rotation)
        // Clamp sinp between -1 and 1 to avoid NaN from asin
        let sinp = 2.0 * (w * y - z * x);
        let yaw = if sinp.abs() >= 1.0 {
            f32::copysign(std::f32::consts::PI / 2.0, sinp) // Use 90 degrees if at the poles
        } else {
            f32::asin(sinp)
        };
        
        // Calculate roll (z-axis rotation)
        let roll = f32::atan2(2.0 * (w * z + x * y), 1.0 - 2.0 * (y * y + z * z));
        
        // Return as Vector3 in the order (pitch, yaw, roll)
        Vector3::new(pitch, yaw, roll)
    }
    
}