pub mod mathematical;

pub trait Solution {
    fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 4, 2, 5, 3] as &[_], 58), (&[1, 2], 3), (&[10, 11, 12], 66)];

        for (arr, expected) in test_cases {
            assert_eq!(S::sum_odd_length_subarrays(arr.to_vec()), expected);
        }
    }
}
