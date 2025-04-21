pub mod sliding_window;

pub trait Solution {
    fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[0, 1, 7, 4, 4, 5] as &[_], 3, 6), 6),
            ((&[1, 7, 9, 2, 5], 11, 11), 1),
            ((&[0, 0, 0, 0, 0, 0], 0, 0), 15),
        ];

        for ((nums, lower, upper), expected) in test_cases {
            assert_eq!(S::count_fair_pairs(nums.to_vec(), lower, upper), expected);
        }
    }
}
