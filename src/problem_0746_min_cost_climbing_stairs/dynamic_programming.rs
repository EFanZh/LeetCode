pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        assert!(cost.len() > 1);

        let mut cost_1 = cost[0];
        let mut cost_2 = cost[1];

        for &value in &cost[2..] {
            let new_cost_2 = cost_1.min(cost_2) + value;

            cost_1 = cost_2;
            cost_2 = new_cost_2;
        }

        cost_1.min(cost_2)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        Self::min_cost_climbing_stairs(cost)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
