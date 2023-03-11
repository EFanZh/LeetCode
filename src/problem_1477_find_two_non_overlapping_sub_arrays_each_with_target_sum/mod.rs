pub mod sliding_window;

pub trait Solution {
    fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 2, 2, 4, 3] as &[_], 3), 2),
            ((&[7, 3, 4, 7], 7), 2),
            ((&[4, 3, 2, 6, 2, 3, 4], 6), -1),
        ];

        for ((arr, target), expected) in test_cases {
            assert_eq!(S::min_sum_of_lengths(arr.to_vec(), target), expected);
        }
    }
}
