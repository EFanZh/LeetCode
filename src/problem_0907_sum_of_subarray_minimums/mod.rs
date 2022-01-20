pub mod monotonic_stack;

pub trait Solution {
    fn sum_subarray_mins(arr: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 1, 2, 4] as &[_], 17),
            (&[11, 81, 94, 43, 3], 444),
            (&[71, 55, 82, 55], 593),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::sum_subarray_mins(arr.to_vec()), expected);
        }
    }
}
