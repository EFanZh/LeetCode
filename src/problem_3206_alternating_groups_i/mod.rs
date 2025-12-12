pub mod iterative;

pub trait Solution {
    fn number_of_alternating_groups(colors: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 1, 1] as &[_], 0), (&[0, 1, 0, 0, 1], 3)];

        for (colors, expected) in test_cases {
            assert_eq!(S::number_of_alternating_groups(colors.to_vec()), expected);
        }
    }
}
