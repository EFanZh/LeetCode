pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_product(nums: Vec<i32>, m: i32) -> i64 {
        let mut iter = nums
            .iter()
            .copied()
            .map(i64::from)
            .zip(nums[m.cast_unsigned() as usize - 1..].iter().copied().map(i64::from));

        let (left, right) = iter.next().unwrap();
        let mut min = left;
        let mut max = min;
        let mut result = if right < 0 { min } else { max } * right;

        iter.for_each(|(left, right)| {
            if left < min {
                min = left;
            } else if left > max {
                max = left;
            }

            result = result.max(right * if right < 0 { min } else { max });
        });

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_product(nums: Vec<i32>, m: i32) -> i64 {
        Self::maximum_product(nums, m)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
