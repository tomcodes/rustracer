use std::fmt;
use std::ops;

pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 {
            e: [e0, e1, e2]
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ]
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] * rhs,
                self.e[1] * rhs,
                self.e[2] * rhs,
            ]
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.e[0], self.e[1], self.e[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_add() {

    }

    #[test]
    fn test_vec3_sub() {

    }

    #[test]
    fn test_vec3_mul() {

    }

    #[test]
    fn test_vec3_div() {

    }
}
