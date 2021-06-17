pub mod index_map;
pub mod sort_then_bidirectional_search;
pub mod sort_then_bidirectional_search_2;

pub trait Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 7, 11, 15] as &[_], 9), [0, 1]),
            ((&[-3, 4, 3, 90], 0), [0, 2]),
            ((&[3, 2, 4], 6), [1, 2]),
            ((&[3, 2, 3], 6), [0, 2]),
        ];

        for ((nums, target), expected) in test_cases {
            assert_eq!(S::two_sum(nums.to_vec(), target), expected);
        }
    }
}
