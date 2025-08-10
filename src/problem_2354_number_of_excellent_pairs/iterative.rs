pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn count_excellent_pairs(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as u32 as usize;
        let mut counts = [0_u32; 33];

        for &num in &HashSet::<_>::from_iter(nums) {
            counts[num.count_ones() as usize] += 1;
        }

        let mut result = 0;

        for lhs_count in 0..31 {
            for rhs_count in k.saturating_sub(lhs_count)..31 {
                result += u64::from(counts[lhs_count]) * u64::from(counts[rhs_count]);
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_excellent_pairs(nums: Vec<i32>, k: i32) -> i64 {
        Self::count_excellent_pairs(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
