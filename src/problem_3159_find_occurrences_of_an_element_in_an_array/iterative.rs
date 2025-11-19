pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
        let mut nums = nums;
        let nums = &mut *nums;
        let mut count = 0;

        for i in 0..nums.len() {
            if nums[i] == x {
                nums[count] = i as _;
                count += 1;
            }
        }

        let indices = &mut nums[..count];

        queries
            .into_iter()
            .map(|i| indices.get(i.cast_unsigned() as usize - 1).copied().unwrap_or(-1))
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
        Self::occurrences_of_element(nums, queries, x)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
