pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_middle_element_unique(nums: Vec<i32>) -> bool {
        let half = nums.len() / 2;
        let middle = nums[half];

        !(nums[..half].contains(&middle) || nums[half + 1..].contains(&middle))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_middle_element_unique(nums: Vec<i32>) -> bool {
        Self::is_middle_element_unique(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
