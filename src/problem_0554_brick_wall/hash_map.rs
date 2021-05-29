pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let rows = wall.len();
        let mut counts = HashMap::new();

        for row in wall {
            let mut x = 0;

            for brick in row.split_last().unwrap().1 {
                x += brick;

                counts.entry(x).and_modify(|c| *c += 1).or_insert(1);
            }
        }

        (rows - counts.values().copied().max().unwrap_or(0)) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        Self::least_bricks(wall)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
