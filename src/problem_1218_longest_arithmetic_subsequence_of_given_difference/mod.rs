pub mod dynamic_programming;

pub trait Solution {
    fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4] as &[_], 1), 4),
            ((&[1, 3, 5, 7], 1), 1),
            ((&[1, 5, 7, 8, 5, 3, 4, 2, 1], -2), 4),
        ];

        for ((arr, difference), expected) in test_cases {
            assert_eq!(S::longest_subsequence(arr.to_vec(), difference), expected);
        }
    }
}
