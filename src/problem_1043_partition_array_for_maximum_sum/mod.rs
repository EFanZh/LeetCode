pub mod dynamic_programming;

pub trait Solution {
    fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 15, 7, 9, 2, 5, 10] as &[_], 3), 84),
            ((&[1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3], 4), 83),
            ((&[1], 1), 1),
        ];

        for ((arr, k), expected) in test_cases {
            assert_eq!(S::max_sum_after_partitioning(arr.to_vec(), k), expected);
        }
    }
}
