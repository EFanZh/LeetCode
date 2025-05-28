pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_increments(n: i32, cost: Vec<i32>) -> i32 {
        _ = n;

        let mut cost = cost;

        (0..cost.len() / 2)
            .rev()
            .map(|i| {
                let left = cost[i * 2 + 1];
                let right = cost[i * 2 + 2];

                let (min, max) = if u32::lt(&(right as _), &(left as _)) {
                    (right, left)
                } else {
                    (left, right)
                };

                cost[i] += max;

                max - min
            })
            .sum()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_increments(n: i32, cost: Vec<i32>) -> i32 {
        Self::min_increments(n, cost)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
