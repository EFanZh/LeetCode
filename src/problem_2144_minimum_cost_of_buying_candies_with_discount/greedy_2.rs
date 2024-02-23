pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_cost(cost: Vec<i32>) -> i32 {
        let mut cost = cost;

        cost.sort_unstable_by(|lhs, rhs| rhs.cmp(lhs));

        let mut result = cost.iter().sum::<i32>();

        if let Some(cost) = cost.get(2..) {
            for num in cost.iter().step_by(3) {
                result -= num;
            }
        }

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
