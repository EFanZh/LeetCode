pub mod iterative;

pub trait Solution {
    fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 1, 1, 1, 1] as &[_], &[1, 0, 1] as &[_]), &[1, 0, 0, 0, 0] as &[_]),
            ((&[0], &[0]), &[0]),
            ((&[0], &[1]), &[1]),
            ((&[1], &[1]), &[1, 1, 0]),
            ((&[1], &[1, 1]), &[0]),
            ((&[1], &[1, 1, 0, 1]), &[1, 0]),
            ((&[1, 0, 1], &[1, 0, 1]), &[1, 1, 1, 1, 0]),
            ((&[1], &[1, 0, 0, 1]), &[1, 1, 1, 0]),
        ];

        for ((arr1, arr2), expected) in test_cases {
            assert_eq!(S::add_negabinary(arr1.to_vec(), arr2.to_vec()), expected);
        }
    }
}
