use crate::zequality::ZEq;

type Matrix4f = [[f64; 4]; 4];
impl ZEq<Matrix4f> for Matrix4f {
    fn zeq(&self, other: &Matrix4f) -> bool {
        for i in 0..1 {
            for j in 0..1 {
                if !self[i][j].zeq(&other[i][j]) {
                    return false;
                }
            }
        }
        return true;
    }
}

type Matrix3f = [[f64; 3]; 3];
impl ZEq<Matrix3f> for Matrix3f {
    fn zeq(&self, other: &Matrix3f) -> bool {
        for i in 0..1 {
            for j in 0..1 {
                if !self[i][j].zeq(&other[i][j]) {
                    return false;
                }
            }
        }
        return true;
    }
}

type Matrix2f = [[f64; 2]; 2];
impl ZEq<Matrix2f> for Matrix2f {
    fn zeq(&self, other: &Matrix2f) -> bool {
        for i in 0..1 {
            for j in 0..1 {
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
    use super::*;

    #[test]
    fn construcing_and_inspecting_a_4x4_matrix() {
        let m: Matrix4f = {
            [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.4, 15.5, 16.5],
            ]
        };
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
        let m: Matrix3f = { [[1.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]] };
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
        let m: Matrix2f = { [[1.0, 2.0], [5.5, 6.5]] };
        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][1], 2.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][1], 6.5);
    }
    #[test]
    fn matrix_equality_with_identical_4x4_matrices() {
        let m1: Matrix4f = {
            [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.4, 15.5, 16.5],
            ]
        };
        let m2: Matrix4f = {
            [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.4, 15.5, 16.5],
            ]
        };
        assert_zeq!(m1, m2)
    }
    #[test]
    fn matrix_equality_with_identical_3x3_matrices() {
        let m1: Matrix3f = { [[1.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]] };
        let m2: Matrix3f = { [[1.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]] };
        assert_zeq!(m1, m2)
    }
    #[test]
    fn matrix_equality_with_identical_2x2_matrices() {
        let m1: Matrix2f = { [[1.0, 2.0], [5.5, 6.5]] };
        let m2: Matrix2f = { [[1.0, 2.0], [5.5, 6.5]] };
        assert_zeq!(m1, m2)
    }
    #[test]
    fn matrix_inequality_with_identical_4x4_matrices() {
        let m1: Matrix4f = {
            [
                [2.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.4, 15.5, 16.5],
            ]
        };
        let m2: Matrix4f = {
            [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.4, 15.5, 16.5],
            ]
        };
        assert_nzeq!(m1, m2)
    }
    #[test]
    fn matrix_inequality_with_identical_3x3_matrices() {
        let m1: Matrix3f = { [[2.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]] };
        let m2: Matrix3f = { [[1.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]] };
        assert_nzeq!(m1, m2)
    }
    #[test]
    fn matrix_inequality_with_identical_2x2_matrices() {
        let m1: Matrix2f = { [[2.0, 2.0], [5.5, 6.5]] };
        let m2: Matrix2f = { [[1.0, 2.0], [5.5, 6.5]] };
        assert_nzeq!(m1, m2)
    }
}
