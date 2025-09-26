pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let mut sum = 0_u32;
        let mut count = 0_u32;

        for num in nums {
            let num = num as u32;

            if num.is_multiple_of(6) {
                sum += num;
                count += 1;
            }
        }

        sum.checked_div(count).unwrap_or(0) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn average_value(nums: Vec<i32>) -> i32 {
        Self::average_value(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
