pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;
use std::num::NonZeroU8;

impl Solution {
    pub fn count_distinct(nums: Vec<i32>, k: i32, p: i32) -> i32 {
        let k = k as u8;
        let p = NonZeroU8::new(p as _).unwrap();
        let nums = nums.into_iter().map(|num| num as u8).collect::<Vec<_>>();
        let mut start = 0;
        let mut count = 0;
        let mut sub_arrays = HashSet::new();

        for (i, &num) in (1..).zip(&nums) {
            count += u8::from(num % p == 0);

            while count > k {
                count -= u8::from(nums[start] % p == 0);
                start += 1;
            }

            for start in start..i {
                sub_arrays.insert(&nums[start..i]);
            }
        }

        sub_arrays.len() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_distinct(nums: Vec<i32>, k: i32, p: i32) -> i32 {
        Self::count_distinct(nums, k, p)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
