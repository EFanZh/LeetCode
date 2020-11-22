pub struct Solution;

use std::collections::HashMap;

impl Solution {
    fn prefix_sum(tree: &[i32], mut i: usize) -> i32 {
        let mut result = 0;

        while i != 0 {
            result += tree[i - 1];

            i &= i - 1;
        }

        result
    }

    fn update(tree: &mut [i32], mut i: usize) {
        loop {
            tree[i] += 1;

            i += !i & (i + 1);

            if i >= tree.len() {
                break;
            }
        }
    }

    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted_nums = nums.clone();

        sorted_nums.sort_unstable();

        let ranks = sorted_nums.into_iter().zip(0..).collect::<HashMap<_, _>>();
        let mut tree = vec![0; nums.len()];
        let mut result = vec![0; nums.len()];

        for (target, num) in result.iter_mut().zip(nums).rev() {
            let rank = ranks[&num];

            *target = Self::prefix_sum(&tree, rank);
            Self::update(&mut tree, rank);
        }

        result
    }
}

impl super::Solution for Solution {
    fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        Self::count_smaller(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
