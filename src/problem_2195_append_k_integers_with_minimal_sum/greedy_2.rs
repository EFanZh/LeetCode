pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let nums = nums.into_iter().map(|x| x as u32).collect::<HashSet<_>>();
        let k = k as u32;
        let mut candidate = u64::from(k) * (u64::from(k) + 1) / 2;
        let mut last = k;

        for &num in &nums {
            if num <= k {
                loop {
                    last += 1;

                    if !nums.contains(&last) {
                        break;
                    }
                }

                candidate += u64::from(last - num);
            }
        }

        candidate as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64 {
        Self::minimal_k_sum(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
