pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_increment_for_unique(nums: Vec<i32>) -> i32 {
        let mut nums = nums;

        nums.sort_unstable();

        let mut result = 0;
        let mut prev = -1;

        for num in nums {
            if num <= prev {
                prev += 1;
                result += prev - num;
            } else {
                prev = num;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_increment_for_unique(nums: Vec<i32>) -> i32 {
        Self::min_increment_for_unique(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
