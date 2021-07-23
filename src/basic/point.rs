use super::tuple::{
    BasicTuple,
    Tuple,
};

#[derive(Debug, PartialEq)]
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