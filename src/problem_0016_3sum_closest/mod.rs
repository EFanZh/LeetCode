pub mod two_pointers;

pub trait Solution {
    fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[-1, 2, 1, -4] as &[_], 1), 2),
            ((&[1, 1, 1, 1], 0), 3),
            ((&[1, 1, -1, -1, 3], 1), 1),
            ((&[0, 2, 1, -3], 1), 0),
        ];

        for ((nums, target), expected) in test_cases.iter().copied() {
            assert_eq!(S::three_sum_closest(nums.to_vec(), target), expected);
        }
    }
}
