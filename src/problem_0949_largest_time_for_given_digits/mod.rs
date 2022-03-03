pub mod brute_force;

pub trait Solution {
    fn largest_time_from_digits(arr: Vec<i32>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ([1, 2, 3, 4], "23:41"),
            ([5, 5, 5, 5], ""),
            ([0, 0, 0, 0], "00:00"),
            ([1, 9, 6, 0], "19:06"),
            ([3, 2, 7, 0], "23:07"),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::largest_time_from_digits(arr.to_vec()), expected);
        }
    }
}
