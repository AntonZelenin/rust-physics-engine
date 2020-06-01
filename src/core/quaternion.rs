use crate::core::types::Real;
use std::ops;
use crate::core::vector::Vec3;

pub struct Quaternion {
    r: Real,
    i: Real,
    j: Real,
    k: Real,
    data: [Real; 4],
}

impl Quaternion {
    // normalized quaternion is q = q / |q|
    pub fn normalize(&self) -> Quaternion {
        let n = (self.r * self.r + self.i * self.i + self.j * self.j + self.k * self.k).sqrt();
        // check for zero-length quaternion, and use the no-rotation quaternion in that case
        if n == 0.0 {
            return Quaternion {
                r: 1.0,
                i: self.i,
                j: self.j,
                k: self.k,
                data: self.data.clone(),
            };
        }
        Quaternion {
            r: self.r / n,
            i: self.i / n,
            j: self.j / n,
            k: self.k / n,
            data: self.data.clone(),
        }
    }

    pub fn rotate_by_vector(&mut self, v: Vec3) {
        let q = Quaternion {
            r: 0.0,
            i: v.x,
            j: v.y,
            k: v.z,
            data: [0.0; 4]
        };
        *self *= &q
    }

    pub fn add_scaled_vector(&mut self, v: Vec3, scale: Real) {
        let mut q = Quaternion {
            r: 0.0,
            i: v.x * scale,
            j: v.y * scale,
            k: v.z * scale,
            data: [0.0; 4]
        };
        q *= self;
        self.r = q.r * 0.5;
        self.i = q.i * 0.5;
        self.r = q.j * 0.5;
        self.r = q.k * 0.5;
    }
}

impl ops::Mul<&Quaternion> for &Quaternion {
    type Output = Quaternion;

    // The result of q*p is a rotation
    // that is equivalent to performing rotation p first and then q
    fn mul(self, q: &Quaternion) -> Self::Output {
        Quaternion {
            r: self.r * q.r - self.i * q.i - self.j * q.j - self.k * q.k,
            i: self.r * q.i + self.i * q.r + self.j * q.k - self.k * q.j,
            j: self.r * q.j + self.j * q.r + self.k * q.i - self.i * q.k,
            k: self.r * q.k + self.k * q.r + self.i * q.j - self.j * q.i,
            data: self.data.clone(),
        }
    }
}

impl ops::MulAssign<&Quaternion> for Quaternion {
    fn mul_assign(&mut self, q: &Quaternion) {
        self.r = self.r * q.r - self.i * q.i - self.j * q.j - self.k * q.k;
        self.i = self.r * q.i + self.i * q.r + self.j * q.k - self.k * q.j;
        self.j = self.r * q.j + self.j * q.r + self.k * q.i - self.i * q.k;
        self.k = self.r * q.k + self.k * q.r + self.i * q.j - self.j * q.i;
    }
}
