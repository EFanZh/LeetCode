pub mod iterative;

pub trait Solution {
    fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[1, 3, 1, 7] as &[_], &[1, 3, 2, 4] as &[_], 1),
                &[0, -1, 2, -1] as &[_],
            ),
            ((&[1, 2, 3], &[10], 5), &[-1]),
        ];

        for ((nums, queries, x), expected) in test_cases {
            assert_eq!(S::occurrences_of_element(nums.to_vec(), queries.to_vec(), x), expected);
        }
    }
}
