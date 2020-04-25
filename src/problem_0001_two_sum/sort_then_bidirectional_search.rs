pub struct Solution {}

use std::cmp::Ordering;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut sorted_nums = nums.into_iter().enumerate().collect::<Box<_>>();

        sorted_nums.sort_unstable_by_key(|(_, value)| *value);

        let mut slice = &sorted_nums[..];

        // TODO: Update to use `slice_patterns`: https://doc.rust-lang.org/unstable-book/language-features/slice-patterns.html.

        while slice.len() > 1 {
            let (first_index, first_value) = slice.first().unwrap();
            let (last_index, last_value) = slice.last().unwrap();

            match (first_value + last_value).cmp(&target) {
                Ordering::Less => slice = &slice[1..],
                Ordering::Equal => return vec![*first_index as _, *last_index as _],
                Ordering::Greater => slice = &slice[..slice.len() - 1],
            }
        }

        Vec::new()
    }
}

impl super::Solution for Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::two_sum(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
