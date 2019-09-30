use crate::core::types::Real;
use std::ops;

#[derive(Default, Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: Real,
    pub y: Real,
    pub z: Real,
    // four word alignment in memory, not sure if it's needed
    pad: Real,
}

impl Vec3 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_vec(vec: Vec<Real>) -> Vec3 {
        Vec3 {
            x: vec[0],
            y: vec[1],
            z: vec[2],
            ..Vec3::default()
        }
    }

    // is it overhead?
    pub fn build(&mut self) -> Self {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
            ..Default::default()
        }
    }

    pub fn set_values(&mut self, x: Real, y: Real, z: Real) -> &mut Self {
        self.x = x;
        self.y = y;
        self.z = z;
        self
    }

    pub fn invert(&mut self) -> &mut Self {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }

    pub fn magnitude(&self) -> Real {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn square_magnitude(&self) -> Real {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn normalize(&mut self) -> &mut Self {
        let m = self.magnitude();
        if m > 0.0 {
            *self *= (1 as Real) / m;
        }
        self
    }

    pub fn add_scaled(&mut self, v: Vec3, scale: Real) -> &mut Self {
        self.x += v.x * scale;
        self.y += v.y * scale;
        self.z += v.z * scale;
        self
    }

    pub fn component_product(&self, v: Vec3) -> Self {
        Vec3 {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
            ..Default::default()
        }
    }

    pub fn component_product_update(&mut self, v: Vec3) -> &mut Self {
        self.x *= v.x;
        self.y *= v.y;
        self.z *= v.z;
        self
    }

    pub fn scalar_product(&self, v: Vec3) -> Real {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn vector_product(&self, v: Vec3) -> Self {
        Vec3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
            ..Default::default()
        }
    }

    /// this algorithm is designed for right-handed system
    pub fn make_orthogonal_basis(a: &mut Vec3, b: &mut Vec3, c: &mut Vec3) {
        a.normalize();
        *c = *a % *b;
        if c.square_magnitude() == 0.0 {
            // TODO: generate an error?
            return;
        }
        c.normalize();
        *b = *c % *a;
    }

    pub fn set_to_zero(&mut self) -> &mut Self {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
        self
    }
}

impl ops::MulAssign<Real> for Vec3 {
    fn mul_assign(&mut self, v: Real) {
        self.x *= v;
        self.y *= v;
        self.z *= v;
    }
}

impl ops::Mul<Real> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Real) -> Self::Output {
        Vec3 {
            x: self.x * v,
            y: self.y * v,
            z: self.z * v,
            ..Default::default()
        }
    }
}

impl ops::Mul<Real> for &mut Vec3 {
    type Output = Vec3;

    fn mul(self, v: Real) -> Self::Output {
        Vec3 {
            x: self.x * v,
            y: self.y * v,
            z: self.z * v,
            ..Default::default()
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
            ..Default::default()
        }
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, v: Vec3) {
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
            ..Default::default()
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Real;

    fn mul(self, v: Vec3) -> Self::Output {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

impl ops::RemAssign<Vec3> for Vec3 {
    fn rem_assign(&mut self, v: Vec3) {
        self.x = self.y * v.z - self.z * v.y;
        self.y = self.z * v.x - self.x * v.z;
        self.z = self.x * v.y - self.y * v.x;
    }
}

impl ops::Rem<Vec3> for Vec3 {
    type Output = Vec3;

    fn rem(self, v: Vec3) -> Self::Output {
        self.vector_product(v)
    }
}
