use std::fmt::Display;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 {
            x,
            y,
            z,
        }
    }
    pub fn x(self) -> f64{
        self.x
    }

    pub fn y(self) -> f64{
        self.y
    }

    pub fn z(self) -> f64{
        self.z
    }
    
    pub fn length_squared(self) -> f64 {
            self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot_prod(self, other_vec: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other_vec.x(),
            y: self.y * other_vec.y(),
            z: self.z * other_vec.z(),
        }
    }

    pub fn cross_prod(self, other_vec: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other_vec.z() - self.z * other_vec.y(),
            y: self.z * other_vec.x() - self.x * other_vec.z(),
            z: self.x * other_vec.y() - self.y * other_vec.x(),
        }
    }

    pub fn unit_vector(v: Vec3) -> Vec3{
        v / v.length()
    }
}
