pub mod divide_by_3;

pub trait Solution {
    fn sum_of_three(num: i64) -> Vec<i64>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(33, &[10_i64, 11, 12] as &[_]), (4, &[])];

        for (num, expected) in test_cases {
            assert_eq!(S::sum_of_three(num), expected);
        }
    }
}
