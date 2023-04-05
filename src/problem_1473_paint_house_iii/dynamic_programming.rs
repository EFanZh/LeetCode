pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

const INVALID: u32 = u32::MAX / 2;

impl Solution {
    fn get_min_costs(costs: &[u32]) -> (u32, u32) {
        let mut result = (INVALID, INVALID);

        for &cost in costs {
            if cost < result.0 {
                result.1 = result.0;
                result.0 = cost;
            } else if cost < result.1 {
                result.1 = cost;
            }
        }

        result
    }

    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let _ = (m, n);
        let n = cost.first().map_or(0, Vec::len);
        let target = target as u32 as usize;
        let mut cache = vec![INVALID; n * target].into_boxed_slice();

        let mut iter = houses.into_iter().map(|house| house as u32 as usize).zip(cost);

        {
            let (house, house_costs) = iter.next().unwrap();

            if house == 0 {
                for (slot, cost) in cache.iter_mut().zip(house_costs) {
                    *slot = cost as _;
                }
            } else {
                cache[house - 1] = 0;
            }
        }

        for (house, house_costs) in iter {
            if house == 0 {
                // `neighborhoods > 1`.

                for neighborhoods in (2..=target).rev() {
                    let (top_row, row) = cache[n * (neighborhoods - 2)..].split_at_mut(n);
                    let (top_min_cost_1, top_min_cost_2) = Self::get_min_costs(top_row);

                    for ((slot, &mut top_cost), &cost) in row.iter_mut().zip(top_row).zip(&house_costs) {
                        *slot = (*slot).min(if top_cost == top_min_cost_1 {
                            top_min_cost_2
                        } else {
                            top_min_cost_1
                        }) + cost as u32;
                    }
                }

                // `neighborhoods == 1`.

                for (slot, &cost) in cache.iter_mut().zip(&house_costs) {
                    *slot = slot.saturating_add(cost as _);
                }
            } else {
                // `neighborhoods > 1`.

                for neighborhoods in (2..=target).rev() {
                    let (top_row, row) = cache[n * (neighborhoods - 2)..].split_at_mut(n);

                    row[..house - 1].fill(INVALID);

                    let slot = &mut row[house - 1];
                    let mut min_top_cost = INVALID;

                    for &cost in &top_row[..house - 1] {
                        min_top_cost = min_top_cost.min(cost);
                    }

                    for &cost in &top_row[house..] {
                        min_top_cost = min_top_cost.min(cost);
                    }

                    *slot = (*slot).min(min_top_cost);

                    row[house..n].fill(INVALID);
                }

                // `neighborhoods == 1`.

                cache[..house - 1].fill(INVALID);
                cache[house..n].fill(INVALID);
            }
        }

        let candidate = cache[cache.len() - n..].iter().copied().min().unwrap();

        if candidate < INVALID {
            candidate as _
        } else {
            -1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        Self::min_cost(houses, cost, m, n, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
