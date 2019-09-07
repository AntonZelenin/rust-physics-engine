use std::ops;

type Real = f64;

#[derive(Debug)]
pub struct Vec3 {
    x: Real,
    y: Real,
    z: Real,
    // four word alignment in emory, not sure if needed
    pad: Real,
}

impl Vec3 {
    pub fn new() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            pad: 0.0,
        }
    }

    // is it overhead?
    pub fn build(&mut self) -> Self {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
            pad: 0.0,
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
            pad: 0.0,
        }
    }
}
