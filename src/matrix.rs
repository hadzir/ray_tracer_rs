use crate::tuple::VTuple;
use crate::zequality::ZEq;
use std::convert::From;
use std::ops;

type Matrix4fArrayRow = [f64; 4];
type Matrix4fArray = [Matrix4fArrayRow; 4];
type Matrix3fArrayRow = [f64; 3];
type Matrix3fArray = [Matrix3fArrayRow; 3];
type Matrix2fArrayRow = [f64; 2];
type Matrix2fArray = [Matrix2fArrayRow; 2];
#[derive(Debug, Clone, Copy)]
pub struct Matrix4f {
    data: Matrix4fArray,
}
impl Matrix4f {
    pub fn default() -> Matrix4f {
        Matrix4f::from([[0.0; 4]; 4])
    }
    pub fn identity() -> Matrix4f {
        let mut result = Matrix4f::default();
        for i in 0..4 {
            result[i][i] = 1.0;
        }
        result
    }
}
impl From<Matrix4fArray> for Matrix4f {
    fn from(data: Matrix4fArray) -> Self {
        Matrix4f { data }
    }
}
impl ops::Index<usize> for Matrix4f {
    type Output = Matrix4fArrayRow;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl ops::IndexMut<usize> for Matrix4f {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
impl ops::Mul<Matrix4f> for Matrix4f {
    type Output = Matrix4f;
    fn mul(self, other: Matrix4f) -> Self::Output {
        let mut result = Matrix4f::default();
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self[i][0] * other[0][j]
                    + self[i][1] * other[1][j]
                    + self[i][2] * other[2][j]
                    + self[i][3] * other[3][j];
            }
        }
        result
    }
}
impl ops::Mul<VTuple> for Matrix4f {
    type Output = VTuple;
    fn mul(self, other: VTuple) -> Self::Output {
        VTuple::new(
            self[0][0] * other.x
                + self[0][1] * other.y
                + self[0][2] * other.z
                + self[0][3] * other.w,
            self[1][0] * other.x
                + self[1][1] * other.y
                + self[1][2] * other.z
                + self[1][3] * other.w,
            self[2][0] * other.x
                + self[2][1] * other.y
                + self[2][2] * other.z
                + self[2][3] * other.w,
            self[3][0] * other.x
                + self[3][1] * other.y
                + self[3][2] * other.z
                + self[3][3] * other.w,
        )
    }
}
impl ZEq<Matrix4f> for Matrix4f {
    fn zeq(&self, other: &Matrix4f) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if !self[i][j].zeq(&other[i][j]) {
                    return false;
                }
            }
        }
        return true;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Matrix3f {
    data: Matrix3fArray,
}
impl Matrix3f {
    pub fn default() -> Matrix3f {
        Matrix3f::from([[0.0; 3]; 3])
    }
}
impl From<Matrix3fArray> for Matrix3f {
    fn from(data: Matrix3fArray) -> Self {
        Matrix3f { data }
    }
}
impl ops::Index<usize> for Matrix3f {
    type Output = Matrix3fArrayRow;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl ops::IndexMut<usize> for Matrix3f {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
impl ZEq<Matrix3f> for Matrix3f {
    fn zeq(&self, other: &Matrix3f) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if !self[i][j].zeq(&other[i][j]) {
                    return false;
                }
            }
        }
        return true;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Matrix2f {
    data: Matrix2fArray,
}
impl Matrix2f {
    pub fn default() -> Matrix2f {
        Matrix2f::from([[0.0; 2]; 2])
    }
}
impl From<Matrix2fArray> for Matrix2f {
    fn from(data: Matrix2fArray) -> Self {
        Matrix2f { data }
    }
}
impl ops::Index<usize> for Matrix2f {
    type Output = Matrix2fArrayRow;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl ops::IndexMut<usize> for Matrix2f {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
impl ZEq<Matrix2f> for Matrix2f {
    fn zeq(&self, other: &Matrix2f) -> bool {
        for i in 0..2 {
            for j in 0..2 {
                if !self[i][j].zeq(&other[i][j]) {
                    return false;
                }
            }
        }
        return true;
    }
}

#[cfg(test)]
mod test {
    use crate::tuple::VTuple;

    use super::*;

    #[test]
    fn construcing_and_inspecting_a_4x4_matrix() {
        let m: Matrix4f = Matrix4f::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.4, 15.5, 16.5],
        ]);
        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][1], 2.0);
        assert_eq!(m[0][2], 3.0);
        assert_eq!(m[0][3], 4.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][1], 6.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[1][3], 8.5);
        assert_eq!(m[2][0], 9.0);
        assert_eq!(m[2][1], 10.0);
        assert_eq!(m[2][2], 11.0);
        assert_eq!(m[2][3], 12.0);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][1], 14.4);
        assert_eq!(m[3][2], 15.5);
        assert_eq!(m[3][3], 16.5);
    }
    #[test]
    fn construcing_and_inspecting_a_3x3_matrix() {
        let m: Matrix3f = Matrix3f::from([[1.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]]);
        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][1], 2.0);
        assert_eq!(m[0][2], 3.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][1], 6.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[2][0], 9.0);
        assert_eq!(m[2][1], 10.0);
        assert_eq!(m[2][2], 11.0);
    }
    #[test]
    fn construcing_and_inspecting_a_2x2_matrix() {
        let m = Matrix2f::from([[1.0, 2.0], [5.5, 6.5]]);
        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][1], 2.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][1], 6.5);
    }
    #[test]
    fn matrix_equality_with_identical_4x4_matrices() {
        let m1: Matrix4f = Matrix4f::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.4, 15.5, 16.5],
        ]);
        let m2: Matrix4f = Matrix4f::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.4, 15.5, 16.5],
        ]);
        assert_zeq!(m1, m2)
    }
    #[test]
    fn matrix_equality_with_identical_3x3_matrices() {
        let m1: Matrix3f = Matrix3f::from([[2.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]]);
        let m2: Matrix3f = Matrix3f::from([[2.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]]);
        assert_zeq!(m1, m2)
    }
    #[test]
    fn matrix_equality_with_identical_2x2_matrices() {
        let m1: Matrix2f = Matrix2f::from([[1.0, 2.0], [5.5, 6.5]]);
        let m2: Matrix2f = Matrix2f::from([[1.0, 2.0], [5.5, 6.5]]);
        assert_zeq!(m1, m2)
    }
    #[test]
    fn matrix_inequality_with_identical_4x4_matrices() {
        let m1: Matrix4f = Matrix4f::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.4, 15.5, 16.5],
        ]);
        let m2: Matrix4f = Matrix4f::from([
            [2.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.4, 15.5, 16.5],
        ]);
        assert_nzeq!(m1, m2)
    }
    #[test]
    fn matrix_inequality_with_identical_3x3_matrices() {
        let m1: Matrix3f = Matrix3f::from([[1.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]]);
        let m2: Matrix3f = Matrix3f::from([[2.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]]);
        assert_nzeq!(m1, m2)
    }
    #[test]
    fn matrix_inequality_with_identical_2x2_matrices() {
        let m1: Matrix2f = Matrix2f::from([[1.0, 2.0], [5.5, 6.5]]);
        let m2: Matrix2f = Matrix2f::from([[2.0, 2.0], [5.5, 1.5]]);
        assert_nzeq!(m1, m2)
    }
    #[test]
    fn multiplying_two_4x4_matrices() {
        let m1: Matrix4f = Matrix4f::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2: Matrix4f = Matrix4f::from([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let result: Matrix4f = m1 * m2;
        let expected_result: Matrix4f = Matrix4f::from([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        assert_zeq!(result, expected_result)
    }
    #[test]
    fn multiplying_4x4_matrix_by_the_identity_matrix() {
        let m1: Matrix4f = Matrix4f::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2: Matrix4f = Matrix4f::identity();

        let result: Matrix4f = m1 * m2;
        let expected_result: Matrix4f = m1;

        assert_zeq!(result, expected_result)
    }
    #[test]
    fn multiplying_4x4_matrix_by_point() {
        let m = Matrix4f::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let v = VTuple::point(1.0, 2.0, 3.0);

        let result: VTuple = m * v;
        let expected_result = VTuple::point(18.0, 24.0, 33.0);

        assert_zeq!(result, expected_result)
    }
}
