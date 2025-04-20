pub mod iterative;

pub trait Solution {
    fn find_the_array_conc_val(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[7, 52, 2, 4] as &[_], 596), (&[5, 14, 13, 8, 12], 673)];

        for (nums, expected) in test_cases {
            assert_eq!(S::find_the_array_conc_val(nums.to_vec()), expected);
        }
    }
}
