pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn next_permutation(nums: &mut [i32]) {
        if let Some((_, rest)) = nums.split_first() {
            if let Some((i, (current, _))) = nums
                .iter()
                .zip(rest)
                .enumerate()
                .rfind(|(_, (current, next))| current < next)
            {
                let j = nums[i + 2..]
                    .binary_search_by(|x| match current.cmp(x) {
                        Ordering::Less => Ordering::Less,
                        _ => Ordering::Greater,
                    })
                    .unwrap_err();

                nums.swap(i, i + 1 + j);
                nums[i + 1..].reverse();
            } else {
                nums.reverse();
            }
        }
    }
}

impl super::Solution for Solution {
    fn next_permutation(nums: &mut Vec<i32>) {
        Self::next_permutation(nums);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
