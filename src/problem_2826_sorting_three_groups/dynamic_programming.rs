pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut max_length_1 = 0;
        let mut max_length_2 = 0;
        let mut max_length_3 = 0;
        let n = nums.len() as u32;

        for num in nums {
            match num {
                1 => max_length_1 += 1,
                2 => max_length_2 = u32::max(max_length_1, max_length_2) + 1,
                _ => max_length_3 = u32::max(u32::max(max_length_1, max_length_2), max_length_3) + 1,
            }
        }

        let max_length = u32::max(u32::max(max_length_1, max_length_2), max_length_3);

        (n - max_length) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_operations(nums: Vec<i32>) -> i32 {
        Self::minimum_operations(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
