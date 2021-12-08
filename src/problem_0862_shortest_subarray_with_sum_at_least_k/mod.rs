pub mod sliding_window;

pub trait Solution {
    fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1] as &[_], 1), 1),
            ((&[1, 2], 4), -1),
            ((&[2, -1, 2], 3), 3),
            ((&[17, 85, 93, -45, -21], 150), 2),
            ((&[84, -37, 32, 40, 95], 167), 3),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::shortest_subarray(nums.to_vec(), k), expected);
        }
    }
}
