pub mod bidirectional_search;

pub trait Solution {
    fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[2, 7, 11, 15] as &[_], 9), [1, 2])];

        for ((numbers, target), expected) in test_cases.iter().copied() {
            assert_eq!(S::two_sum(numbers.to_vec(), target), expected);
        }
    }
}
