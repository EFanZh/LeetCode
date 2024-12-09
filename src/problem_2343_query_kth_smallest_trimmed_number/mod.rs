pub mod radix_sort;

pub trait Solution {
    fn smallest_trimmed_numbers(nums: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &["102", "473", "251", "814"] as &[_],
                    &[[1, 1], [2, 3], [4, 2], [1, 2]] as &[_],
                ),
                &[2, 2, 1, 0] as &[_],
            ),
            ((&["24", "37", "96", "04"], &[[2, 1], [2, 2]]), &[3, 0]),
        ];

        for ((nums, queries), expected) in test_cases {
            assert_eq!(
                S::smallest_trimmed_numbers(
                    nums.iter().copied().map(str::to_string).collect(),
                    queries.iter().map(Vec::from).collect(),
                ),
                expected,
            );
        }
    }
}
