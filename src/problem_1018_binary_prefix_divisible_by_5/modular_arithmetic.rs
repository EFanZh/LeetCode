pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut value = 0_u32;

        nums.into_iter()
            .map(|x| {
                value = value * 2 + x as u32;
                value = value.checked_sub(5).unwrap_or(value);

                value == 0
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        Self::prefixes_div_by5(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
