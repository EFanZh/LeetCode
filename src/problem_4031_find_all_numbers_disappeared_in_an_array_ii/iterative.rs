pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn three_way_partition(nums: &mut [u32], lower: u32, upper: u32) -> (usize, usize) {
        let mut middle_start = 0;
        let mut middle_end = 0;
        let mut greater_start = nums.len();

        while middle_end < greater_start {
            let num = nums[middle_end];

            if num < lower {
                nums.swap(middle_start, middle_end);
                middle_start += 1;
            } else if upper < num {
                greater_start -= 1;
                nums.swap(middle_end, greater_start);

                continue;
            }

            middle_end += 1;
        }

        (middle_start, middle_end)
    }

    pub fn find_disappeared_numbers(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<Vec<i32>> {
        let mut nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let lower = lower.cast_unsigned();
        let upper = upper.cast_unsigned();
        let (middle_start, middle_end) = Self::three_way_partition(&mut nums, lower, upper);
        let middle = &mut nums[middle_start..middle_end];

        middle.sort_unstable();

        let mut result = Vec::new();
        let mut expected = lower;

        for &num in &*middle {
            if num > expected {
                result.push(vec![expected.cast_signed(), num.cast_signed() - 1]);
            }

            expected = num + 1;
        }

        if upper + 1 != expected {
            result.push(vec![expected.cast_signed(), upper.cast_signed()]);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_disappeared_numbers(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<Vec<i32>> {
        Self::find_disappeared_numbers(nums, lower, upper)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
