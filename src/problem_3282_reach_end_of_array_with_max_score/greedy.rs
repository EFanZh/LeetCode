pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_maximum_score(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        let mut prev = 0;

        nums.pop();

        nums.into_iter()
            .map(|num| {
                prev = prev.max(u64::from(num.cast_unsigned()));

                prev
            })
            .sum::<u64>()
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_maximum_score(nums: Vec<i32>) -> i64 {
        Self::find_maximum_score(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
