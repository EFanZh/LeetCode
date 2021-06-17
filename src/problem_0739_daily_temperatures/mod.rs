pub mod stack;

pub trait Solution {
    fn daily_temperatures(t: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(
            &[73, 74, 75, 71, 69, 72, 76, 73] as &[_],
            &[1, 1, 4, 2, 1, 1, 0, 0] as &[_],
        )];

        for (t, expected) in test_cases {
            assert_eq!(S::daily_temperatures(t.to_vec()), expected);
        }
    }
}
