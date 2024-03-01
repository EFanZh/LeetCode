pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn max_score_indices(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0];
        let mut max_score = 0;
        let mut score = 0;

        for (i, num) in (1..).zip(nums) {
            score += 1 - num * 2;

            match score.cmp(&max_score) {
                Ordering::Less => continue,
                Ordering::Equal => {}
                Ordering::Greater => {
                    max_score = score;

                    result.clear();
                }
            }

            result.push(i);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_score_indices(nums: Vec<i32>) -> Vec<i32> {
        Self::max_score_indices(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
