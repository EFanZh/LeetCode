pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    fn add_and_get(lhs: &mut u32, rhs: u32) -> u32 {
        *lhs = u32::wrapping_add(*lhs, rhs);

        *lhs
    }

    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let mut counts = HashMap::<_, u32>::new();
        let mut count_counts = vec![0_u32; nums.len() + 1];
        let mut count_count_count = 0;
        let mut result = 0;

        for (candidate, num) in (1..).zip(nums) {
            if count_count_count < 2 {
                result = candidate;
            }

            let count = *counts.entry(num).and_modify(|count| *count += 1).or_insert(1) as usize;

            if Self::add_and_get(&mut count_counts[count - 1], u32::MAX) == 0 {
                count_count_count -= 1;
            }

            if Self::add_and_get(&mut count_counts[count], 1) == 1 {
                count_count_count += 1;
            }

            if count_count_count == 2 && (count_counts[1] == 1 || count_counts.get(count + 1).copied() == Some(1)) {
                result = candidate;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_equal_freq(nums: Vec<i32>) -> i32 {
        Self::max_equal_freq(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
