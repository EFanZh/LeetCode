pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut right_counts = [0_u8; 50];
        let mut right_unique_count = 0;

        for &num in &nums {
            let count = &mut right_counts[num as u32 as usize - 1];

            right_unique_count += i32::from(*count == 0);
            *count += 1;
        }

        let mut left_counts = [0_u8; 50];
        let mut left_unique_count = 0;

        for num in &mut nums {
            let index = *num as u32 as usize - 1;
            let left_count = &mut left_counts[index];

            left_unique_count += i32::from(*left_count == 0);
            *left_count += 1;

            let right_count = &mut right_counts[index];

            *right_count -= 1;
            right_unique_count -= i32::from(*right_count == 0);

            *num = left_unique_count - right_unique_count;
        }

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        Self::distinct_difference_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
