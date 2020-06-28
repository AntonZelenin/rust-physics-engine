use crate::quaternion::Quaternion;
use crate::types::Real;
use crate::vector::Vec3;
use std::ops;

const MATRIX_3_SIZE: usize = 9;
const MATRIX_4_SIZE: usize = 12;

/**
* Holds a 3 x 3 row major matrix representing a transformation in
* 3D space that does not include a translational component. This
* matrix is not padded to produce an aligned structure.
*/
pub struct Matrix3 {
    pub data: [Real; MATRIX_3_SIZE],
}

/**
* This is a 3x4 matrix
*
* Holds a transform matrix, consisting of a rotation matrix and
* a position. The matrix has 12 elements, and it is assumed that the
* remaining four are (0,0,0,1), producing a homogeneous matrix.
*/
pub struct Matrix4 {
    pub data: [Real; MATRIX_4_SIZE],
}

impl Matrix3 {
    pub fn new() -> Self {
        Matrix3 {
            data: [0.0; MATRIX_3_SIZE],
        }
    }

    pub fn transform(&self, v: &Vec3) -> Vec3 {
        self * v
    }

    pub fn invert(&mut self) {
        if let Some(m) = self.get_inverse() {
            *self = m;
        }
        // TODO what if None?
    }

    pub fn get_inverse(&self) -> Option<Matrix3> {
        let mut result = Matrix3::new();
        let det = self.get_determinant();
        if det == 0.0 {
            return None;
        }
        let invd: Real = 1.0 / det;

        result.data[0] = (self.data[4] * self.data[8] - self.data[5] * self.data[7]) * invd;
        result.data[1] = -(self.data[1] * self.data[8] - self.data[2] * self.data[7]) * invd;
        result.data[2] = (self.data[1] * self.data[5] - self.data[2] * self.data[4]) * invd;
        result.data[3] = -(self.data[3] * self.data[8] - self.data[5] * self.data[6]) * invd;
        result.data[4] = (self.data[0] * self.data[8] - self.data[2] * self.data[6]) * invd;
        result.data[5] = -(self.data[0] * self.data[5] - self.data[2] * self.data[3]) * invd;
        result.data[6] = (self.data[3] * self.data[7] - self.data[4] * self.data[6]) * invd;
        result.data[7] = -(self.data[0] * self.data[7] - self.data[1] * self.data[6]) * invd;
        result.data[8] = (self.data[0] * self.data[4] - self.data[1] * self.data[3]) * invd;

        Some(result)
    }

    fn get_determinant(&self) -> Real {
        let t1 = self.data[0] * self.data[4];
        let t2 = self.data[0] * self.data[5];
        let t3 = self.data[1] * self.data[3];
        let t4 = self.data[2] * self.data[3];
        let t5 = self.data[1] * self.data[6];
        let t6 = self.data[2] * self.data[6];
        t1 * self.data[8] - t2 * self.data[7] - t3 * self.data[8]
            + t4 * self.data[7]
            + t5 * self.data[5]
            - t6 * self.data[4]
    }

    pub fn transpose(&mut self) {
        *self = self.get_transpose()
    }

    pub fn get_transpose(&self) -> Matrix3 {
        let mut result = Matrix3::new();
        result.data[0] = self.data[0];
        result.data[1] = self.data[3];
        result.data[2] = self.data[6];
        result.data[3] = self.data[1];
        result.data[4] = self.data[4];
        result.data[5] = self.data[7];
        result.data[6] = self.data[2];
        result.data[7] = self.data[5];
        result.data[8] = self.data[8];
        result
    }

