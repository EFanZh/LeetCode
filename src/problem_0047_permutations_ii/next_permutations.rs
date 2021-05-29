pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn next_permutation(nums: &mut [i32]) -> bool {
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

                return true;
            }
        }

        false
    }

    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        nums.sort_unstable();

        loop {
            result.push(nums.clone());

            if !Self::next_permutation(&mut nums) {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::permute_unique(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
