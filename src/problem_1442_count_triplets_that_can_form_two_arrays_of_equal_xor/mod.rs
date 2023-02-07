pub mod iterative;

pub trait Solution {
    fn count_triplets(arr: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 3, 1, 6, 7] as &[_], 4), (&[1, 1, 1, 1, 1], 10)];

        for (arr, expected) in test_cases {
            assert_eq!(S::count_triplets(arr.to_vec()), expected);
        }
    }
}
