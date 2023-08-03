pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut counts = [0_u8; 100];
        let mut result = 0;

        for num in nums {
            let count = &mut counts[num as u32 as usize - 1];

            if *count < 2 {
                result += (1 - i32::from(*count) * 2) * num;
                *count += 1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_of_unique(nums: Vec<i32>) -> i32 {
        Self::sum_of_unique(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
