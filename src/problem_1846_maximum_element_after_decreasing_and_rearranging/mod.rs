pub mod greedy;

pub trait Solution {
    fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 2, 1, 2, 1] as &[_], 2),
            (&[100, 1, 1000], 3),
            (&[1, 2, 3, 4, 5], 5),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(
                S::maximum_element_after_decrementing_and_rearranging(arr.to_vec()),
                expected,
            );
        }
    }
}
