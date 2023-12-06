pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let total_sum = nums.iter().sum::<i32>();
        let mut result = -1;
        let mut sum = 0;

        for (i, num) in (0..).zip(nums) {
            if sum + sum + num == total_sum {
                result = i;

                break;
            }

            sum += num;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_middle_index(nums: Vec<i32>) -> i32 {
        Self::find_middle_index(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
