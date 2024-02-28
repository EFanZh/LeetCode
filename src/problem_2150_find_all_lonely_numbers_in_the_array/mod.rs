pub mod hash_map;

pub trait Solution {
    fn find_lonely(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [(&[10, 6, 5, 8] as &[_], &[8, 10] as &[_]), (&[1, 3, 5, 3], &[1, 5])];

        for (nums, expected) in test_cases {
            assert_eq!(test_utilities::unstable_sorted(S::find_lonely(nums.to_vec())), expected,);
        }
    }
}
