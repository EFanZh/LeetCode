pub mod greedy;

pub trait Solution {
    fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3] as &[_], 2), 4),
            ((&[1, 1, 1, 1], 2), 1),
            ((&[2, 3, 4, 5], 1), 5),
            ((&[12, 1, 42], 3), 53),
        ];

        for ((happiness, k), expected) in test_cases {
            assert_eq!(S::maximum_happiness_sum(happiness.to_vec(), k), expected);
        }
    }
}
