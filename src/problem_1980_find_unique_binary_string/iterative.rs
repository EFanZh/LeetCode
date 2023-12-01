pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        String::from_utf8(
            nums.into_iter()
                .enumerate()
                .map(|(i, s)| b'0' + b'1' - s.as_bytes()[i])
                .collect(),
        )
        .unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_different_binary_string(nums: Vec<String>) -> String {
        Self::find_different_binary_string(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
