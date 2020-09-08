pub struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums_string = nums.iter().map(i32::to_string).collect::<Vec<_>>();

        nums_string.sort_unstable_by(|lhs, rhs| rhs.bytes().chain(lhs.bytes()).cmp(lhs.bytes().chain(rhs.bytes())));

        if nums_string.first().map(String::as_str) == Some("0") {
            "0".to_string()
        } else {
            nums_string.join("")
        }
    }
}

impl super::Solution for Solution {
    fn largest_number(nums: Vec<i32>) -> String {
        Self::largest_number(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
