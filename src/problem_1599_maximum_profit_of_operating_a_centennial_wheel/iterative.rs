pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_operations_max_profit(customers: Vec<i32>, boarding_cost: i32, running_cost: i32) -> i32 {
        let n = customers.len() as i32;
        let mut profit = 0;
        let mut max_profit = i32::MIN;
        let mut result = 0;
        let mut waiting = 0;

        for (i, customer) in (1..).zip(customers) {
            waiting += customer;

            let onboard = waiting.min(4);

            profit += boarding_cost * onboard;
            waiting -= onboard;
            profit -= running_cost;

            if profit > max_profit {
                max_profit = profit;
                result = i;
            }
        }

        if (boarding_cost << 2) > running_cost {
            let full_extra_turns = waiting >> 2;

            profit += ((boarding_cost << 2) - running_cost) * full_extra_turns;
            waiting &= 3;

            if profit > max_profit {
                max_profit = profit;
                result = n + full_extra_turns;
            }

            if boarding_cost * waiting > running_cost {
                profit += boarding_cost * waiting - running_cost;

                if profit > max_profit {
                    max_profit = profit;
                    result = n + full_extra_turns + 1;
                }
            }
        }

        if max_profit > 0 { result } else { -1 }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations_max_profit(customers: Vec<i32>, boarding_cost: i32, running_cost: i32) -> i32 {
        Self::min_operations_max_profit(customers, boarding_cost, running_cost)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
