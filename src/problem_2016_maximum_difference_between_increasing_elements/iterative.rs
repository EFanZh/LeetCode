pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut result = -1;
        let mut min = i32::MAX;

        for num in nums {
            min = min.min(num);

            if num > min {
                result = result.max(num - min);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_difference(nums: Vec<i32>) -> i32 {
        Self::maximum_difference(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
