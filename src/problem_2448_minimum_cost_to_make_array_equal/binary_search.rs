pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn get_cost(nums: &[i32], cost: &[i32], target: i32) -> u64 {
        nums.iter()
            .zip(cost)
            .map(|(&num, &cost)| u64::from(num.abs_diff(target)) * u64::from(cost as u32))
            .sum()
    }

    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        let mut left = nums.iter().fold(i32::MAX, |min, &num| min.min(num));
        let mut right = nums.iter().fold(i32::MIN, |max, &num| max.max(num));

        while left < right {
            let middle = ((left ^ right) >> 1) + (left & right);

            if Self::get_cost(&nums, &cost, middle) < Self::get_cost(&nums, &cost, middle - 1) {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        Self::get_cost(&nums, &cost, left - 1).min(Self::get_cost(&nums, &cost, left)) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        Self::min_cost(nums, cost)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
