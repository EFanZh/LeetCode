pub mod dynamic_programming;

pub trait Solution {
    fn num_factored_binary_trees(arr: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 4] as &[_], 3), (&[2, 4, 5, 10], 7), (&[18, 3, 6, 2], 12)];

        for (arr, expected) in test_cases {
            assert_eq!(S::num_factored_binary_trees(arr.to_vec()), expected);
        }
    }
}
