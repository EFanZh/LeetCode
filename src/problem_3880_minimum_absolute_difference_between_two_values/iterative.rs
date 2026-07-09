pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_absolute_difference(nums: Vec<i32>) -> i32 {
        let mut prev = 0;
        let mut prev_index = 0;
        let mut result = u32::MAX;

        (0_u32..).zip(nums).for_each(|(i, num)| {
            if num != 0 {
                if prev + num == 3 {
                    result = result.min(i - prev_index);
                }

                prev = num;
                prev_index = i;
            }
        });

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_absolute_difference(nums: Vec<i32>) -> i32 {
        Self::min_absolute_difference(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
