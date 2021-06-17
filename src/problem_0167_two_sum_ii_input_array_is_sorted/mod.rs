pub mod bidirectional_search;

pub trait Solution {
    fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 7, 11, 15] as &[_], 9), [1, 2]),
            ((&[2, 3, 4], 6), [1, 3]),
            ((&[-1, 0], -1), [1, 2]),
            ((&[5, 25, 75], 100), [2, 3]),
        ];

        for ((numbers, target), expected) in test_cases {
            assert_eq!(S::two_sum(numbers.to_vec(), target), expected);
        }
    }
}
