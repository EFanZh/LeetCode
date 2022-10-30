pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU32;

impl Solution {
    pub fn closest_divisors(num: i32) -> Vec<i32> {
        let num = num as u32;
        let num_1 = num + 1;
        let num_2 = num + 2;
        let mut lhs = f64::from(num_2).sqrt() as u32;

        loop {
            let non_zero_lhs = NonZeroU32::new(lhs).unwrap();

            let num = if num_1 % non_zero_lhs == 0 {
                num_1
            } else if num_2 % non_zero_lhs == 0 {
                num_2
            } else {
                lhs -= 1;

                continue;
            };

            return vec![non_zero_lhs.get() as _, (num / non_zero_lhs) as _];
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn closest_divisors(num: i32) -> Vec<i32> {
        Self::closest_divisors(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
