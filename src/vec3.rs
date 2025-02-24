use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug, Copy, Clone)]
pub struct Vec3(f64, f64, f64);

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        return self.0 * self.0 + self.1 * self.1 + self.2 * self.2;
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.0, self.2 - rhs.2)
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    return u.x() * v.x() + u.y() * v.y() + u.z() * v.z();
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    return Vec3::new(
        u.y() * v.z() - u.z() * v.y(),
        u.y() * v.x() - u.x() * v.z(),
        u.x() * v.y() - u.y() * v.x(),
    );
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    return v / v.length();
}
