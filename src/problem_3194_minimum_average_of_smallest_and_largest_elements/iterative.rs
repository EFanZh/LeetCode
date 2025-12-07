pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_average(nums: Vec<i32>) -> f64 {
        let mut nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();

        nums.sort_unstable();

        let (left, right) = nums.split_at(nums.len() / 2);

        f64::from(
            left.iter()
                .zip(right.iter().rev())
                .fold(u32::MAX, |min, (&x, &y)| min.min(x + y)),
        ) / 2.0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_average(nums: Vec<i32>) -> f64 {
        Self::minimum_average(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
