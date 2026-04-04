pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_costs(cost: Vec<i32>) -> Vec<i32> {
        let mut cost = cost;
        let mut min = i32::MAX;

        for cost in &mut cost {
            if *cost < min {
                min = *cost;
            } else {
                *cost = min;
            }
        }

        cost
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_costs(cost: Vec<i32>) -> Vec<i32> {
        Self::min_costs(cost)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