    // Sets this matrix to be the rotation matrix corresponding to
    // the given quaternion.
    // TODO refactor?
    pub fn set_rotation(&mut self, q: &Quaternion) {
        self.data[0] = 1.0 - (2.0 * q.j * q.j + 2.0 * q.k * q.k);
        self.data[1] = 2.0 * q.i * q.j + 2.0 * q.k * q.r;
        self.data[2] = 2.0 * q.i * q.k - 2.0 * q.j * q.r;
        self.data[3] = 2.0 * q.i * q.j - 2.0 * q.k * q.r;
        self.data[4] = 1.0 - (2.0 * q.i * q.i + 2.0 * q.k * q.k);
        self.data[5] = 2.0 * q.j * q.k + 2.0 * q.i * q.r;
        self.data[6] = 2.0 * q.i * q.k + 2.0 * q.j * q.r;
        self.data[7] = 2.0 * q.j * q.k - 2.0 * q.i * q.r;
        self.data[8] = 1.0 - (2.0 * q.i * q.i + 2.0 * q.j * q.j);
    }
}

impl Matrix4 {
    pub fn new() -> Self {
        Matrix4 {
            data: [0.0; MATRIX_4_SIZE],
        }
    }

    pub fn transform(&self, v: &Vec3) -> Vec3 {
        self * v
    }

    // Transform the given vector by the transformational inverse
    // of this matrix.
    pub fn transform_inverse(&self, mut v: Vec3) -> Vec3 {
        v.x -= self.data[3];
        v.y -= self.data[7];
        v.z -= self.data[11];
        self.transform_inverse_direction(v)
    }

    pub fn transform_direction(&self, v: Vec3) -> Vec3 {
        Vec3 {
            x: v.x * self.data[0] + v.y * self.data[1] + v.z * self.data[2],
            y: v.x * self.data[4] + v.y * self.data[5] + v.z * self.data[6],
            z: v.x * self.data[8] + v.y * self.data[9] + v.z * self.data[10],
            ..Default::default()
        }
    }

    pub fn transform_inverse_direction(&self, v: Vec3) -> Vec3 {
        Vec3 {
            x: v.x * self.data[0] + v.y * self.data[4] + v.z * self.data[8],
            y: v.x * self.data[1] + v.y * self.data[5] + v.z * self.data[9],
            z: v.x * self.data[2] + v.y * self.data[6] + v.z * self.data[10],
            ..Default::default()
        }
    }

    pub fn invert(&mut self) {
        if let Some(m) = self.get_inverse() {
            *self = m;
        }
        // TODO what if None?
    }

    pub fn get_inverse(&self) -> Option<Matrix4> {
        let det = self.get_determinant();
        if det == 0.0 {
            return None;
        }
        let mut result = Matrix4::new();
        let invd = 1.0 / det;
        result.data[0] = (-self.data[9] * self.data[6] + self.data[5] * self.data[10]) * invd;
        result.data[4] = (self.data[8] * self.data[6] - self.data[4] * self.data[10]) * invd;
        result.data[8] = (-self.data[8] * self.data[5] + self.data[4] * self.data[9]) * invd;
        result.data[1] = (self.data[9] * self.data[2] - self.data[1] * self.data[10]) * invd;
        result.data[5] = (-self.data[8] * self.data[2] + self.data[0] * self.data[10]) * invd;
        result.data[9] = (self.data[8] * self.data[1] - self.data[0] * self.data[9]) * invd;
        result.data[2] = (-self.data[5] * self.data[2] + self.data[1] * self.data[6]) * invd;
        result.data[6] = (self.data[4] * self.data[2] - self.data[0] * self.data[6]) * invd;
        result.data[10] = (-self.data[4] * self.data[1] + self.data[0] * self.data[5]) * invd;
        result.data[3] = (self.data[9] * self.data[6] * self.data[3]
            - self.data[5] * self.data[10] * self.data[3]
            - self.data[9] * self.data[2] * self.data[7]
            + self.data[1] * self.data[10] * self.data[7]
            + self.data[5] * self.data[2] * self.data[11]
            - self.data[1] * self.data[6] * self.data[11])
            * invd;
        result.data[7] = (-self.data[8] * self.data[6] * self.data[3]
            + self.data[4] * self.data[10] * self.data[3]
            + self.data[8] * self.data[2] * self.data[7]
            - self.data[0] * self.data[10] * self.data[7]
            - self.data[4] * self.data[2] * self.data[11]
            + self.data[0] * self.data[6] * self.data[11])
            * invd;
        result.data[11] = (self.data[8] * self.data[5] * self.data[3]
            - self.data[4] * self.data[9] * self.data[3]
            - self.data[8] * self.data[1] * self.data[7]
            + self.data[0] * self.data[9] * self.data[7]
            + self.data[4] * self.data[1] * self.data[11]
            - self.data[0] * self.data[5] * self.data[11])
            * invd;

        Some(result)
    }

