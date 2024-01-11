pub mod iterative;

pub trait Solution {
    fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[0, 1, 1, 3] as &[_], 2), &[0, 3, 2, 3] as &[_]),
            ((&[2, 3, 4, 7], 3), &[5, 2, 6, 5]),
            ((&[0, 1, 2, 2, 5, 7], 3), &[4, 3, 6, 4, 6, 7]),
        ];

        for ((nums, maximum_bit), expected) in test_cases {
            assert_eq!(S::get_maximum_xor(nums.to_vec(), maximum_bit), expected);
        }
    }
}
