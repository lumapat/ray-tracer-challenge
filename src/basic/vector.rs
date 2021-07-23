use super::tuple::{
    BasicTuple,
    Tuple,
};

#[derive(Debug, PartialEq)]
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

    // fn dot(&self, &Self) -> f64;
    // fn cross(&self, &Self) -> Self;
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
}