pub mod sort_indices;

pub trait Solution {
    fn find_relative_ranks(score: Vec<i32>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[5, 4, 3, 2, 1] as &[_],
                &["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"] as &[_],
            ),
            (
                &[10, 3, 8, 9, 4],
                &["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"],
            ),
        ];

        for (score, expected) in test_cases {
            assert_eq!(S::find_relative_ranks(score.to_vec()), expected);
        }
    }
}
