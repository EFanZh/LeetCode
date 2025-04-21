pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut nums = nums;

        nums.sort_unstable();

        let mut start = nums.len();
        let mut end = nums.len();
        let mut result = 0;

        (0..).zip(&nums).for_each(|(i, &num)| {
            let lower_target = lower - num;
            let upper_target = upper - num;

            loop {
                let start_m1 = start.wrapping_sub(1);

                if nums.get(start_m1).is_some_and(|&x| x >= lower_target) {
                    start = start_m1;
                } else {
                    break;
                }
            }

            loop {
                let end_m1 = end.wrapping_sub(1);

                if nums.get(end_m1).is_some_and(|&x| x > upper_target) {
                    end = end_m1;
                } else {
                    break;
                }
            }

            result += end.min(i) - start.min(i);
        });

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        Self::count_fair_pairs(nums, lower, upper)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