    pub fn get_determinant(&self) -> Real {
        self.data[8] * self.data[5] * self.data[2]
            + self.data[4] * self.data[9] * self.data[2]
            + self.data[8] * self.data[1] * self.data[6]
            - self.data[0] * self.data[9] * self.data[6]
            - self.data[4] * self.data[1] * self.data[10]
            + self.data[0] * self.data[5] * self.data[10]
    }

    pub fn set_orientation_and_pos(&mut self, q: &Quaternion, pos: &Vec3) {
        self.data[0] = 1.0 - (2.0 * q.j * q.j + 2.0 * q.k * q.k);
        self.data[1] = 2.0 * q.i * q.j + 2.0 * q.k * q.r;
        self.data[2] = 2.0 * q.i * q.k - 2.0 * q.j * q.r;
        self.data[3] = pos.x;
        self.data[4] = 2.0 * q.i * q.j - 2.0 * q.k * q.r;
        self.data[5] = 1.0 - (2.0 * q.i * q.i + 2.0 * q.k * q.k);
        self.data[6] = 2.0 * q.j * q.k + 2.0 * q.i * q.r;
        self.data[7] = pos.y;
        self.data[8] = 2.0 * q.i * q.k + 2.0 * q.j * q.r;
        self.data[9] = 2.0 * q.j * q.k - 2.0 * q.i * q.r;
        self.data[10] = 1.0 - (2.0 * q.i * q.i + 2.0 * q.j * q.j);
        self.data[11] = pos.z;
    }
}

impl ops::Mul<&Vec3> for &Matrix3 {
    type Output = Vec3;

    // TODO should I own values here and in Vec3 or use references?
    fn mul(self, v: &Vec3) -> Self::Output {
        Vec3 {
            x: v.x * self.data[0] + v.y * self.data[1] + v.z * self.data[2],
            y: v.x * self.data[3] + v.y * self.data[4] + v.z * self.data[5],
            z: v.x * self.data[6] + v.y * self.data[7] + v.z * self.data[8],
            ..Default::default()
        }
    }
}

impl ops::Mul<Matrix3> for &Matrix3 {
    type Output = Matrix3;

    fn mul(self, m: Matrix3) -> Self::Output {
        Matrix3 {
            data: [
                self.data[0] * m.data[0] + self.data[1] * m.data[3] + self.data[2] * m.data[6],
                self.data[0] * m.data[1] + self.data[1] * self.data[4] + self.data[2] * m.data[7],
                self.data[0] * m.data[2] + self.data[1] * m.data[5] + self.data[2] * m.data[8],
                self.data[3] * m.data[0] + self.data[4] * m.data[3] + self.data[5] * m.data[6],
                self.data[3] * m.data[1] + self.data[4] * m.data[4] + self.data[5] * m.data[7],
                self.data[3] * m.data[2] + self.data[4] * m.data[5] + self.data[5] * m.data[8],
                self.data[6] * m.data[0] + self.data[7] * m.data[3] + self.data[8] * m.data[6],
                self.data[6] * m.data[1] + self.data[7] * m.data[4] + self.data[8] * m.data[7],
                self.data[6] * m.data[2] + self.data[7] * m.data[5] + self.data[8] * m.data[8],
            ],
        }
    }
}

impl ops::Mul<&Vec3> for &Matrix4 {
    type Output = Vec3;

