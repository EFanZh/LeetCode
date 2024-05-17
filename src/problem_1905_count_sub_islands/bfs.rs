pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let mut grid2 = grid2;
        let rows = grid1.len();
        let columns = grid1.first().map_or(0, Vec::len);
        let mut result = 0;
        let mut queue = VecDeque::new();

        for y in 0..rows {
            for x in 0..columns {
                if let state @ 1 = &mut grid2[y][x] {
                    *state = 0;

                    let mut is_sub_island = grid1[y][x] != 0;
                    let mut y = y;
                    let mut x = x;

                    loop {
                        for next in [(y.wrapping_sub(1), x), (y, x.wrapping_sub(1)), (y, x + 1), (y + 1, x)] {
                            if let Some(state @ 1) = grid2.get_mut(next.0).and_then(|row| row.get_mut(next.1)) {
                                *state = 0;

                                is_sub_island &= grid1[next.0][next.1] != 0;

                                queue.push_back((next.0 as u16, next.1 as u16));
                            }
                        }

                        if let Some(next) = queue.pop_front() {
                            y = usize::from(next.0);
                            x = usize::from(next.1);
                        } else {
                            break;
                        }
                    }

                    result += i32::from(is_sub_island);
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        Self::count_sub_islands(grid1, grid2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
