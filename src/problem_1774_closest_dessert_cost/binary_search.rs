pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::slice::Iter;

impl Solution {
    fn generate_sums_helper(mut iter: Iter<i32>, mut sum: i32, target: &mut Vec<u32>) {
        while let Some(&value) = iter.next() {
            Self::generate_sums_helper(iter.clone(), sum, target);

            sum += value;

            Self::generate_sums_helper(iter.clone(), sum, target);

            sum += value;
        }

        target.push(sum as _);
    }

    fn generate_sums(values: &[i32]) -> Box<[u32]> {
        let mut result = Vec::with_capacity(1 << values.len());

        Self::generate_sums_helper(values.iter(), 0, &mut result);

        result.into_boxed_slice()
    }

    pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        let target = target as u32;

        // Note: a better splitting point could be used.
        let (left, right) = topping_costs.split_at(topping_costs.len() / 2);

        let left_topping_costs = Self::generate_sums(left);
        let mut right_topping_costs = Self::generate_sums(right);

        right_topping_costs.sort_unstable();

        let mut result = u32::MAX;

        for base_cost in base_costs {
            for &left_cost in &*left_topping_costs {
                let current_cost = base_cost as u32 + left_cost;

                if let Some(expected_right_cost) = target.checked_sub(current_cost) {
                    if expected_right_cost == 0 {
                        return target as _;
                    }

                    let i = right_topping_costs.partition_point(|&x| x < expected_right_cost);

                    let smaller = right_topping_costs
                        .get(i.wrapping_sub(1))
                        .map_or(current_cost, |&right_cost| current_cost + right_cost);

                    if target - smaller <= result.abs_diff(target) {
                        result = smaller;
                    }

                    if let Some(&right_cost) = right_topping_costs.get(i) {
                        let greater = current_cost + right_cost;

                        if greater - target < result.abs_diff(target) {
                            result = greater;
                        }
                    }
                } else if current_cost - target < result.abs_diff(target) {
                    result = current_cost;
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        Self::closest_cost(base_costs, topping_costs, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
