pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn kth_largest_number(nums: Vec<String>, k: i32) -> String {
        let mut nums = nums;

        mem::take(
            nums.select_nth_unstable_by(k as u32 as usize - 1, |x, y| {
                y.len().cmp(&x.len()).then_with(|| y.cmp(x))
            })
            .1,
        )
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn kth_largest_number(nums: Vec<String>, k: i32) -> String {
        Self::kth_largest_number(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
