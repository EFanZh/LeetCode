pub mod greedy;
pub mod greedy_binary_heap;

pub trait Solution {
    fn max_satisfaction(satisfaction: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[-1, -8, 0, 5, -9] as &[_], 14), (&[4, 3, 2], 20), (&[-1, -4, -5], 0)];

        for (satisfaction, expected) in test_cases {
            assert_eq!(S::max_satisfaction(satisfaction.to_vec()), expected);
        }
    }
}
