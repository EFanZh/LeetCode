pub mod array;
pub mod hash_map;

pub trait Solution {
    fn get_largest_outlier(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 3, 5, 10] as &[_], 10),
            (&[-2, -1, -3, -6, 4], 4),
            (&[1, 1, 1, 1, 1, 5, 5], 5),
            (&[-947, -326, 200, -747], -326),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::get_largest_outlier(nums.to_vec()), expected);
        }
    }
}
