pub struct Solution;

use std::cmp::Reverse;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut indices = (0..score.len()).collect::<Vec<_>>();

        indices.sort_unstable_by_key(|&i| Reverse(score[i]));

        let mut result = vec![String::new(); score.len()];

        for (rank, index) in (1..).zip(indices) {
            match rank {
                1 => result[index].push_str("Gold Medal"),
                2 => result[index].push_str("Silver Medal"),
                3 => result[index].push_str("Bronze Medal"),
                _ => result[index] = rank.to_string(),
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        Self::find_relative_ranks(score)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
