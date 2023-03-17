pub mod iterative;

pub trait Solution {
    fn num_of_subarrays(arr: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 3, 5] as &[_], 4), (&[2, 4, 6], 0), (&[1, 2, 3, 4, 5, 6, 7], 16)];

        for (arr, expected) in test_cases {
            assert_eq!(S::num_of_subarrays(arr.to_vec()), expected);
        }
    }
}
