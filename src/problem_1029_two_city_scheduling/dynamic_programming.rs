pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryFrom;

impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let n = costs.len() / 2;
        let mut cache = vec![u32::MAX / 2; n + 1];

        cache[0] = 0;

        for (max_count, cost) in (2..).zip(&costs) {
            let [cost_a, cost_b] = <[_; 2]>::try_from(cost.as_slice()).unwrap();
            let (cost_a, cost_b) = (cost_a as u32, cost_b as u32);
            let count = max_count.min(n + 1);

            let mut iter = cache[..count].iter_mut().rev();
            let mut next = iter.next().unwrap();

            for current in iter {
                *next = (*current + cost_a).min(*next + cost_b);
                next = current;
            }

            *next += cost_b;
        }

        *cache.last().unwrap() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        Self::two_city_sched_cost(costs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
