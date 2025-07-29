pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let nums = nums.into_iter().map(|num| num as u32).collect::<Vec<_>>();
        let n = nums.len();

        assert!(n > 1);

        let mut right = vec![0; n - 2];
        let mut max = 0;

        right.iter_mut().zip(&nums[2..]).rev().for_each(|(target, &num)| {
            max = max.max(num);
            *target = max;
        });

        let mut left_max = nums[0];

        nums[1..].iter().zip(&right).fold(0, |max, (&middle, &right_max)| {
            let max = max.max(i64::from(left_max.wrapping_sub(middle) as i32) * i64::from(right_max));

            left_max = left_max.max(middle);

            max
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        Self::maximum_triplet_value(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
