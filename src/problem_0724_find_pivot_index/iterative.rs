pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let total = nums.iter().sum::<i32>();
        let mut left_sum = 0;

        for (i, num) in (0..).zip(nums) {
            let right_sum = total - left_sum - num;

            if left_sum == right_sum {
                return i;
            }

            left_sum += num;
        }

        -1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn pivot_index(nums: Vec<i32>) -> i32 {
        Self::pivot_index(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
