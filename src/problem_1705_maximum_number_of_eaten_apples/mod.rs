pub mod greedy_binary_heap;

pub trait Solution {
    fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 5, 2] as &[_], &[3, 2, 1, 4, 2] as &[_]), 7),
            ((&[3, 0, 0, 0, 0, 2], &[3, 0, 0, 0, 0, 2]), 5),
        ];

        for ((apples, days), expected) in test_cases {
            assert_eq!(S::eaten_apples(apples.to_vec(), days.to_vec()), expected);
        }
    }
}
