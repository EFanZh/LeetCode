pub mod quick_select;

pub trait Solution {
    fn kth_largest_number(nums: Vec<String>, k: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["3", "6", "7", "10"] as &[_], 4), "3"),
            ((&["2", "21", "12", "1"], 3), "2"),
            ((&["0", "0"], 2), "0"),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(
                S::kth_largest_number(nums.iter().copied().map(str::to_string).collect(), k),
                expected,
            );
        }
    }
}
