pub mod brute_force;

pub trait Solution {
    fn find_lucky(arr: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 2, 3, 4] as &[_], 2),
            (&[1, 2, 2, 3, 3, 3], 3),
            (&[2, 2, 2, 3, 3], -1),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::find_lucky(arr.to_vec()), expected);
        }
    }
}
