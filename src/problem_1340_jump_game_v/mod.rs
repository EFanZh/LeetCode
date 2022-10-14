pub mod dfs;

pub trait Solution {
    fn max_jumps(arr: Vec<i32>, d: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[6, 4, 14, 6, 8, 13, 9, 7, 10, 6, 12] as &[_], 2), 4),
            ((&[3, 3, 3, 3, 3], 3), 1),
            ((&[7, 6, 5, 4, 3, 2, 1], 1), 7),
        ];

        for ((arr, d), expected) in test_cases {
            assert_eq!(S::max_jumps(arr.to_vec(), d), expected);
        }
    }
}
