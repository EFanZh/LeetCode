pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_total_sum(maximum_height: Vec<i32>) -> i64 {
        let mut maximum_height = maximum_height.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();

        maximum_height.sort_unstable();

        let mut result = 0;
        let mut prev = u32::MAX;

        while let Some(max) = maximum_height.pop() {
            if prev < 2 {
                return -1;
            }

            prev = (prev - 1).min(max);
            result += u64::from(prev);
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_total_sum(maximum_height: Vec<i32>) -> i64 {
        Self::maximum_total_sum(maximum_height)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
