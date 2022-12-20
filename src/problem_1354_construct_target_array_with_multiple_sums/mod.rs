pub mod restore_original_state;

pub trait Solution {
    fn is_possible(target: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[9, 3, 5] as &[_], true),
            (&[1, 1, 1, 2], false),
            (&[8, 5], true),
            (&[9, 9, 9], false),
            (&[1, 1_000_000_000], true),
            (&[1], true),
        ];

        for (target, expected) in test_cases {
            assert_eq!(S::is_possible(target.to_vec()), expected);
        }
    }
}
