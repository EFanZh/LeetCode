pub mod iterative;

pub trait Solution {
    fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4] as &[_], &[2, 4, 1, 3] as &[_]), true),
            ((&[7], &[7]), true),
            ((&[3, 7, 9], &[3, 7, 11]), false),
        ];

        for ((target, arr), expected) in test_cases {
            assert_eq!(S::can_be_equal(target.to_vec(), arr.to_vec()), expected);
        }
    }
}
