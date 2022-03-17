pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut counts = HashMap::from([(0_u32, 1_i32)]);
        let mut result = 0;
        let mut sum = 0;

        assert_ne!(k, 0);

        for num in nums {
            sum += num.rem_euclid(k) as u32;
            sum = sum.checked_sub(k as _).unwrap_or(sum);

            if let Some(&count) = counts.get(&sum) {
                result += count;
            }

            counts.entry(sum).and_modify(|count| *count += 1).or_insert(1);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        Self::subarrays_div_by_k(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
