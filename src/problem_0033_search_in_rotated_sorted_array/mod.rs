pub mod binary_search;
pub mod binary_search_2;

pub trait Solution {
    fn search(nums: Vec<i32>, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[4, 5, 6, 7, 0, 1, 2] as &[_], 0), 4),
            ((&[4, 5, 6, 7, 0, 1, 2], 3), -1),
            ((&[1], 0), -1),
            ((&[1], 1), 0),
        ];

        for ((nums, target), expected) in test_cases.iter().copied() {
            assert_eq!(S::search(nums.to_vec(), target), expected);
        }
    }
}
