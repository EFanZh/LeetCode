pub struct Solution {}

use std::cmp::Ordering;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if let Some((i, (current, _))) = nums
            .iter()
            .zip(&nums[1..])
            .enumerate()
            .rfind(|(_, (current, next))| current < next)
        {
            let j = i + nums[i + 1..]
                .binary_search_by(|x| match current.cmp(x) {
                    Ordering::Less => Ordering::Less,
                    _ => Ordering::Greater,
                })
                .unwrap_err();

            nums.swap(i, j);
            nums[i + 1..].reverse();
        } else {
            nums.reverse();
        }
    }
}

impl super::Solution for Solution {
    fn next_permutation(nums: &mut Vec<i32>) {
        Self::next_permutation(nums)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run;
    use super::Solution;

    #[test]
    fn test_solution() {
        run::<Solution>();
    }
}
