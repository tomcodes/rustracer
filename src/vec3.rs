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

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] / rhs,
                self.e[1] / rhs,
                self.e[2] / rhs,
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_add() {
        assert_eq!(
            Vec3::new(2.0, 3.0, 5.0) + Vec3::new(3.0, 12.0, 2.0),
            Vec3::new(5.0, 15.0, 7.0)
        );

        assert_ne!(
            Vec3::new(2.0, 3.0, 5.0) + Vec3::new(3.0, 12.0, 2.0),
            Vec3::new(6.0, 15.0, 7.0)
        );
    }

    #[test]
    fn test_vec3_sub() {

    }

    #[test]
    fn test_vec3_mul() {
        assert_eq!(
            Vec3::new(3.0, 2.0, 0.0) * 15.0,
            Vec3::new(45.0, 30.0, 0.0)
        );

        assert_ne!(
            Vec3::new(3.0, 2.0, 0.0) * 11.0,
            Vec3::new(45.0, 30.0, 0.0)
        );
    }

    #[test]
    fn test_vec3_div() {
        assert_eq!(
            Vec3::new(3.0, 4.0, 6.0) / 2.0,
            Vec3::new(1.5, 2.0, 3.0)
        );
    }

    #[test]
    fn test_vec3_length() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0).length(),
            (14f32).sqrt()
        );
    }

    #[test]
    fn test_vec3_length_squared() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0).length_squared(),
            14.0
        );
    }
}
