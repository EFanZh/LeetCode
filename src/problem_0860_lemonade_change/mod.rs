pub mod iterative;

pub trait Solution {
    fn lemonade_change(bills: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[5, 5, 5, 10, 20] as &[_], true),
            (&[5, 5, 10, 10, 20], false),
            (&[5, 5, 10], true),
            (&[10, 10], false),
            (&[5, 5, 5, 10, 5, 5, 10, 20, 20, 20], false),
            (&[5, 5, 5, 5, 20, 20, 5, 5, 20, 5], false),
        ];

        for (bills, expected) in test_cases {
            assert_eq!(S::lemonade_change(bills.to_vec()), expected);
        }
    }
}
