pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_removal(beans: Vec<i32>) -> i64 {
        let mut beans = beans.into_iter().map(|x| x as u32).collect::<Vec<_>>();

        beans.sort_unstable();

        let mut left_sum = 0;
        let mut right_length = beans.len() as u64;
        let mut right_sum = beans.iter().fold(0_u64, |sum, &x| sum + u64::from(x));

        beans.into_iter().fold(u64::MAX, |mut result, value| {
            let value = u64::from(value);

            result = result.min(left_sum + (right_sum - value * right_length));

            left_sum += value;
            right_length -= 1;
            right_sum -= value;

            result
        }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_removal(beans: Vec<i32>) -> i64 {
        Self::minimum_removal(beans)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
