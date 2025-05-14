pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let value = value as u32;

        assert_ne!(value, 0);

        let mut counts = vec![0_u32; value as usize].into_boxed_slice();

        for num in nums {
            counts[num.rem_euclid(value as _) as usize] += 1;
        }

        let mut min_count = u32::MAX;
        let mut min_index = 0;

        (0..).zip(counts).for_each(|(i, count)| {
            if count < min_count {
                min_count = count;
                min_index = i as u32;
            }
        });

        (value * min_count + min_index) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        Self::find_smallest_integer(nums, value)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
