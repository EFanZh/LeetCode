pub mod modular_arithmetic;

pub trait Solution {
    fn num_pairs_divisible_by60(time: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[30, 20, 150, 100, 40] as &[_], 3), (&[60, 60, 60], 3)];

        for (time, expected) in test_cases {
            assert_eq!(S::num_pairs_divisible_by60(time.to_vec()), expected);
        }
    }
}
