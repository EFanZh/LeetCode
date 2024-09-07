pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_even_split(final_sum: i64) -> Vec<i64> {
        let final_sum = final_sum as u64;

        if final_sum % 2 == 0 {
            #[expect(clippy::cast_precision_loss, reason = "optimal")]
            let count = (((final_sum * 4 + 1) as f64).sqrt() as u64 - 1) / 2;

            let mut result = Vec::with_capacity(count as _);

            result.extend((2..count * 2).step_by(2).map(|i| i as i64));
            result.push((final_sum - (count - 1) * count) as _);

            result
        } else {
            Vec::new()
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_even_split(final_sum: i64) -> Vec<i64> {
        Self::maximum_even_split(final_sum)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
