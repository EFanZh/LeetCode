pub mod two_pointers;

pub trait Solution {
    fn count_pairs(nums: Vec<i32>, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[-1, 1, 2, 3, 1] as &[_], 2), 3),
            ((&[-6, 2, 5, -2, -7, -1, 3], -2), 10),
        ];

        for ((nums, target), expected) in test_cases {
            assert_eq!(S::count_pairs(nums.to_vec(), target), expected);
        }
    }
}
