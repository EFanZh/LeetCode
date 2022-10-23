pub mod cheating;

pub trait Solution {
    fn sort_by_bits(arr: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[0, 1, 2, 3, 4, 5, 6, 7, 8] as &[_],
                &[0, 1, 2, 4, 8, 3, 5, 6, 7] as &[_],
            ),
            (
                &[1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1],
                &[1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024],
            ),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::sort_by_bits(arr.to_vec()), expected);
        }
    }
}
