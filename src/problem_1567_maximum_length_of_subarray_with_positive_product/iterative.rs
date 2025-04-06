pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        let mut result = 0_u32;
        let mut negative = 0;
        let mut positive = 0;

        for num in nums {
            if num == 0 {
                negative = 0;
                positive = 0;
            } else {
                let next_negative = if negative == 0 { 0 } else { negative + 1 };
                let next_positive = positive + 1;

                if num < 0 {
                    negative = next_positive;
                    positive = next_negative;
                } else {
                    negative = next_negative;
                    positive = next_positive;
                }

                result = result.max(positive);
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_max_len(nums: Vec<i32>) -> i32 {
        Self::get_max_len(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
