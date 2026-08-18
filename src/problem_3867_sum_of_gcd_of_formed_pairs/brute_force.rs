pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZero;

impl Solution {
    fn gcd(mut x: u32, mut y: NonZero<u32>) -> NonZero<u32> {
        loop {
            if let Some(z) = NonZero::new(x % y) {
                (x, y) = (y.get(), z);
            } else {
                return y;
            }
        }
    }

    pub fn gcd_sum(nums: Vec<i32>) -> i64 {
        let mut max = 0;

        let mut prefix_grid = nums
            .into_iter()
            .map(|num| {
                let num = NonZero::new(num.cast_unsigned()).unwrap();

                max = max.max(num.get());

                Self::gcd(max, num)
            })
            .collect::<Vec<_>>();

        prefix_grid.sort_unstable();

        let n = prefix_grid.len();
        let (left, right) = prefix_grid.split_at(n / 2);

        left.iter()
            .zip(right.iter().rev())
            .fold(0, |result, (&x, &y)| result + i64::from(Self::gcd(x.get(), y).get()))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn gcd_sum(nums: Vec<i32>) -> i64 {
        Self::gcd_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
