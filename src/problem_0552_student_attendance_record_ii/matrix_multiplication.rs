pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::ops::{Add, Mul};

const MODULUS: u32 = 1_000_000_007;

#[derive(Clone, Copy)]
struct WrappingNum(u32);

impl Add for WrappingNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let value = self.0 + rhs.0;

        Self(if value < MODULUS { value } else { value - MODULUS })
    }
}

impl Mul for WrappingNum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let modulus = u64::from(MODULUS);
        let value = u64::from(self.0) * u64::from(rhs.0);

        Self(if value < modulus { value } else { value % modulus } as _)
    }
}

#[derive(Clone, Copy)]
struct Vec6<T>(T, T, T, T, T, T);

impl<T> Vec6<T> {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> Vec6<U> {
        Vec6(f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5))
    }

    fn zip<U>(self, rhs: Vec6<U>) -> Vec6<(T, U)> {
        Vec6(
            (self.0, rhs.0),
            (self.1, rhs.1),
            (self.2, rhs.2),
            (self.3, rhs.3),
            (self.4, rhs.4),
            (self.5, rhs.5),
        )
    }

    fn sum(self) -> T
    where
        T: Add<Output = T>,
    {
        self.0 + self.1 + self.2 + self.3 + self.4 + self.5
    }

    fn dot(self, rhs: Self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.zip(rhs).map(|(x, y)| x * y).sum()
    }
}

impl Vec6<Vec6<WrappingNum>> {
    fn transpose(self) -> Self {
        Self(
            Vec6(self.0 .0, self.1 .0, self.2 .0, self.3 .0, self.4 .0, self.5 .0),
            Vec6(self.0 .1, self.1 .1, self.2 .1, self.3 .1, self.4 .1, self.5 .1),
            Vec6(self.0 .2, self.1 .2, self.2 .2, self.3 .2, self.4 .2, self.5 .2),
            Vec6(self.0 .3, self.1 .3, self.2 .3, self.3 .3, self.4 .3, self.5 .3),
            Vec6(self.0 .4, self.1 .4, self.2 .4, self.3 .4, self.4 .4, self.5 .4),
            Vec6(self.0 .5, self.1 .5, self.2 .5, self.3 .5, self.4 .5, self.5 .5),
        )
    }

    fn mul_mat(self, rhs: Self) -> Self {
        let transposed_rhs = rhs.transpose();

        self.map(|lhs_row| transposed_rhs.map(|rhs_column| lhs_row.dot(rhs_column)))
    }

    fn mul_vec(self, rhs: Vec6<WrappingNum>) -> Vec6<WrappingNum> {
        self.map(|row| row.dot(rhs))
    }

    fn pow(self, n: u32) -> Self {
        let mut result = self;

        for bit in (0..(31 - n.leading_zeros())).rev() {
            result = result.mul_mat(result);

            if n & (1 << bit) != 0 {
                result = result.mul_mat(self);
            }
        }

        result
    }
}

impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let matrix = Vec6(
            Vec6(1, 1, 1, 0, 0, 0),
            Vec6(1, 0, 0, 0, 0, 0),
            Vec6(0, 1, 0, 0, 0, 0),
            Vec6(1, 1, 1, 1, 1, 1),
            Vec6(0, 0, 0, 1, 0, 0),
            Vec6(0, 0, 0, 0, 1, 0),
        )
        .map(|row| row.map(WrappingNum))
        .pow(n as _);

        matrix.mul_vec(Vec6(1, 0, 0, 0, 0, 0).map(WrappingNum)).sum().0 as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_record(n: i32) -> i32 {
        Self::check_record(n)
    }
}

#[cfg(test)]
mod tests {
    use super::{Vec6, WrappingNum};

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }

    #[test]
    fn test_clone_wrapping_num() {
        for i in 0..10 {
            assert_eq!(Clone::clone(&WrappingNum(i)).0, WrappingNum(i).0);
        }
    }

    #[test]
    fn test_clone_vec6() {
        assert!(matches!(Clone::clone(&Vec6(0, 1, 2, 3, 4, 5)), Vec6(0, 1, 2, 3, 4, 5)));
    }
}
