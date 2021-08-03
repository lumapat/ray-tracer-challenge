use std::ops;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Clone, Debug, PartialEq, StructOpt)]
pub struct BasicTuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Debug)]
pub struct TupleParseError(String);

impl std::fmt::Display for TupleParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let TupleParseError(s) = self;
        write!(f, "{}", s)
    }
}

impl From<regex::Error> for TupleParseError {
    fn from(e: regex::Error) -> Self {
        match e {
            regex::Error::Syntax(s) => TupleParseError(s),
            regex::Error::CompiledTooBig(_) => TupleParseError("TODO (too big)".to_string()),
            _ => TupleParseError("TODO (unknown)".to_string()),
        }
    }
}

impl From<std::num::ParseFloatError> for TupleParseError {
    fn from(_e: std::num::ParseFloatError) -> Self {
        TupleParseError("TODO (parse error)".to_string())
    }
}

impl FromStr for BasicTuple {
    type Err = TupleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tuple_re = regex::Regex::new(r"\((?P<x>\d+\.?\d*),(?P<y>\d+\.?\d*),(?P<z>\d+\.?\d*)\)")?;
        let caps = tuple_re.captures(s).ok_or_else(|| TupleParseError("invalid tuple format (#.#, #.#, #.#)".to_string()))?;

        Ok(BasicTuple{
            x: caps.name("x").ok_or_else(|| TupleParseError("missing x value".to_string()))?.as_str().parse::<f64>()?,
            y: caps.name("y").ok_or_else(|| TupleParseError("missing y value".to_string()))?.as_str().parse::<f64>()?,
            z: caps.name("z").ok_or_else(|| TupleParseError("missing z value".to_string()))?.as_str().parse::<f64>()?,
            w: 0.0,
        })
    }
}

pub trait Tuple {
    fn is_point(&self) -> bool;
    fn is_vector(&self) -> bool;
    fn tuple(&self) -> &BasicTuple;
}

impl Tuple for BasicTuple {
    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    fn tuple(&self) -> &BasicTuple {
        return &self;
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
