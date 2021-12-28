use num::{Num, Zero, One};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Complex<T>
{
    pub real: T,
    pub imaginary: T
}

impl<T> Complex<T> {
    pub fn new(real: T, imaginary: T) -> Complex<T> {
        Complex { real, imaginary }
    }
}

impl<T: Clone + Num> Complex<T> {
    pub fn i() -> Complex<T> {
        Self::new(T::zero(), T::one())
    }

    pub fn absolute(&self) -> T {
        self.real.clone() * self.real.clone() + self.imaginary.clone() * self.imaginary.clone()
    }
}

impl<T: Clone + Num> Zero for Complex<T> {
    fn zero() -> Complex<T> {
        Self::new(T::zero(), T::zero())
    }

    fn set_zero(&mut self) {
        self.real = T::zero();
        self.imaginary = T::zero();
    }

    fn is_zero(&self) -> bool {
        self.real.is_zero() && self.imaginary.is_zero()
    }
}

impl<T: Clone + Num> One for Complex<T> {
    fn one() -> Complex<T> {
        Self::new(T::one(), T::zero())
    }
}

impl<T: Clone + Num> Add<Self> for Complex<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(
            self.real.clone() + rhs.real.clone(),
            self.imaginary.clone() + rhs.imaginary.clone()
        )
    }
}

impl<T: Clone + Num> Sub<Self> for Complex<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(
            self.real.clone() - rhs.real.clone(),
            self.imaginary.clone() - rhs.imaginary.clone()
        )
    }
}

impl<T: Clone + Num> Mul<Self> for Complex<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output::new(
            self.real.clone() * rhs.real.clone() - self.imaginary.clone() * rhs.imaginary.clone(),
            self.real.clone() * rhs.imaginary.clone() + self.imaginary.clone() * self.real.clone()
        )
    }
}

impl<T: Clone + Num> Div<Self> for Complex<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let absolute = rhs.absolute();
        Self::Output::new(
            (self.real.clone() * rhs.real.clone() + self.imaginary.clone() * rhs.imaginary.clone()) / absolute.clone(),
            (self.imaginary.clone() * rhs.real.clone() - self.real.clone() * rhs.imaginary.clone()) / absolute,
        )
    }
}

#[cfg(test)]
mod test {
    use super::Complex;
    use num::{Zero, One};

    #[test]
    fn test_new_real() {
        assert_eq!(Complex::new(3.0, 0.0), Complex { real: 3.0, imaginary: 0.0 })
    }

    #[test]
    fn test_zero() {
        assert_eq!(Complex::zero(), Complex { real: 0.0, imaginary: 0.0 })
    }

    #[test]
    fn test_one() {
        assert_eq!(Complex::one(), Complex { real: 1.0, imaginary: 0.0 })
    }

    #[test]
    fn test_i() {
        assert_eq!(Complex::i(), Complex { real: 0.0, imaginary: 1.0 })
    }

    #[test]
    fn test_absolute() {
        assert_eq!(Complex { real: 3.0, imaginary: 4.0}.absolute(), 25.0)
    }
}