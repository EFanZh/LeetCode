pub mod dynamic_programming;

pub trait Solution {
    fn make_array_increasing(arr1: Vec<i32>, arr2: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 5, 3, 6, 7] as &[_], &[1, 3, 2, 4] as &[_]), 1),
            ((&[1, 5, 3, 6, 7], &[4, 3, 1]), 2),
            ((&[1, 5, 3, 6, 7], &[1, 6, 3, 3]), -1),
        ];

        for ((arr1, arr2), expected) in test_cases {
            assert_eq!(S::make_array_increasing(arr1.to_vec(), arr2.to_vec()), expected);
        }
    }
}
