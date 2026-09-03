pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn centered_subarrays(nums: Vec<i32>) -> i32 {
        let mut seen = HashSet::new();
        let mut iter = nums.iter().copied();
        let mut result = nums.len() as _;

        while let Some(mut sum) = iter.next() {
            seen.insert(sum);

            for num in iter.clone() {
                sum += num;
                seen.insert(num);
                result += i32::from(seen.contains(&sum));
            }

            seen.clear();
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn centered_subarrays(nums: Vec<i32>) -> i32 {
        Self::centered_subarrays(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
