use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};
#[derive(Copy, Clone)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn x(self) -> f32 {
        self.x
    }

    pub fn y(self) -> f32 {
        self.y
    }
    pub fn z(self) -> f32 {
        self.z
    }

    fn dot(&self, vec: &Vec3) -> f32 {
        self.x * vec.x + self.y * vec.y + self.z + vec.z
    }

    fn cross(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * vec.z - self.z * vec.y,
            y: self.z * vec.x - self.x * vec.y,
            z: self.x * vec.y - self.y * vec.x,
        }
    }

    fn len(&self) -> f32 {
        f32::sqrt(self.len_squared())
    }

    fn len_squared(&self) -> f32 {
        f32::powf(self.x, 2.0) + f32::powf(self.y, 2.0) + f32::powf(self.z, 2.0)
    }

    pub fn unit_vector(&self) -> Vec3 {
        (1.00 / self.len()) * self
    }

    pub fn to_pixel_row(&self) -> String {
        format!(
            "{} {} {}\n",
            (&self.x * 255.999) as i32,
            (&self.y * 255.999) as i32,
            (&self.z * 255.999) as i32
        )
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}
impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.00 / rhs)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = format!("{} {} {}", self.x, self.y, self.z);

        write!(f, "{}", data)
    }
}
