use std::ops::{Add, Div, Mul, Sub};

// allow printing (useful for debugging)
// Clone = explicit .clone(), Copy = implicit copying when need
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.x, self * other.y, self * other.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Vec3 {
        Vec3::new(self.x / other, self.y / other, self.z / other)
    }
}

// Type alias
pub type Point3 = Vec3;

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn test_add() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let w = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v + w, Vec3::new(2.0, 4.0, 6.0))
    }

    #[test]
    fn test_mul() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(10.0 * v, Vec3::new(10.0, 20.0, 30.0));
    }
}
