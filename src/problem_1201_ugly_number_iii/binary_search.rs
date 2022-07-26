pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU32;

impl Solution {
    fn gcd(mut x: NonZeroU32, mut y: NonZeroU32) -> NonZeroU32 {
        while let Some(z) = NonZeroU32::new(x.get() % y) {
            x = y;
            y = z;
        }

        y
    }

    fn lcm(x: NonZeroU32, y: NonZeroU32) -> NonZeroU32 {
        x.get()
            .checked_mul(y.get() / Self::gcd(x, y))
            .and_then(NonZeroU32::new)
            .unwrap_or(NonZeroU32::new(u32::MAX).unwrap())
    }

    pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
        let a = NonZeroU32::new(a as _).unwrap();
        let b = NonZeroU32::new(b as _).unwrap();
        let c = NonZeroU32::new(c as _).unwrap();
        let ab = Self::lcm(a, b);
        let ac = Self::lcm(a, c);
        let bc = Self::lcm(b, c);
        let abc = Self::lcm(ab, c);

        let mut left = 1;
        let mut right = 2_000_000_000;

        while left < right {
            let middle = (left + right) / 2;
            let count = middle / a + middle / b + middle / c - middle / ab - middle / ac - middle / bc + middle / abc;

            if count < n as _ {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        left as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
        Self::nth_ugly_number(n, a, b, c)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
