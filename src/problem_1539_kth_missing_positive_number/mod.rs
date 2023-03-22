pub mod binary_search;

pub trait Solution {
    fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[2, 3, 4, 7, 11] as &[_], 5), 9), ((&[1, 2, 3, 4], 2), 6)];

        for ((arr, k), expected) in test_cases {
            assert_eq!(S::find_kth_positive(arr.to_vec(), k), expected);
        }
    }
}
