pub mod greedy;

pub trait Solution {
    fn max_distance(colors: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 1, 1, 6, 1, 1, 1] as &[_], 3), (&[1, 8, 3, 8, 3], 4), (&[0, 1], 1)];

        for (colors, expected) in test_cases {
            assert_eq!(S::max_distance(colors.to_vec()), expected);
        }
    }
}
