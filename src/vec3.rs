use std::fmt::Display;
use std::ops::{Add, Div, Mul, Range, Sub};

use rand::Rng;
#[derive(Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn x(self) -> f64 {
        self.x
    }

    pub fn y(self) -> f64 {
        self.y
    }
    pub fn z(self) -> f64 {
        self.z
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.y,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn len(&self) -> f64 {
        f64::sqrt(self.len_squared())
    }

    pub fn len_squared(&self) -> f64 {
        f64::powf(self.x, 2.0) + f64::powf(self.y, 2.0) + f64::powf(self.z, 2.0)
    }

    pub fn unit_vector(&self) -> Vec3 {
        (1.00 / self.len()) * self
    }

    pub fn reflect(self, n: Vec3) -> Self {
        self - 2.0 * self.dot(&n) * n
    }

    pub fn is_near_zero(self) -> bool {
        const EPS: f64 = 1.0e-8;
        self.x.abs() < EPS && self.y < EPS && self.z < EPS
    }

    pub fn format_color(self, samples_per_pixel: u64) -> String {
        let ir = (256.0
            * (self.x / (samples_per_pixel as f64))
                .sqrt()
                .clamp(0.0, 0.999)) as u64;
        let ig = (256.0
            * (self.y / (samples_per_pixel as f64))
                .sqrt()
                .clamp(0.0, 0.999)) as u64;

        let ib = (256.0
            * (self.z / (samples_per_pixel as f64))
                .sqrt()
                .clamp(0.0, 0.999)) as u64;

        format!("{} {} {}", ir, ig, ib)
    }

    pub fn random(r: Range<f64>) -> Self {
        let mut rng = rand::thread_rng();

        Vec3 {
            x: rng.gen_range(r.clone()),
            y: rng.gen_range(r.clone()),
            z: rng.gen_range(r.clone()),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        {
            loop {
                let vec = Vec3::random(-1.0..1.0);
                if vec.len() < 1.0 {
                    return vec;
                }
            }
        }
    }

    pub fn refract(self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = ((-1.0) * self).dot(&n).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -1.0 * (1.0 - r_out_perp.len().powi(2)).abs().sqrt() * n;
        r_out_perp + r_out_parallel
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

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.00 / rhs)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = format!("{} {} {}", self.x, self.y, self.z);

        write!(f, "{}", data)
    }
}
