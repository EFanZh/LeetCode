pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut target = 0;
        let mut cost_1 = 0;
        let mut cost_2 = 0;

        (0..).zip(nums).for_each(|(i, num)| {
            if num & 1 == 0 {
                cost_1 += i32::abs_diff(i, target);
                cost_2 += i32::abs_diff(i, target + 1);
                target += 2;
            }
        });

        match target - n {
            -1 => cost_2,
            0 => u32::min(cost_1, cost_2),
            1 => cost_1,
            _ => u32::MAX,
        }
        .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_swaps(nums: Vec<i32>) -> i32 {
        Self::min_swaps(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
