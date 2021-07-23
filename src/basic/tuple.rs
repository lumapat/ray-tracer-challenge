use std::ops;

#[derive(Debug, PartialEq)]
pub struct BasicTuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

pub trait Tuple {
    fn is_point(&self) -> bool;
    fn is_vector(&self) -> bool;
}

impl Tuple for BasicTuple {
    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

impl ops::Add for BasicTuple {
    type Output = BasicTuple;

    fn add(self, other: BasicTuple) -> Self::Output {
        BasicTuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl ops::Div<f64> for BasicTuple {
    type Output = BasicTuple;

    fn div(self, rhs: f64) -> Self::Output {
        BasicTuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl ops::Mul<f64> for BasicTuple {
    type Output = BasicTuple;

    fn mul(self, rhs: f64) -> Self::Output {
        BasicTuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl ops::Neg for BasicTuple {
    type Output = BasicTuple;

    fn neg(self) -> Self::Output {
        BasicTuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl ops::Sub for BasicTuple {
    type Output = BasicTuple;

    fn sub(self, other: BasicTuple) -> Self::Output {
        BasicTuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

#[cfg(test)]
mod operation_tests {
    use super::*;

    #[test]
    fn add_tuples() {
        let a1 = BasicTuple{x: 3.0, y: -2.0, z: 5.0, w: 1.0};
        let a2 = BasicTuple{x: -2.0, y: 3.0, z: 1.0, w: 0.0};

        assert_eq!(a1 + a2, BasicTuple{x: 1.0, y: 1.0, z: 6.0, w: 1.0});
    }

    #[test]
    fn subtract_two_points() {
        let p1 = BasicTuple{x: 3.0, y: 2.0, z: 1.0, w: 1.0};
        let p2 = BasicTuple{x: 5.0, y: 6.0, z: 7.0, w: 1.0};

        assert_eq!(p1 - p2, BasicTuple{x: -2.0, y: -4.0, z: -6.0, w: 0.0});
    }

    #[test]
    fn subtract_vector_from_point() {
        let p1 = BasicTuple{x: 3.0, y: 2.0, z: 1.0, w: 1.0};
        let p2 = BasicTuple{x: 5.0, y: 6.0, z: 7.0, w: 0.0};

        assert_eq!(p1 - p2, BasicTuple{x: -2.0, y: -4.0, z: -6.0, w: 1.0});
    }

    #[test]
    fn subtract_two_vectors() {
        let p1 = BasicTuple{x: 3.0, y: 2.0, z: 1.0, w: 0.0};
        let p2 = BasicTuple{x: 5.0, y: 6.0, z: 7.0, w: 0.0};

        assert_eq!(p1 - p2, BasicTuple{x: -2.0, y: -4.0, z: -6.0, w: 0.0});
    }

    #[test]
    fn subtract_zero_vector() {
        let zero = BasicTuple{x: 0.0, y: 0.0, z: 0.0, w: 0.0};
        let v = BasicTuple{x: 1.0, y: -2.0, z: 3.0, w: 0.0};

        assert_eq!(zero - v, BasicTuple{x: -1.0, y: 2.0, z: -3.0, w: 0.0});
    }

    #[test]
    fn negate_tuple() {
        let a = BasicTuple{x: 1.0, y: -2.0, z: 3.0, w: -4.0};

        assert_eq!(-a, BasicTuple{x: -1.0, y: 2.0, z: -3.0, w: 4.0});
    }

    #[test]
    fn scalar_multiply_tuple() {
        let a = BasicTuple{x: 1.0, y: -2.0, z: 3.0, w: -4.0};

        assert_eq!(a * 3.5, BasicTuple{x: 3.5, y: -7.0, z: 10.5, w: -14.0});
    }

    #[test]
    fn scalar_multiply_tuple_fraction() {
        let a = BasicTuple{x: 1.0, y: -2.0, z: 3.0, w: -4.0};

        assert_eq!(a * 0.5, BasicTuple{x: 0.5, y: -1.0, z: 1.5, w: -2.0});
    }

    #[test]
    fn scalar_divide_tuple() {
        let a = BasicTuple{x: 1.0, y: -2.0, z: 3.0, w: -4.0};

        assert_eq!(a / 2.0, BasicTuple{x: 0.5, y: -1.0, z: 1.5, w: -2.0});
    }
}
