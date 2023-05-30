pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut prev = 0;

        for &value in &target {
            let value = value as u32;

            result += value.saturating_sub(prev);

            prev = value;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_number_operations(target: Vec<i32>) -> i32 {
        Self::min_number_operations(target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
