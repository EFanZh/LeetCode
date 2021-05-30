pub mod backtracking;
pub mod backtracking_2;
pub mod backtracking_3;

pub trait Solution {
    fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[10, 1, 2, 7, 6, 1, 5] as &[_], 8),
                &[&[1, 1, 6] as &[_], &[1, 2, 5], &[1, 7], &[2, 6]] as &[&[_]],
            ),
            ((&[2, 5, 2, 1, 2], 5), &[&[1, 2, 2], &[5]]),
            ((&[1], 2), &[]),
        ];

        for ((candidates, target), expected) in test_cases.iter().copied() {
            assert_eq!(
                test_utilities::unstable_sorted(S::combination_sum2(candidates.to_vec(), target).into_iter().map(
                    |mut item| {
                        item.sort_unstable();

                        item
                    }
                )),
                expected
            );
        }
    }
}
