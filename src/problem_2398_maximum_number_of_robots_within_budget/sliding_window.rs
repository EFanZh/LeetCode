pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::ptr;

impl Solution {
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        let charge_times = charge_times.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let running_costs = running_costs.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let budget = budget as u64;
        let n = charge_times.len();
        let mut start = 0;
        let mut charge_time_queue = VecDeque::new();
        let mut running_cost_sum = 0;

        (1..)
            .zip(charge_times.iter().zip(&running_costs))
            .for_each(|(end, (charge_time, &running_cost))| {
                while let Some(&back) = charge_time_queue.back() {
                    if back < charge_time {
                        charge_time_queue.pop_back();
                    } else {
                        break;
                    }
                }

                charge_time_queue.push_back(charge_time);
                running_cost_sum += u64::from(running_cost);

                let max_charge_time = *charge_time_queue.front().unwrap();

                if u64::from(*max_charge_time) + (end - start) as u64 * running_cost_sum > budget {
                    if ptr::eq(max_charge_time, &raw const charge_times[start]) {
                        charge_time_queue.pop_front();
                    }

                    running_cost_sum -= u64::from(running_costs[start]);
                    start += 1;
                }
            });

        (n - start) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        Self::maximum_robots(charge_times, running_costs, budget)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
