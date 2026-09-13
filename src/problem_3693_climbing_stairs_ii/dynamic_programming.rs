pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        _ = n;

        costs
            .into_iter()
            .fold((u32::MAX / 2, u32::MAX / 2, 0), |(prev_1, prev_2, prev_3), cost| {
                (
                    prev_2,
                    prev_3,
                    (prev_1 + 9).min(prev_2 + 4).min(prev_3 + 1) + cost.cast_unsigned(),
                )
            })
            .2
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        Self::climb_stairs(n, costs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
