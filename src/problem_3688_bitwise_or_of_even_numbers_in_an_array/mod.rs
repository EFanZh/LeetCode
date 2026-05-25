pub mod sliding_window;

pub trait Solution {
    fn even_number_bitwise_o_rs(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3, 4, 5, 6] as &[_], 6), (&[7, 9, 11], 0), (&[1, 8, 16], 24)];

        for (nums, expected) in test_cases {
            assert_eq!(S::even_number_bitwise_o_rs(nums.to_vec()), expected);
        }
    }
}
