pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());

        for (num, index) in nums.into_iter().zip(index) {
            result.insert(index as usize, num);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        Self::create_target_array(nums, index)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
