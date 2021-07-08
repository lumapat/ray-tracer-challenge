use std::ops;

#[derive(Debug, PartialEq)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

trait Vector {
    fn magnitude(&self) -> f64;
    fn normalize(&self) -> Self;
    // fn dot(&self, &Self) -> f64;
    // fn cross(&self, &Self) -> Self;
}

pub fn point(
    x: f64,
    y: f64,
    z: f64,
) -> Tuple {
    Tuple{x, y, z, w: 1.0}
}

pub fn vector(
    x: f64,
    y: f64,
    z: f64,
) -> Tuple {
    Tuple{x, y, z, w: 0.0}
}

impl Vector for Tuple {
    // TODO: Make Vector a type to restrict this behavior
    fn magnitude(&self) -> f64 {
        if self.is_vector() {
            (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
        } else {
            0.0
        }
    }

    fn normalize(&self) -> Self {
        let mag = self.magnitude();

        Tuple {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }
}


impl ops::Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Self::Output {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl ops::Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Self::Output {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_is_point() {
        let t = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };

        assert_eq!(t.is_point(), true);
        assert_eq!(t.is_vector(), false);
    }

    #[test]
    fn tuple_is_vector() {
        let t = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };

        assert_eq!(t.is_point(), false);
        assert_eq!(t.is_vector(), true);
    }

    #[test]
    fn create_point() {
        let p = point(4.0, -4.0, 3.0);

        assert_eq!(p, Tuple {
            x: p.x,
            y: p.y,
            z: p.z,
            w: 1.0,
        });
        assert_eq!(p.is_point(), true);
    }

    #[test]
    fn create_vector() {
        let p = vector(4.0, -4.0, 3.0);

        assert_eq!(p, Tuple {
            x: p.x,
            y: p.y,
            z: p.z,
            w: 0.0,
        });
        assert_eq!(p.is_vector(), true);
    }
}

#[cfg(test)]
mod operation_tests {
    use super::*;

    #[test]
    fn add_tuples() {
        let a1 = Tuple{x: 3.0, y: -2.0, z: 5.0, w: 1.0};
        let a2 = Tuple{x: -2.0, y: 3.0, z: 1.0, w: 0.0};

        assert_eq!(a1 + a2, Tuple{x: 1.0, y: 1.0, z: 6.0, w: 1.0});
    }

    #[test]
    fn subtract_two_points() {
        let p1 = Tuple{x: 3.0, y: 2.0, z: 1.0, w: 1.0};
        let p2 = Tuple{x: 5.0, y: 6.0, z: 7.0, w: 1.0};

        assert_eq!(p1 - p2, Tuple{x: -2.0, y: -4.0, z: -6.0, w: 0.0});
    }

    #[test]
    fn subtract_vector_from_point() {
        let p1 = Tuple{x: 3.0, y: 2.0, z: 1.0, w: 1.0};
        let p2 = Tuple{x: 5.0, y: 6.0, z: 7.0, w: 0.0};

        assert_eq!(p1 - p2, Tuple{x: -2.0, y: -4.0, z: -6.0, w: 1.0});
    }

    #[test]
    fn subtract_two_vectors() {
        let p1 = Tuple{x: 3.0, y: 2.0, z: 1.0, w: 0.0};
        let p2 = Tuple{x: 5.0, y: 6.0, z: 7.0, w: 0.0};

        assert_eq!(p1 - p2, Tuple{x: -2.0, y: -4.0, z: -6.0, w: 0.0});
    }

    #[test]
    fn subtract_zero_vector() {
        let zero = Tuple{x: 0.0, y: 0.0, z: 0.0, w: 0.0};
        let v = Tuple{x: 1.0, y: -2.0, z: 3.0, w: 0.0};

        assert_eq!(zero - v, Tuple{x: -1.0, y: 2.0, z: -3.0, w: 0.0});
    }

    #[test]
    fn negate_tuple() {
        let a = Tuple{x: 1.0, y: -2.0, z: 3.0, w: -4.0};

        assert_eq!(-a, Tuple{x: -1.0, y: 2.0, z: -3.0, w: 4.0});
    }

    #[test]
    fn scalar_multiply_tuple() {
        let a = Tuple{x: 1.0, y: -2.0, z: 3.0, w: -4.0};

        assert_eq!(a * 3.5, Tuple{x: 3.5, y: -7.0, z: 10.5, w: -14.0});
    }

    #[test]
    fn scalar_multiply_tuple_fraction() {
        let a = Tuple{x: 1.0, y: -2.0, z: 3.0, w: -4.0};

        assert_eq!(a * 0.5, Tuple{x: 0.5, y: -1.0, z: 1.5, w: -2.0});
    }

    #[test]
    fn scalar_divide_tuple() {
        let a = Tuple{x: 1.0, y: -2.0, z: 3.0, w: -4.0};

        assert_eq!(a / 2.0, Tuple{x: 0.5, y: -1.0, z: 1.5, w: -2.0});
    }
}

#[cfg(test)]
mod func_tests {
    use super::*;

    // TODO: Parametrize wit something pls
    #[test]
    fn magnitude_1() {
        assert_eq!(vector(1.0, 0.0, 0.0).magnitude(), 1.0);
    }

    #[test]
    fn magnitude_2() {
        assert_eq!(vector(0.0, 1.0, 0.0).magnitude(), 1.0);
    }

    #[test]
    fn magnitude_3() {
        assert_eq!(vector(0.0, 0.0, 1.0).magnitude(), 1.0);
    }

    #[test]
    fn magnitude_4() {
        assert_eq!(vector(1.0, 2.0, 3.0).magnitude(), (14.0 as f64).sqrt());
    }

    #[test]
    fn magnitude_5() {
        assert_eq!(vector(-1.0, -2.0, -3.0).magnitude(), (14.0 as f64).sqrt());
    }

    #[test]
    fn normalize_1() {
        assert_eq!(vector(4.0, 0.0, 0.0).normalize(), vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normalize_2() {
        let actual = vector(1.0, 2.0, 3.0).normalize();

        assert_relative_eq!(actual.x, 0.26726, max_relative = 0.00001);
        assert_relative_eq!(actual.y, 0.53452, max_relative = 0.00001);
        assert_relative_eq!(actual.z, 0.80178, max_relative = 0.00001);
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        assert_eq!(vector(1.0, 2.0, 3.0).normalize().magnitude(), 1.0);
    }
}