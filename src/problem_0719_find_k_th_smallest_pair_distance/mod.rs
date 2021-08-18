pub mod binary_search;

pub trait Solution {
    fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3, 1] as &[_], 1), 0),
            ((&[1, 1, 1], 2), 0),
            ((&[1, 6, 1], 3), 5),
            ((&[38, 33, 57, 65, 13, 2, 86, 75, 4, 56], 26), 36),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::smallest_distance_pair(nums.to_vec(), k), expected);
        }
    }
}
