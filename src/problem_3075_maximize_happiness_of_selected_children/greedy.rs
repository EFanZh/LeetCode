pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

// TODO: Use quick select.

impl Solution {
    pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        let mut happiness = happiness.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let n = happiness.len();
        let k = k.cast_unsigned() as usize;

        happiness.select_nth_unstable(n - k);

        let selected = &mut happiness[n - k..];

        selected.sort_unstable();

        selected
            .iter()
            .rev()
            .zip(0..selected.len() as u32)
            .map_while(|(x, y)| x.checked_sub(y))
            .fold(0_u64, |sum, value| sum + u64::from(value))
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        Self::maximum_happiness_sum(happiness, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
