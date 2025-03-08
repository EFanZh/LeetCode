pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn interval_overlaps(left: (u32, u32), right: (u32, u32)) -> bool {
        left.0 <= right.1 && left.1 >= right.0
    }

    pub fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut candy_sums = candies_count;
        let mut sum = 0;

        for target in &mut candy_sums {
            sum += *target;
            *target = sum;
        }

        queries
            .into_iter()
            .map(|query| {
                let [favorite_type, favorite_day, daily_cap] = query.try_into().ok().unwrap();
                let favorite_type = favorite_type as u32 as usize;
                let favorite_day = favorite_day as u32 as usize;
                let daily_cap = daily_cap as u32;
                let left_sum = candy_sums.get(favorite_type.wrapping_sub(1)).copied().unwrap_or(0) as u32;

                Self::interval_overlaps(
                    (
                        left_sum.saturating_sub(daily_cap - 1),
                        candy_sums[favorite_type] as u32 - 1,
                    ),
                    (favorite_day as _, daily_cap.saturating_mul(favorite_day as _)),
                )
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        Self::can_eat(candies_count, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
