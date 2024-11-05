pub mod backtracking;

pub trait Solution {
    fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[8, 15, 10, 20, 8] as &[_], 2), 31),
            ((&[6, 1, 3, 2, 2, 4, 1, 2] as &[_], 3), 7),
        ];

        for ((cookies, k), expected) in test_cases {
            assert_eq!(S::distribute_cookies(cookies.to_vec(), k), expected);
        }
    }
}
