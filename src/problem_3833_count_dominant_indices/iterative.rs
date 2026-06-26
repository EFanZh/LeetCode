pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn dominant_indices(nums: Vec<i32>) -> i32 {
        let mut nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut sum = nums.pop().unwrap();
        let mut count = 1;
        let mut result = 0;

        while let Some(num) = nums.pop() {
            result += i32::from(num * count > sum);
            sum += num;
            count += 1;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn dominant_indices(nums: Vec<i32>) -> i32 {
        Self::dominant_indices(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
