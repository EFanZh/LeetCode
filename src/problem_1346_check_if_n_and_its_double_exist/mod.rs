pub mod hash_set;

pub trait Solution {
    fn check_if_exist(arr: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[10, 2, 5, 3] as &[_], true), (&[3, 1, 7, 11], false)];

        for (arr, expected) in test_cases {
            assert_eq!(S::check_if_exist(arr.to_vec()), expected);
        }
    }
}
