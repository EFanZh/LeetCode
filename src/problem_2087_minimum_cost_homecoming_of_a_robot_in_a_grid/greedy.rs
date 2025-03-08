pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn parse_point(point: Vec<i32>) -> (u32, u32) {
        let [y, x] = point.try_into().ok().unwrap();

        (y as _, x as _)
    }

    pub fn min_cost(start_pos: Vec<i32>, home_pos: Vec<i32>, row_costs: Vec<i32>, col_costs: Vec<i32>) -> i32 {
        let (start_y, start_x) = Self::parse_point(start_pos);
        let (home_y, home_x) = Self::parse_point(home_pos);

        let (x_0, x_1, y_0, y_1) = if home_y < start_y {
            if home_x < start_x {
                (home_x, start_x, home_y, start_y)
            } else {
                (start_x + 1, home_x + 1, home_y, start_y)
            }
        } else if home_x < start_x {
            (home_x, start_x, start_y + 1, home_y + 1)
        } else {
            (start_x + 1, home_x + 1, start_y + 1, home_y + 1)
        };

        let fold_fn = |sum, x| sum + x;
        let column_cost = col_costs[x_0 as usize..x_1 as usize].iter().fold(0, fold_fn);

        row_costs[y_0 as usize..y_1 as usize].iter().fold(column_cost, fold_fn)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_cost(start_pos: Vec<i32>, home_pos: Vec<i32>, row_costs: Vec<i32>, col_costs: Vec<i32>) -> i32 {
        Self::min_cost(start_pos, home_pos, row_costs, col_costs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
