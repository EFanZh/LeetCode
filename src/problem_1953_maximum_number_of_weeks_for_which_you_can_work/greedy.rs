pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper(milestones: Vec<u32>) -> u64 {
        let (sum, max) = milestones
            .iter()
            .fold((0_u64, 0_u32), |(sum, max), &x| (sum + u64::from(x), max.max(x)));

        let other = sum - u64::from(max);

        if u64::from(max) <= other + 1 {
            sum
        } else {
            other * 2 + 1
        }
    }

    pub fn number_of_weeks(milestones: Vec<i32>) -> i64 {
        Self::helper(milestones.into_iter().map(|x| x as _).collect()) as _
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
