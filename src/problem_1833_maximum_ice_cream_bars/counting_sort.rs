pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let iter_costs = || costs.iter().map(|&cost| cost as u32);
        let min = iter_costs().min().unwrap();

        assert_ne!(min, 0);

        let max = iter_costs().max().unwrap();
        let mut counts = vec![0; (max - min) as usize + 1].into_boxed_slice();

        for cost in costs {
            counts[(cost as u32 - min) as usize] += 1;
        }

        let mut coins = coins as u32;
        let mut result = 0;

        for (cost, &count) in (min..u32::MAX).zip(&*counts) {
            let total_cost = cost * count;

            if total_cost < coins {
                coins -= total_cost;
                result += count;
            } else {
                result += coins / cost;

                break;
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        Self::max_ice_cream(costs, coins)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
