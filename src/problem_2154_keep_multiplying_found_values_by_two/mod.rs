pub mod hash_set;

pub trait Solution {
    fn find_final_value(nums: Vec<i32>, original: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[5, 3, 6, 1, 12] as &[_], 3), 24), ((&[2, 7, 9], 4), 4)];

        for ((nums, original), expected) in test_cases {
            assert_eq!(S::find_final_value(nums.to_vec(), original), expected);
        }
    }
}
