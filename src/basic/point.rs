use super::tuple::{
    BasicTuple,
    Tuple,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Point(BasicTuple);

impl Tuple for Point {
    fn is_point(&self) -> bool {
        let Point(t) = self;
        t.is_point()
    }

    fn is_vector(&self) -> bool {
        let Point(t) = self;
        t.is_vector()
    }

    fn tuple(&self) -> &BasicTuple {
        let Point(t) = self;
        return &t;
    }
}

impl std::ops::Add<super::vector::Vector> for Point {
    type Output = Self;

    fn add(self, other: super::vector::Vector) -> Self::Output {
        let self_tuple = self.tuple();
        let other_tuple = other.tuple();
        point(
            self_tuple.x + other_tuple.x,
            self_tuple.y + other_tuple.y,
            self_tuple.z + other_tuple.z,
        )
    }
}

impl From<&BasicTuple> for Point {
    fn from(s: &BasicTuple) -> Point {
        point(
            s.x,
            s.y,
            s.z,
        )
    }
}

pub fn point(
    x: f64,
    y: f64,
    z: f64,
) -> Point {
    Point(BasicTuple{x, y, z, w: 1.0})
}

#[cfg(test)]
mod tuple_trait_tests {
    use super::*;

    #[test]
    fn point_is_point() {
        let p = point(4.3, -4.2, 3.1);

        assert_eq!(p.is_point(), true);
        assert_eq!(p.is_vector(), false);
    }
}