pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::num::NonZeroU32;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let mut nums = nums;
        let p = NonZeroU32::new(p as _).unwrap();
        let mut extra = 0_u32;

        for num in &mut nums {
            extra = (extra + *num as u32) % p;
            *num = extra as _;
        }

        if extra == 0 {
            0
        } else {
            let n = nums.len() as u32;
            let mut result = u32::MAX;
            let mut cache = HashMap::from([(0, 0)]);

            for (j, sum) in (1..).zip(nums) {
                let sum = sum as u32;
                let mut sum_2 = sum;

                if sum_2 < extra {
                    sum_2 += p.get();
                }

                if let Some(&i) = cache.get(&(sum_2 - extra)) {
                    result = result.min(j - i);
                }

                cache.insert(sum, j);
            }

            if result < n { result as _ } else { -1 }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        Self::min_subarray(nums, p)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
