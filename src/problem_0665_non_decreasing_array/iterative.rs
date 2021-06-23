pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn is_sorted(nums: &[i32]) -> bool {
        nums.iter().zip(&nums[1..]).all(|(lhs, rhs)| lhs <= rhs)
    }

    pub fn check_possibility(nums: Vec<i32>) -> bool {
        if let Some((i, (lhs, rhs))) = nums
            .iter()
            .zip(&nums[1..])
            .enumerate()
            .find(|(_, (lhs, rhs))| lhs > rhs)
        {
            if nums.get(i.wrapping_sub(1)).map_or(true, |x| x <= rhs) {
                Self::is_sorted(&nums[i + 1..])
            } else {
                let suffix = &nums[i + 2..];

                suffix
                    .first()
                    .map_or(true, |suffix_first| lhs <= suffix_first && Self::is_sorted(suffix))
            }
        } else {
            true
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_possibility(nums: Vec<i32>) -> bool {
        Self::check_possibility(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
