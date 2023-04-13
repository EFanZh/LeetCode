pub mod sliding_window;

pub trait Solution {
    fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 10, 4, 2, 3, 5] as &[_], 3),
            (&[5, 4, 3, 2, 1], 4),
            (&[1, 2, 3], 0),
            (&[10, 13, 17, 21, 15, 15, 9, 17, 22, 22, 13], 7),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::find_length_of_shortest_subarray(arr.to_vec()), expected);
        }
    }
}
