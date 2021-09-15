pub mod greedy;

pub trait Solution {
    fn max_chunks_to_sorted(arr: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[4, 3, 2, 1, 0] as &[_], 1), (&[1, 0, 2, 3, 4], 4)];

        for (arr, expected) in test_cases {
            assert_eq!(S::max_chunks_to_sorted(arr.to_vec()), expected);
        }
    }
}
