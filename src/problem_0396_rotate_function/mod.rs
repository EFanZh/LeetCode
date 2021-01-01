pub mod iterative;

pub trait Solution {
    fn max_rotate_function(a: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[4, 3, 2, 6] as &[_], 26), (&[], 0)];

        for (a, expected) in test_cases.iter().copied() {
            assert_eq!(S::max_rotate_function(a.to_vec()), expected);
        }
    }
}
