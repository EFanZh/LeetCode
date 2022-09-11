pub mod monotonic_stack;

pub trait Solution {
    fn mct_from_leaf_values(arr: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[6, 2, 4] as &[_], 32), (&[4, 11], 44), (&[15, 13, 5, 3, 15], 500)];

        for (arr, expected) in test_cases {
            assert_eq!(S::mct_from_leaf_values(arr.to_vec()), expected);
        }
    }
}
