pub struct Solution {}

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut cache = grid.last().unwrap().clone();

        {
            let mut prev = 0;

            for cache_item in cache.iter_mut().rev() {
                *cache_item += prev;
                prev = *cache_item;
            }
        }

        for row in grid.into_iter().rev().skip(1) {
            let mut prev = i32::max_value();

            for (cell, cache_item) in row.into_iter().zip(cache.iter_mut()).rev() {
                *cache_item = cell + (*cache_item).min(prev);
                prev = *cache_item;
            }
        }

        cache[0]
    }
}

impl super::Solution for Solution {
    fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        Self::min_path_sum(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
