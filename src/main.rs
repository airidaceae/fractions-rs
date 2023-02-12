use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug)]
pub struct Frac(i64, i64);

impl Frac {
    pub const fn new() -> Self {
        Frac(1, 1)
    }

    //TODO generic from method to allow for building from multiple types
    //may potentiall need a trait?
    pub fn from<I64>(a: i64) -> Self {
        Frac(a, 1)
    }
    fn simplify(&self) -> Self {
        todo!();
    }
}

impl PartialOrd for Frac {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!();
    }
}

impl PartialEq for Frac {
    fn eq(&self, other: &Self) -> bool {
        let result = self.0 == other.0;
        return if result { self.1 == other.1 } else { result };
    }
}

impl Mul for Frac {
    type Output = Frac;
    fn mul(self, rhs: Self) -> Self::Output {
        Frac(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Add for Frac {
    type Output = Frac;
    fn add(self, rhs: Self) -> Self::Output {
        todo!();
    }
}

impl Sub for Frac {
    type Output = Frac;
    fn sub(self, rhs: Self) -> Self::Output {
        todo!();
    }
}

impl Div for Frac {
    type Output = Frac;
    fn div(self, rhs: Self) -> Self::Output {
        Frac(self.0 * rhs.1, self.1 * rhs.0)
    }
}
impl fmt::Display for Frac {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}

fn main() {}

#[cfg(test)]
mod tests {

    use crate::Frac;
    #[test]
    fn display() {
        let a = Frac(1, 2);
        let b = format!("{}", a);
        let c = Frac(2231, 4124);
        let d = format!("{}", c);
        assert_eq!(b, "1/2");
        assert_eq!(d, "2231/4124");
    }

    #[test]
    fn from() {
        let a = Frac::from::<i64>(5);
        assert_eq!(a, Frac(5, 1));
        let g = 2;
        let t = 2.0;
        //TODO I would like to implement some kind of value checking between types
        //assert_eq!(a, 5)
    }

    #[test]
    fn equality() {
        let a: Frac = Frac(1, 2);
        let b: Frac = Frac(1, 2);
        assert_eq!(a, b);
        assert_eq!(a, Frac(1, 2))
    }

    #[test]
    fn multiplication() {
        let a: Frac = Frac(1, 2);
        let b: Frac = Frac(1, 2);
        let c = a * b;
        assert_eq!(c, Frac(1, 4));
    }

    #[test]
    fn addition() {
        let a = Frac(2, 5);
        let b = Frac(1, 5);
        let c = a + b;
        assert_eq!(c, Frac(3, 5));
    }

    #[test]
    fn subtraction() {
        let a = Frac(3, 2);
        let b = Frac(1, 4);
        let c = a - b;
        assert_eq!(c, Frac(5, 4));
    }

    #[test]
    fn division() {
        let a = Frac(3, 2);
        let b = Frac(1, 2);
        let c = a / b;
        assert_eq!(c, Frac(6, 2))
    }

    #[test]
    fn simplification() {
        let a = Frac(4, 2);
        let b = Frac(70, 24);
        let c = Frac(9999, 9999);
        assert_eq!(a.simplify(), Frac(2, 1));
        assert_eq!(b.simplify(), Frac(35, 12));
        assert_eq!(c.simplify(), Frac(1, 1))
    }
    fn comparison() {
        let a = Frac(7, 10);
        let b = Frac(3, 10);
        let p = a > b;
        let q = a < b;
        assert_eq!(p, true);
        assert_eq!(q, false);
        let p = a >= b;
        let q = a <= b;
        assert_eq!(p, true);
        assert_eq!(q, false);
    }
}
