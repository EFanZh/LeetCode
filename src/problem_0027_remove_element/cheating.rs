pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|x| *x != val);

        nums.len() as _
    }
}

impl super::Solution for Solution {
    fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        Self::remove_element(nums, val)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
