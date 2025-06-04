pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_prefix_score(nums: Vec<i32>) -> Vec<i64> {
        let mut max = 0;
        let mut score = 0;

        nums.into_iter()
            .map(|num| {
                let num = num as u32;

                max = max.max(num);
                score += u64::from(num + max);

                score as _
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_prefix_score(nums: Vec<i32>) -> Vec<i64> {
        Self::find_prefix_score(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
