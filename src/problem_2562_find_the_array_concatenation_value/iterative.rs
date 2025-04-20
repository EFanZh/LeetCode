pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let half = nums.len() / 2;
        let left = &nums[..half];
        let right = &nums[nums.len() - half..];
        let mut result = 0;

        left.iter().zip(right.iter().rev()).for_each(|(&left, &right)| {
            let mut base = 1;

            for x in [right, left] {
                let mut x = x as u32;

                while x != 0 {
                    result += u64::from(x % 10) * base;
                    x /= 10;
                    base *= 10;
                }
            }
        });

        if let [x] = nums[half..nums.len() - half] {
            let mut base = 1;
            let mut x = x as u32;

            while x != 0 {
                result += u64::from(x % 10) * base;
                x /= 10;
                base *= 10;
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        Self::find_the_array_conc_val(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
