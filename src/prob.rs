use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

use num::traits::{One, Zero};

pub trait ProbNum:
    PartialEq
    + Clone
    + One
    + Zero
    + Div
    + AddAssign
    + MulAssign
    + DivAssign
    + for<'r> Add<&'r Self, Output = Self>
    + for<'r> Mul<&'r Self, Output = Self>
    + for<'r> Div<&'r Self, Output = Self>
    + for<'r> AddAssign<&'r Self>
    + for<'r> MulAssign<&'r Self>
    + for<'r> DivAssign<&'r Self>
{
}

impl<T> ProbNum for T where
    T: PartialEq
        + Clone
        + One
        + Zero
        + Div
        + AddAssign
        + MulAssign
        + DivAssign
        + for<'r> Add<&'r Self, Output = Self>
        + for<'r> Mul<&'r Self, Output = Self>
        + for<'r> Div<&'r Self, Output = Self>
        + for<'r> AddAssign<&'r Self>
        + for<'r> MulAssign<&'r Self>
        + for<'r> DivAssign<&'r Self>
{
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProbDist<T, P>
where
    T: Hash + Eq,
    P: ProbNum,
{
    dist: HashMap<T, P>,
}

impl<T, P> ProbDist<T, P>
where
    T: Hash + Eq,
    P: ProbNum,
{
    pub fn new() -> Self {
        ProbDist {
            dist: HashMap::new(),
        }
    }

    pub fn append(&mut self, x: T, p: P) {
        self.dist.entry(x).and_modify(|q| *q += &p).or_insert(p);
    }

    pub fn sum(&self) -> P {
        let mut s = P::zero();
        for p in self.dist.values() {
            s += p;
        }
        s
    }

    pub fn normalize(&mut self) {
        let s = self.sum();
        if s == P::one() {
            return;
        }
        for p in self.dist.values_mut() {
            *p /= &s;
        }
    }

    pub fn normalized(mut self) -> Self {
        self.normalize();
        self
    }
}

impl<T, P> Add<ProbDist<T, P>> for ProbDist<T, P>
where
    T: Hash + Eq,
    P: ProbNum,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        if self.dist.len() > other.dist.len() {
            let mut ret = self;
            for (x, p) in other.dist {
                ret.dist.entry(x).and_modify(|q| *q += &p).or_insert(p);
            }
            ret
        } else {
            let mut ret = other;
            for (x, p) in self.dist {
                ret.dist.entry(x).and_modify(|q| *q += &p).or_insert(p);
            }
            ret
        }
    }
}

impl<T, P> ProbDist<T, P>
where
    T: Hash + Eq + Clone + Into<P>,
    P: ProbNum,
{
    pub fn mean(&self) -> P {
        let mut s = P::zero();
        for (x, p) in self.dist.iter() {
            s += x.clone().into() * p;
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ratio::BigURatio;

    #[test]
    fn normalize() {
        let mut dist = ProbDist::new();
        dist.append(1, 1.0);
        dist.append(2, 2.0);
        dist.append(3, 1.0);
        let mut dist2 = ProbDist::new();
        dist2.append(1, 0.25);
        dist2.append(2, 0.5);
        dist2.append(3, 0.25);
        assert_eq!(dist.normalized(), dist2);
    }

    #[test]
    fn mean() {
        let mut dist = ProbDist::new();
        dist.append(BigURatio::new(1, 1), BigURatio::new(1, 4));
        dist.append(BigURatio::new(2, 1), BigURatio::new(1, 4));
        dist.append(BigURatio::new(3, 1), BigURatio::new(1, 2));
        assert_eq!(dist.mean(), BigURatio::new(9, 4));
    }
}