    fn mul(self, v: &Vec3) -> Self::Output {
        Vec3 {
            x: v.x * self.data[0] + v.y * self.data[1] + v.z * self.data[2] + self.data[3],
            y: v.x * self.data[4] + v.y * self.data[5] + v.z * self.data[6] + self.data[7],
            z: v.x * self.data[8] + v.y * self.data[9] + v.z * self.data[10] + self.data[11],
            ..Default::default()
        }
    }
}

impl ops::MulAssign<Matrix3> for &mut Matrix3 {
    // the matrix AB is a transformation that would result from first doing B, then doing A.
    // In other words, the order of the transformations is the opposite of the order of the
    // matrices in the multiplication
    fn mul_assign(&mut self, m: Matrix3) {
        let mut t1: Real;
        let mut t2: Real;
        let mut t3: Real;

        t1 = self.data[0] * m.data[0] + self.data[1] * m.data[3] + self.data[2] * m.data[6];
        t2 = self.data[0] * m.data[1] + self.data[1] * m.data[4] + self.data[2] * m.data[7];
        t3 = self.data[0] * m.data[2] + self.data[1] * m.data[5] + self.data[2] * m.data[8];
        self.data[0] = t1;
        self.data[1] = t2;
        self.data[2] = t3;

        t1 = self.data[3] * m.data[0] + self.data[4] * m.data[3] + self.data[5] * m.data[6];
        t2 = self.data[3] * m.data[1] + self.data[4] * m.data[4] + self.data[5] * m.data[7];
        t3 = self.data[3] * m.data[2] + self.data[4] * m.data[5] + self.data[5] * m.data[8];
        self.data[3] = t1;
        self.data[4] = t2;
        self.data[5] = t3;

        t1 = self.data[6] * m.data[0] + self.data[7] * m.data[3] + self.data[8] * m.data[6];
        t2 = self.data[6] * m.data[1] + self.data[7] * m.data[4] + self.data[8] * m.data[7];
        t3 = self.data[6] * m.data[2] + self.data[7] * m.data[5] + self.data[8] * m.data[8];
        self.data[6] = t1;
        self.data[7] = t2;
        self.data[8] = t3;
    }
}

impl ops::Mul<Matrix4> for &Matrix4 {
    type Output = Matrix4;

    // Multiply two 3x4 matrices as if they're 4x4 with the last row 0 0 0 1
    fn mul(self, m: Matrix4) -> Self::Output {
        let mut result = Matrix4::new();

        result.data[0] =
            m.data[0] * self.data[0] + m.data[4] * self.data[1] + m.data[8] * self.data[2];
        result.data[4] =
            m.data[0] * self.data[4] + m.data[4] * self.data[5] + m.data[8] * self.data[6];
        result.data[8] =
            m.data[0] * self.data[8] + m.data[4] * self.data[9] + m.data[8] * self.data[10];
        result.data[1] =
            m.data[1] * self.data[0] + m.data[5] * self.data[1] + m.data[9] * self.data[2];
        result.data[5] =
            m.data[1] * self.data[4] + m.data[5] * self.data[5] + m.data[9] * self.data[6];
        result.data[9] =
            m.data[1] * self.data[8] + m.data[5] * self.data[9] + m.data[9] * self.data[10];
        result.data[2] =
            m.data[2] * self.data[0] + m.data[6] * self.data[1] + m.data[10] * self.data[2];
        result.data[6] =
            m.data[2] * self.data[4] + m.data[6] * self.data[5] + m.data[10] * self.data[6];
        result.data[10] =
            m.data[2] * self.data[8] + m.data[6] * self.data[9] + m.data[10] * self.data[10];
        result.data[3] = m.data[3] * self.data[0]
            + m.data[7] * self.data[1]
            + m.data[11] * self.data[2]
            + self.data[3];
        result.data[7] = m.data[3] * self.data[4]
            + m.data[7] * self.data[5]
            + m.data[11] * self.data[6]
            + self.data[7];
        result.data[11] = m.data[3] * self.data[8]
            + m.data[7] * self.data[9]
            + m.data[11] * self.data[10]
            + self.data[11];

        result
    }
}
