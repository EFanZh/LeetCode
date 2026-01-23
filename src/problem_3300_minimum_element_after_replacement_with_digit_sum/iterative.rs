pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold(u32::MAX, |min, &num| {
                let mut sum = 0;
                let mut num = num.cast_unsigned();

                while num != 0 {
                    sum += num % 10;
                    num /= 10;
                }

                min.min(sum)
            })
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_element(nums: Vec<i32>) -> i32 {
        Self::min_element(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
