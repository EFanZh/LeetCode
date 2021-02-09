pub struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        nums.split_first().map_or(0, |(&first, rest)| {
            let mut min = first;
            let mut sum = first;

            for &num in rest {
                min = min.min(num);
                sum += num;
            }

            sum - min * (nums.len() as i32)
        })
    }
}

impl super::Solution for Solution {
    fn min_moves(nums: Vec<i32>) -> i32 {
        Self::min_moves(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
