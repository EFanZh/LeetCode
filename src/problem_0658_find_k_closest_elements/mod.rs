pub mod binary_search;

pub trait Solution {
    fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5] as &[_], 4, 3), &[1, 2, 3, 4] as &[_]),
            ((&[1, 2, 3, 4, 5], 4, -1), &[1, 2, 3, 4]),
            ((&[1, 1, 1, 10, 10, 10], 1, 9), &[10]),
        ];

        for ((arr, k, x), expected) in test_cases {
            assert_eq!(S::find_closest_elements(arr.to_vec(), k, x), expected);
        }
    }
}
