pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut prev_length = 0;
        let mut length = 0;
        let mut prev = i32::MIN;

        for num in nums {
            if num > prev {
                length += 1;
            } else {
                result = result.max(u32::min(prev_length, length)).max(length / 2);
                prev_length = length;
                length = 1;
            }

            prev = num;
        }

        result.max(u32::min(prev_length, length)).max(length / 2).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        Self::max_increasing_subarrays(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
