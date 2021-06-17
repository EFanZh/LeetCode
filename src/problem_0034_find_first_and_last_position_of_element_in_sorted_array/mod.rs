pub mod binary_search;
pub mod binary_search_2;

pub trait Solution {
    fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[5, 7, 7, 8, 8, 10] as &[_], 8), [3, 4]),
            ((&[5, 7, 7, 8, 8, 10], 6), [-1, -1]),
            ((&[1, 2, 3] as &[_], 3), [2, 2]),
        ];

        for ((nums, target), expected) in test_cases {
            assert_eq!(S::search_range(nums.to_vec(), target), expected);
        }
    }
}
