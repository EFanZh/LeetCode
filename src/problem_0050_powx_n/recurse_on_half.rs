pub struct Solution;

use std::cmp::Ordering;
use std::ops::{Div, Mul};

impl Solution {
    fn my_pow_generic<F: FnMut(f64, f64) -> f64>(x: f64, n: i32, combine: &mut F) -> f64 {
        if n == 0 {
            1.0
        } else {
            let pow_half = Self::my_pow_generic(x, n / 2, combine);
            let pow_half_squared = pow_half * pow_half;

            if n % 2 == 0 {
                pow_half_squared
            } else {
                combine(pow_half_squared, x)
            }
        }
    }

    pub fn my_pow(x: f64, n: i32) -> f64 {
        match n.cmp(&0) {
            Ordering::Less => Self::my_pow_generic(x, n, &mut Div::div),
            Ordering::Equal => 1.0,
            Ordering::Greater => Self::my_pow_generic(x, n, &mut Mul::mul),
        }
    }
}

impl super::Solution for Solution {
    fn my_pow(x: f64, n: i32) -> f64 {
        Self::my_pow(x, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
