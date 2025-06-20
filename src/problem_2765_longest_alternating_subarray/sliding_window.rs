pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut expected_diff = 1;
        let mut prev = -1;
        let mut length = 0;
        let mut result = 0_u32;

        for num in nums {
            let diff = num - prev;

            if diff == expected_diff {
                expected_diff = -expected_diff;
                length += 1;
            } else {
                result = result.max(length);
                (expected_diff, length) = if diff == 1 { (-1, 2) } else { (1, 1) };
            }

            prev = num;
        }

        result = result.max(length);

        (if result < 2 { u32::MAX } else { result }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn alternating_subarray(nums: Vec<i32>) -> i32 {
        Self::alternating_subarray(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
