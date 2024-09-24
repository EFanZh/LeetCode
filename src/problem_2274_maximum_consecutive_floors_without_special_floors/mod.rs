pub mod iterative;

pub trait Solution {
    fn max_consecutive(bottom: i32, top: i32, special: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((2, 9, &[4, 6] as &[_]), 3), ((6, 8, &[7, 6, 8]), 0)];

        for ((bottom, top, special), expected) in test_cases {
            assert_eq!(S::max_consecutive(bottom, top, special.to_vec()), expected);
        }
    }
}
