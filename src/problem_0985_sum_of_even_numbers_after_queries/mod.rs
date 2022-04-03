pub mod iterative;

pub trait Solution {
    fn sum_even_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[1, 2, 3, 4] as &[_], &[[1, 0], [-3, 1], [-4, 0], [2, 3]] as &[_]),
                &[8, 6, 2, 4] as &[_],
            ),
            ((&[1], &[[4, 0]]), &[0]),
        ];

        for ((num, queries), expected) in test_cases {
            assert_eq!(
                S::sum_even_after_queries(num.to_vec(), queries.iter().copied().map(Vec::from).collect()),
                expected
            );
        }
    }
}
