pub mod iterative;

pub trait Solution {
    fn sum_of_good_numbers(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 3, 2, 1, 5, 4] as &[_], 2), 12), ((&[2, 1], 1), 2)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::sum_of_good_numbers(nums.to_vec(), k), expected);
        }
    }
}
