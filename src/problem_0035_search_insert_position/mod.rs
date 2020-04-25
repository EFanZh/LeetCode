pub mod binary_search;
pub mod binary_search_fast;
pub mod binary_search_fast_2;
pub mod binary_search_recursive;

pub trait Solution {
    fn search_insert(nums: Vec<i32>, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3, 5, 6], 5), 2),
            ((&[1, 3, 5, 6], 2), 1),
            ((&[1, 3, 5, 6], 7), 4),
            ((&[1, 3, 5, 6], 0), 0),
        ];

        for ((nums, target), expected) in test_cases.iter().copied() {
            assert_eq!(S::search_insert(nums.to_vec(), target), expected);
        }
    }
}
