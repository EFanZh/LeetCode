pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as u32 as usize;
        let mut indices = (0..nums.len() as u16).collect::<Box<_>>();

        let split = indices
            .select_nth_unstable_by_key(k - 1, |&i| Reverse(nums[usize::from(i)]))
            .0
            .len()
            + 1;

        let selected = &mut indices[..split];

        selected.sort_unstable();

        selected.iter().map(|&i| nums[usize::from(i)]).collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        Self::max_subsequence(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
