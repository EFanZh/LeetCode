pub mod backtracking;
pub mod backtracking_2;
pub mod backtracking_3;

pub trait Solution {
    fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn sorted(mut value: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        value.iter_mut().for_each(|item| item.sort_unstable());

        value.sort_unstable();

        value
    }

    pub fn run_tests<S: Solution>() {
        let test_cases = [
            (
                (&[10, 1, 2, 7, 6, 1, 5] as &[_], 8),
                &[&[1, 1, 6] as &[_], &[1, 2, 5], &[1, 7], &[2, 6]] as &[&[_]],
            ),
            ((&[2, 5, 2, 1, 2], 5), &[&[1, 2, 2], &[5]]),
        ];

        for ((candidates, target), expected) in test_cases.iter().copied() {
            assert_eq!(sorted(S::combination_sum2(candidates.to_vec(), target)), expected);
        }
    }
}
