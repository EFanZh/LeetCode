pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let (left, right) = nums.split_at(k as _);
        let mut sum = left.iter().sum::<i32>();
        let mut max_sum = sum;

        for (old, new) in nums.iter().zip(right) {
            sum += new - old;
            max_sum = max_sum.max(sum);
        }

        f64::from(max_sum) / f64::from(k)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        Self::find_max_average(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
