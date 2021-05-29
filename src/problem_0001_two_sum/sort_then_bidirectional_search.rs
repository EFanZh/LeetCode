pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums = (0..).zip(nums).collect::<Box<_>>();

        nums.sort_unstable_by_key(|&(_, value)| value);

        let mut slice = nums.as_ref();

        // TODO: Update to use `slice_patterns`: https://doc.rust-lang.org/unstable-book/language-features/slice-patterns.html.

        while slice.len() > 2 {
            let &(first_index, first_value) = slice.first().unwrap();
            let &(last_index, last_value) = slice.last().unwrap();

            match (first_value + last_value).cmp(&target) {
                Ordering::Less => slice = &slice[1..],
                Ordering::Equal => return vec![first_index, last_index],
                Ordering::Greater => slice = &slice[..slice.len() - 1],
            }
        }

        slice.iter().map(|&(i, _)| i).collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
