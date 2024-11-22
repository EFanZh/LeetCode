pub mod mathematical;
pub mod mathematical_2;

pub trait Solution {
    fn fill_cups(amount: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ([1, 4, 2], 4),
            ([5, 4, 4], 7),
            ([5, 0, 0], 5),
            ([2, 2, 0], 2),
            ([0, 0, 0], 0),
        ];

        for (amount, expected) in test_cases {
            assert_eq!(S::fill_cups(amount.to_vec()), expected);
        }
    }
}
