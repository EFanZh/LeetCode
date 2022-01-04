pub mod dynamic_programming;
pub mod dynamic_programming_2;

pub trait Solution {
    fn len_longest_fib_subseq(arr: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4, 5, 6, 7, 8] as &[_], 5),
            (&[1, 3, 7, 11, 12, 14, 18], 3),
            (&[2, 4, 7, 8, 9, 10, 14, 15, 18, 23, 32, 50], 5),
            (&[1, 3, 5], 0),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::len_longest_fib_subseq(arr.to_vec()), expected);
        }
    }
}
