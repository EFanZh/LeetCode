pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper(milestones: Vec<u32>) -> u64 {
        let mut iter = milestones.iter().copied();
        let mut sum = 0_u32;
        let mut max = 0_u32;

        while let Some(num) = iter.next() {
            if let Some(next_sum) = sum.checked_add(num) {
                sum = next_sum;
                max = max.max(num);
            } else {
                return iter.fold(u64::from(sum) + u64::from(num), |sum, x| sum + u64::from(x));
            }
        }

        let other = sum - max;

        u64::from(if max <= other + 1 { sum } else { other * 2 + 1 })
    }

    pub fn number_of_weeks(milestones: Vec<i32>) -> i64 {
        Self::helper(milestones.into_iter().map(i32::cast_unsigned).collect()) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_weeks(milestones: Vec<i32>) -> i64 {
        Self::number_of_weeks(milestones)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
