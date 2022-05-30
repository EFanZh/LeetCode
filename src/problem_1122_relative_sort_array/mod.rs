pub mod hash_map;

pub trait Solution {
    fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19] as &[_], &[2, 1, 4, 3, 9, 6] as &[_]),
                &[2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19] as &[_],
            ),
            ((&[28, 6, 22, 8, 44, 17], &[22, 28, 8, 6]), &[22, 28, 8, 6, 17, 44]),
        ];

        for ((arr1, arr2), expected) in test_cases {
            assert_eq!(S::relative_sort_array(arr1.to_vec(), arr2.to_vec()), expected);
        }
    }
}
