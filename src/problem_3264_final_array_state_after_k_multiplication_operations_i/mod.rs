pub mod binary_heap;

pub trait Solution {
    fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 1, 3, 5, 6] as &[_], 5, 2), &[8, 4, 6, 5, 6] as &[_]),
            ((&[1, 2], 3, 4), &[16, 8]),
        ];

        for ((nums, k, multiplier), expected) in test_cases {
            assert_eq!(S::get_final_state(nums.to_vec(), k, multiplier), expected);
        }
    }
}
