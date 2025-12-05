pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut mask = 0_u32;
        let mut result = 0;

        for num in nums {
            let needs_operation = num.cast_unsigned() == (mask & 1);

            mask >>= 1;

            if needs_operation {
                mask ^= 3;
                result += 1;
            }
        }

        if mask == 0 { result } else { -1 }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations(nums: Vec<i32>) -> i32 {
        Self::min_operations(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
