pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn is_self_crossing(x: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 1, 1, 2] as &[_], true),
            (&[1, 2, 3, 4], false),
            (&[1, 1, 1, 1], true),
        ];

        for (x, expected) in test_cases.iter().copied() {
            assert_eq!(S::is_self_crossing(x.to_vec()), expected);
        }
    }
}
