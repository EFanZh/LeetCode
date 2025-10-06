pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::num::NonZeroU32;

impl Solution {
    pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        divisors
            .into_iter()
            .fold((0_u32, u32::MAX), |(max_score, max_score_divisor), divisor| {
                let divisor = NonZeroU32::new(divisor as _).unwrap();
                let score = nums.iter().filter(|&&num| num as u32 % divisor == 0).count() as u32;

                match score.cmp(&max_score) {
                    Ordering::Less => (max_score, max_score_divisor),
                    Ordering::Equal => (score, max_score_divisor.min(divisor.get())),
                    Ordering::Greater => (score, divisor.get()),
                }
            })
            .1 as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        Self::max_div_score(nums, divisors)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
