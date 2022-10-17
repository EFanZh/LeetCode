pub mod mathematical;

pub trait Solution {
    fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2] as &[_], 3), 9),
            ((&[1, -2, 1], 5), 2),
            ((&[-1, -2], 7), 0),
            ((&[1, -1], 1), 1),
        ];

        for ((arr, k), expected) in test_cases {
            assert_eq!(S::k_concatenation_max_sum(arr.to_vec(), k), expected);
        }
    }
}
