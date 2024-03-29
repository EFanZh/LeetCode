pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        nums.iter()
            .copied()
            .min_by_key(|&x| if x < 0 { 1 - x * 2 } else { x * 2 })
            .unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_closest_number(nums: Vec<i32>) -> i32 {
        Self::find_closest_number(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
