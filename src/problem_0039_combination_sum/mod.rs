pub mod backtracking;

pub trait Solution {
    fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run_tests<S: Solution>() {
        let test_cases = [
            ((&[2, 3, 6, 7] as &[_], 7), &[&[2, 2, 3] as &[_], &[7]] as &[&[_]]),
            ((&[2, 3, 5], 8), &[&[2, 2, 2, 2], &[2, 3, 3], &[3, 5]]),
        ];

        for ((candidates, target), expected) in test_cases.iter().copied() {
            assert_eq!(S::combination_sum(candidates.to_vec(), target), expected);
        }
    }
}
