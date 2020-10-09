pub struct Solution;

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let columns = dungeon.first().map_or(0, Vec::len);
        let mut cache = vec![i32::max_value(); columns];
        let (last_row, rest_rows) = dungeon.split_last().unwrap();
        let (last_cell, rest_cells) = last_row.split_last().unwrap();
        let mut prev = (1 - last_cell).max(1);

        *cache.last_mut().unwrap() = prev;

        for (cell, target) in rest_cells.iter().zip(&mut cache).rev() {
            prev = (prev - *cell).max(1);
            *target = prev;
        }

        for row in rest_rows.iter().rev() {
            for (i, cell) in row.iter().enumerate().rev() {
                let cost = cache.get(i + 1).map_or_else(|| cache[i], |&right| right.min(cache[i]));

                cache[i] = (cost - *cell).max(1);
            }
        }

        cache[0]
    }
}

impl super::Solution for Solution {
    fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        Self::calculate_minimum_hp(dungeon)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
