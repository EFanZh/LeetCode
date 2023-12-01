pub mod iterative;

pub trait Solution {
    fn find_different_binary_string(nums: Vec<String>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [&["01", "10"] as &[_], &["00", "01"], &["111", "011", "001"]];

        for nums in test_cases {
            let result = S::find_different_binary_string(nums.iter().copied().map(str::to_string).collect());

            assert_eq!(result.len(), nums[0].len());
            assert!(!nums.contains(&result.as_str()));
        }
    }
}
