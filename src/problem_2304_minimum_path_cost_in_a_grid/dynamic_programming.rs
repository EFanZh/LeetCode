pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
        let columns = grid.first().map_or(0, Vec::len);
        let mut iter = grid.into_iter();
        let mut prev_row = iter.next().unwrap();
        let mut buffer = vec![0_u32; columns * 2].into_boxed_slice();
        let (mut cache, mut temp) = buffer.split_at_mut(columns);

        cache
            .iter_mut()
            .zip(&prev_row)
            .for_each(|(target, &id)| *target = id as u32);

        temp.fill(u32::MAX);

        for row in iter {
            cache.iter().zip(prev_row).for_each(|(&cost, id)| {
                temp.iter_mut()
                    .zip(&move_cost[id as u32 as usize])
                    .for_each(|(target, &move_cost)| *target = (*target).min(cost + move_cost as u32));
            });

            temp.iter_mut().zip(&row).for_each(|(target, &id)| *target += id as u32);

            prev_row = row;
            mem::swap(&mut cache, &mut temp);

            temp.fill(u32::MAX);
        }

        cache.iter().fold(u32::MAX, |min_cost, &cost| min_cost.min(cost)) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
        Self::min_path_cost(grid, move_cost)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
