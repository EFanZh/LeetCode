pub mod dynamic_programming;

pub trait Solution {
    fn largest_combination(candidates: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[16, 17, 71, 62, 12, 24, 14] as &[_], 4), (&[8, 8], 2)];

        for (candidates, expected) in test_cases {
            assert_eq!(S::largest_combination(candidates.to_vec()), expected);
        }
    }
}
