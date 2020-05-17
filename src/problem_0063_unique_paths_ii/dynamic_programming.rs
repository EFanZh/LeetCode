pub struct Solution {}

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let columns = obstacle_grid[0].len();
        let mut cache = vec![0; columns];

        cache[columns - 1] = 1;

        for row in obstacle_grid.into_iter().rev() {
            let mut prev = 0;

            for (cell, cache_item) in row.into_iter().zip(&mut cache).rev() {
                if cell == 0 {
                    *cache_item += prev;
                } else {
                    *cache_item = 0;
                }

                prev = *cache_item;
            }
        }

        cache[0]
    }
}

impl super::Solution for Solution {
    fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        Self::unique_paths_with_obstacles(obstacle_grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
