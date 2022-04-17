pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    fn update_min_cost(cache: &[(u32, u32)], min_day: u32, start_index: &mut usize, prev_cost: &mut u32) {
        while let Some(&(day, cost)) = cache.get(*start_index) {
            if day >= min_day {
                *start_index += 1;
                *prev_cost = cost;
            } else {
                break;
            }
        }
    }

    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let [price_1, price_7, price_30]: [_; 3] = costs.try_into().unwrap();
        let (price_1, price_7, price_30) = (price_1 as u32, price_7 as u32, price_30 as u32);
        let mut cache = Vec::new();
        let mut start_30 = 0;
        let mut cost_30 = 0;
        let mut start_7 = 0;
        let mut cost_7 = 0;
        let mut start_1 = 0;
        let mut cost_1 = 0;

        for day in days.into_iter().rev() {
            let day = day as _;

            Self::update_min_cost(&cache, day + 30, &mut start_30, &mut cost_30);
            Self::update_min_cost(&cache, day + 7, &mut start_7, &mut cost_7);
            Self::update_min_cost(&cache, day + 1, &mut start_1, &mut cost_1);

            cache.push((day, (cost_30 + price_30).min(cost_7 + price_7).min(cost_1 + price_1)));
        }

        cache.pop().map(|(_, result)| result as _).unwrap_or_default()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        Self::mincost_tickets(days, costs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
