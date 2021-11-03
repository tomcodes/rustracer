// use std::fmt;
use std::ops;
use std::ops::Sub;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 {
            e: [e0, e1, e2]
        }
    }

    pub fn x(self) -> f32 {
        self.e[0]
    }

    pub fn y(self) -> f32 {
        self.e[1]
    }

    pub fn z(self) -> f32 {
        self.e[2]
    }

    pub fn r(self) -> f32 {
        self.e[0]
    }

    pub fn g(self) -> f32 {
        self.e[1]
    }

    pub fn b(self) -> f32 {
        self.e[2]
    }

    pub fn length(self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn length_squared(self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        *v / v.length()
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
        v1.e[0] * v2.e[0]
        + v1.e[1] * v2.e[1]
        + v1.e[2] * v2.e[2]
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

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
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

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [
                rhs.e[0] * self,
                rhs.e[1] * self,
                rhs.e[2] * self,
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

// impl ops::Div<Vec3> for f32 {
//     type Output = Vec3;

//     fn div(self, rhs: Vec3) -> Self::Output {
//         Vec3 {
//             e: [
//                 rhs.e[0] / self,
//                 rhs.e[1] / self,
//                 rhs.e[2] / self,
//             ]
//         }
//     }
// }

// impl fmt::Display for Vec3 {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "({}, {}, {})", self.e[0], self.e[1], self.e[2])
//     }
// }

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
        assert_eq!(
            Vec3::new(5.0, 9.0, 8.0) - Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(4.0, 7.0, 5.0)
        );
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
