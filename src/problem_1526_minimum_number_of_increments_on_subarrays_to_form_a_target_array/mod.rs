pub mod greedy;

pub trait Solution {
    fn min_number_operations(target: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3, 2, 1] as &[_], 3), (&[3, 1, 1, 2], 4), (&[3, 1, 5, 4, 2], 7)];

        for (target, expected) in test_cases {
            assert_eq!(S::min_number_operations(target.to_vec()), expected);
        }
    }
}
