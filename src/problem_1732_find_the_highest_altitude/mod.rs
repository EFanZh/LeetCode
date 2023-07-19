pub mod iterative;

pub trait Solution {
    fn largest_altitude(gain: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[-5, 1, 5, 0, -7] as &[_], 1), (&[-4, -3, -2, -1, 4, 3, 2], 0)];

        for (gain, expected) in test_cases {
            assert_eq!(S::largest_altitude(gain.to_vec()), expected);
        }
    }
}
