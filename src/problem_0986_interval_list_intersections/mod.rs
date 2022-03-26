pub mod iterative;

pub trait Solution {
    fn interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[[0, 2], [5, 10], [13, 23], [24, 25]] as &[_],
                    &[[1, 5], [8, 12], [15, 24], [25, 26]] as &[_],
                ),
                &[[1, 2], [5, 5], [8, 10], [15, 23], [24, 24], [25, 25]] as &[_],
            ),
            ((&[[1, 3], [5, 9]], &[]), &[]),
            (
                (&[[10, 12], [18, 19]], &[[1, 6], [8, 11], [13, 17], [19, 20]]),
                &[[10, 11], [19, 19]],
            ),
            ((&[[3, 10]], &[[5, 10]]), &[[5, 10]]),
            ((&[[5, 10]], &[[5, 10]]), &[[5, 10]]),
        ];

        for ((first_list, second_list), expected) in test_cases {
            assert_eq!(
                S::interval_intersection(
                    first_list.iter().copied().map(Vec::from).collect(),
                    second_list.iter().copied().map(Vec::from).collect(),
                ),
                expected
            );
        }
    }
}
