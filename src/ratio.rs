use core::convert::TryFrom;
use core::fmt::{Display, Formatter};
use core::ops::{Add, Div, Mul, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

use num::{
    bigint::BigUint,
    rational::Ratio,
    traits::{float::FloatCore, FromPrimitive, One, Zero},
};

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct BigURatio(Ratio<BigUint>);

impl Display for BigURatio {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        (self.0).fmt(f)
    }
}

impl FromPrimitive for BigURatio {
    #[inline]
    fn from_i64(n: i64) -> Option<Self> {
        BigUint::from_i64(n).map(|n| BigURatio(Ratio::new(n, BigUint::new(vec![1]))))
    }

    #[inline]
    fn from_u8(n: u8) -> Option<Self> {
        Some(BigURatio(Ratio::from_integer(BigUint::new(vec![n as u32]))))
    }

    #[inline]
    fn from_u16(n: u16) -> Option<Self> {
        Some(BigURatio(Ratio::from_integer(BigUint::new(vec![n as u32]))))
    }

    #[inline]
    fn from_u32(n: u32) -> Option<Self> {
        Some(BigURatio(Ratio::from_integer(BigUint::new(vec![n]))))
    }

    #[inline]
    fn from_u64(n: u64) -> Option<Self> {
        Some(BigURatio(Ratio::from_integer(BigUint::from(n))))
    }

    #[inline]
    fn from_u128(n: u128) -> Option<Self> {
        Some(BigURatio(Ratio::from_integer(BigUint::from(n))))
    }
}

macro_rules! impl_ops {
    ($($t:tt $m:ident,)*) => {$(
        impl $t for BigURatio {
            type Output = BigURatio;
            #[inline]
            fn $m(self, other: BigURatio) -> BigURatio {
                BigURatio((self.0).$m(other.0))
            }
        }
        impl $t<BigURatio> for &BigURatio {
            type Output = BigURatio;
            #[inline]
            fn $m(self, other: BigURatio) -> BigURatio {
                BigURatio((&self.0).$m(other.0))
            }
        }
        impl<'a> $t<&'a BigURatio> for BigURatio {
            type Output = BigURatio;
            #[inline]
            fn $m(self, other: &'a BigURatio) -> BigURatio {
                BigURatio((self.0).$m(&other.0))
            }
        }
        impl<'a> $t<&'a BigURatio> for &BigURatio {
            type Output = BigURatio;
            #[inline]
            fn $m(self, other: &'a BigURatio) -> BigURatio {
                BigURatio((&self.0).$m(&other.0))
            }
        }
    )*};
}

impl_ops!(
    Add add,
    Div div,
    Mul mul,
    Rem rem,
    Sub sub,
);

macro_rules! impl_assign_ops {
    ($($t:tt $m:ident,)*) => {$(
        impl $t for BigURatio {
            #[inline]
            fn $m(&mut self, other: BigURatio) {
                self.0 += other.0;
            }
        }
        impl<'a> $t<&'a BigURatio> for BigURatio {
            #[inline]
            fn $m(&mut self, other: &'a BigURatio) {
                self.0 += &other.0;
            }
        }
    )*};
}

impl_assign_ops!(
    AddAssign add_assign,
    DivAssign div_assign,
    MulAssign mul_assign,
    RemAssign rem_assign,
    SubAssign sub_assign,
);

macro_rules! impl_from {
    ($($t:tt $m:ident,)*) => {$(
        impl From<$t> for BigURatio {
            #[inline]
            fn from(n: $t) -> Self {
                BigURatio::$m(n).unwrap()
            }
        }
    )*};
}

macro_rules! impl_try_from {
    ($($t:tt $m:ident,)*) => {$(
        impl TryFrom<$t> for BigURatio {
            type Error = ();
            #[inline]
            fn try_from(n: $t) -> Result<Self, Self::Error> {
                BigURatio::$m(n).ok_or(())
            }
        }
    )*};
}

impl_from!(
    u8 from_u8,
    u16 from_u16,
    u32 from_u32,
    u64 from_u64,
    u128 from_u128,
    usize from_usize,
);
impl_try_from!(
    i8 from_i8,
    i16 from_i16,
    i32 from_i32,
    i64 from_i64,
    i128 from_i128,
    isize from_isize,
    f32 from_float,
    f64 from_float,
);

impl One for BigURatio {
    #[inline]
    fn one() -> Self {
        BigURatio(Ratio::one())
    }
}

impl Zero for BigURatio {
    #[inline]
    fn zero() -> Self {
        BigURatio(Ratio::zero())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        (self.0).is_zero()
    }
}

impl BigURatio {
    #[inline]
    pub fn new_from_u32(numer: u32, denom: u32) -> Self {
        Self::new(BigUint::new(vec![numer]), BigUint::new(vec![denom]))
    }

    #[inline]
    pub fn new(numer: BigUint, denom: BigUint) -> Self {
        BigURatio(Ratio::new(numer, denom))
    }

    #[inline]
    pub fn from_integer(numer: BigUint) -> Self {
        Self::new(numer, One::one())
    }

    pub fn from_float<T: FloatCore>(f: T) -> Option<Self> {
        if !f.is_finite() {
            return None;
        }
        let (mantissa, exponent, sign) = f.integer_decode();
        if sign == -1 {
            return None;
        }
        if exponent < 0 {
            let one: BigUint = One::one();
            let denom: BigUint = one << ((-exponent) as usize);
            let numer: BigUint = FromPrimitive::from_u64(mantissa).unwrap();
            Some(BigURatio(Ratio::new(numer, denom)))
        } else {
            let mut numer: BigUint = FromPrimitive::from_u64(mantissa).unwrap();
            numer <<= exponent as usize;
            Some(BigURatio(Ratio::from_integer(numer)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_float() {
        assert_eq!(
            BigURatio::from_float(0.25),
            Some(BigURatio::new_from_u32(1, 4))
        );
    }
}
