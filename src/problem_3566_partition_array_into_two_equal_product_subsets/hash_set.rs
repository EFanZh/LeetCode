pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;
use std::num::NonZero;

impl Solution {
    pub fn check_equal_partitions(nums: Vec<i32>, target: i64) -> bool {
        let target = target.cast_unsigned();

        if nums
            .iter()
            .copied()
            .map(i32::cast_unsigned)
            .map(u128::from)
            .product::<u128>()
            != u128::from(target).pow(2)
        {
            return false;
        }

        let Some(nums) = nums
            .into_iter()
            .map(i32::cast_unsigned)
            .map(|num| NonZero::new(num).filter(|&num| target % NonZero::from(num) == 0))
            .collect::<Option<Vec<_>>>()
        else {
            return false;
        };

        let mut values = HashSet::<u64>::from([1]);
        let mut buffer = Vec::new();

        for &num in &nums {
            let num = NonZero::from(num);

            if values.contains(&(target / num)) {
                return true;
            }

            buffer.extend(values.iter().filter_map(|&lhs| lhs.checked_mul(num.get())));
            values.extend(&buffer);
            buffer.clear();
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_equal_partitions(nums: Vec<i32>, target: i64) -> bool {
        Self::check_equal_partitions(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
