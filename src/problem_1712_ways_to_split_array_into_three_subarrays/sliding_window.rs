pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn ways_to_split(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let sum = nums.iter().sum::<i32>();
        let max_left_and_middle_sum = sum * 2 / 3;
        let mut min_left_split = 1;
        let mut min_left_sum = nums[0];
        let mut max_left_split = 1;
        let mut max_left_sum = min_left_sum;
        let mut left_and_middle_sum = min_left_sum;
        let mut result = 0_u64;

        for (i, num) in (1..).zip(&nums[1..n - 1]) {
            left_and_middle_sum += num;

            if left_and_middle_sum > max_left_and_middle_sum {
                break;
            }

            while min_left_split < i && min_left_sum < left_and_middle_sum * 2 - sum {
                min_left_sum += nums[min_left_split];
                min_left_split += 1;
            }

            while max_left_split <= i && max_left_sum * 2 <= left_and_middle_sum {
                max_left_sum += nums[max_left_split];
                max_left_split += 1;
            }

            result += (max_left_split - min_left_split) as u64;
        }

        (result % 1_000_000_007) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn ways_to_split(nums: Vec<i32>) -> i32 {
        Self::ways_to_split(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
