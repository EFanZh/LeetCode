pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut prev = i32::MIN;
        let mut sum = 0;
        let mut max_sum = 0;

        for num in nums {
            if num > prev {
                sum += num;
            } else {
                max_sum = max_sum.max(sum);
                sum = num;
            }

            prev = num;
        }

        max_sum = max_sum.max(sum);

        max_sum
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        Self::max_ascending_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
