pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::ops::Add;

#[derive(Clone, Copy)]
struct WrappingNum(i32);

impl Add for WrappingNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let modulus = 1_000_000_007;
        let value = self.0 + rhs.0;

        Self(if value < modulus { value } else { value - modulus })
    }
}

impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let mut a0l0 = WrappingNum(1);
        let mut a0l1 = WrappingNum(0);
        let mut a0l2 = WrappingNum(0);
        let mut a1l0 = WrappingNum(0);
        let mut a1l1 = WrappingNum(0);
        let mut a1l2 = WrappingNum(0);

        for _ in 0..n {
            let next_a0l0 = a0l0 + a0l1 + a0l2;
            let next_a0l1 = a0l0;
            let next_a0l2 = a0l1;
            let next_a1l0 = a0l0 + a0l1 + a0l2 + a1l0 + a1l1 + a1l2;
            let next_a1l1 = a1l0;
            let next_a1l2 = a1l1;

            a0l0 = next_a0l0;
            a0l1 = next_a0l1;
            a0l2 = next_a0l2;
            a1l0 = next_a1l0;
            a1l1 = next_a1l1;
            a1l2 = next_a1l2;
        }

        (a0l0 + a0l1 + a0l2 + a1l0 + a1l1 + a1l2).0
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
    use super::WrappingNum;

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }

    #[test]
    fn test_clone_wrapping_num() {
        for i in -10..10 {
            assert_eq!(Clone::clone(&WrappingNum(i)).0, WrappingNum(i).0);
        }
    }
}
