use core::ops::Mul;
use std::ops::{Add, Div, Sub};
#[derive(Debug)]

pub struct Frac(i64, i64);

impl Frac {
    pub const fn new() -> Self {
        Frac(1, 1)
    }

    pub fn from<I64>(a: i64) -> Self {
        Frac(a, 1)
    }

    fn simplify(&self) -> Self {
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

fn main() {}

#[cfg(test)]
mod tests {
    use crate::Frac;
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
}
