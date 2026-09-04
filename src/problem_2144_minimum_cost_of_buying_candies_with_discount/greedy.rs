pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_cost(cost: Vec<i32>) -> i32 {
        let mut cost = cost;

        cost.sort_unstable_by(|lhs, rhs| rhs.cmp(lhs));

        let mut result = 0;
        let (chunks, remainder) = cost.as_chunks::<3>();

        for chunk in chunks {
            result += chunk[0] + chunk[1];
        }

        result += remainder.iter().sum::<i32>();

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_cost(cost: Vec<i32>) -> i32 {
        Self::minimum_cost(cost)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
