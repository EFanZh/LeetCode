pub mod prefix_sums;

pub trait Solution {
    fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 4, 1, 2, 6] as &[_], &[[0, 4]] as &[_]), &[false] as &[_]),
            ((&[4, 3, 1, 6], &[[0, 2], [2, 3]]), &[false, true]),
        ];

        for ((nums, queries), expected) in test_cases {
            assert_eq!(
                S::is_array_special(nums.to_vec(), queries.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
