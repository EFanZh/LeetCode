pub mod iterative;

pub trait Solution {
    fn num_times_all_blue(flips: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[3, 2, 4, 1, 5] as &[_], 2), (&[4, 1, 2, 3], 1)];

        for (flips, expected) in test_cases {
            assert_eq!(S::num_times_all_blue(flips.to_vec()), expected);
        }
    }
}
