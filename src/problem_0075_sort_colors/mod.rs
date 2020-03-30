pub mod counting_sort;

pub trait Solution {
    fn sort_colors(nums: &mut Vec<i32>);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run_tests<S: Solution>() {
        let test_cases = [(&[2, 0, 2, 1, 1, 0] as &[_], &[0, 0, 1, 1, 2, 2] as &[_])];

        for (nums, expected) in test_cases.iter().copied() {
            let mut nums = nums.to_owned();

            S::sort_colors(&mut nums);

            assert_eq!(nums, expected);
        }
    }
}
