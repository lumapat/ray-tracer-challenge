use std::ops;
use super::tuple::{
    BasicTuple,
    Tuple,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Vector(BasicTuple);

impl Vector {
    // TODO: Make Vector a type to restrict this behavior
    fn magnitude(&self) -> f64 {
        let Vector(t) = self;

        (t.x * t.x + t.y * t.y + t.z * t.z).sqrt()
    }

    fn normalize(&self) -> Self {
        let mag = self.magnitude();
        let Vector(t) = self;

        Vector(BasicTuple{
            x: t.x / mag,
            y: t.y / mag,
            z: t.z / mag,
            w: t.w / mag,
        })
    }

    fn dot(&self, other: &Self) -> f64 {
        let Vector(t1) = self;
        let Vector(t2) = other;

        t1.x * t2.x + t1.y * t2.y + t1.z * t2.z + t1.w * t2.w
    }

    fn cross(&self, other: &Self) -> Self {
        let Vector(a) = self;
        let Vector(b) = other;

        vector(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }
}

impl Tuple for Vector {
    fn is_point(&self) -> bool {
        let Vector(t) = self;
        t.is_point()
    }

    fn is_vector(&self) -> bool {
        let Vector(t) = self;
        t.is_vector()
    }

    fn tuple(&self) -> &BasicTuple {
        let Vector(t) = self;
        return &t;
    }
}

impl ops::Add for Vector {
    type Output = Vector;

    fn add(self, other: Self) -> Self::Output {
        vector(
            self.0.x + other.0.x,
            self.0.y + other.0.y,
            self.0.z + other.0.z,
        )
    }
}

impl From<&BasicTuple> for Vector {
    fn from(s: &BasicTuple) -> Vector {
        vector(
            s.x,
            s.y,
            s.z,
        )
    }
}
pub fn vector(
    x: f64,
    y: f64,
    z: f64,
) -> Vector {
    Vector(BasicTuple{x, y, z, w: 0.0})
}

#[cfg(test)]
mod tuple_trait_tests {
    use super::*;

    #[test]
    fn vector_is_vector() {
        let v = vector(4.3, -4.2, 3.1);

        assert_eq!(v.is_point(), false);
        assert_eq!(v.is_vector(), true);
    }
}

#[cfg(test)]
mod vector_trait_tests {
    use super::*;
    use approx::assert_relative_eq;

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
        let Vector(actual) = vector(1.0, 2.0, 3.0).normalize();

        assert_relative_eq!(actual.x, 0.26726, max_relative = 0.00001);
        assert_relative_eq!(actual.y, 0.53452, max_relative = 0.00001);
        assert_relative_eq!(actual.z, 0.80178, max_relative = 0.00001);
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        assert_eq!(vector(1.0, 2.0, 3.0).normalize().magnitude(), 1.0);
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);

        assert_eq!(v1.dot(&v2), 20.0)
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);

        assert_eq!(v1.cross(&v2), vector(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(&v1), vector(1.0, -2.0, 1.0));
    }
}