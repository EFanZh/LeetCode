pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let grid = grid.as_slice();

        (0..grid.first().map_or(0, Vec::len))
            .map(|mut location| {
                for row in grid {
                    let row = row.as_slice();
                    let direction = row[location];
                    let next_location = location.wrapping_add(direction as _);

                    if row.get(next_location).copied() == Some(direction) {
                        location = next_location;
                    } else {
                        return -1;
                    }
                }

                location as _
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        Self::find_ball(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
