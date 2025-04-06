pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn possible_to_stamp(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
        let columns = grid.first().map_or(0, Vec::len);
        let rows = grid.len();
        let stamp_height = stamp_height as u32;
        let stamp_width = stamp_width as u32 as usize;

        if rows < stamp_height as usize || columns < stamp_width {
            return grid.iter().all(|row| row.iter().all(|&x| x != 0));
        }

        let mut states = vec![(0_u32, 0_u32); columns].into_boxed_slice();
        let mut unblocked_window = VecDeque::with_capacity(stamp_width);

        for row in grid {
            let mut left_unstamped = 0;
            let columns = states.len().min(row.len());

            for i in 0..stamp_width - 1 {
                let (mut top_unblocked, mut top_unstamped) = states[i];

                if row[i] == 0 {
                    left_unstamped += 1;
                    top_unblocked += 1;
                    top_unstamped += 1;
                } else {
                    if top_unstamped != 0 {
                        return false;
                    }

                    left_unstamped = 0;
                    top_unblocked = 0;
                    top_unstamped = 0;
                }

                // Sliding window minimum for unblocked columns.

                while let Some(&back) = unblocked_window.back() {
                    if top_unblocked < back {
                        unblocked_window.pop_back();
                    } else {
                        break;
                    }
                }

                unblocked_window.push_back(top_unblocked);

                states[i] = (top_unblocked, top_unstamped);
            }

            for i in stamp_width - 1..columns {
                let (mut top_unblocked, mut top_unstamped) = states[i];

                if row[i] == 0 {
                    left_unstamped += 1;
                    top_unblocked += 1;
                    top_unstamped += 1;
                } else {
                    if top_unstamped != 0 {
                        return false;
                    }

                    left_unstamped = 0;
                    top_unblocked = 0;
                    top_unstamped = 0;
                }

                // Sliding window minimum for unblocked columns.

                while let Some(&back) = unblocked_window.back() {
                    if top_unblocked < back {
                        unblocked_window.pop_back();
                    } else {
                        break;
                    }
                }

                unblocked_window.push_back(top_unblocked);

                // Check if we can stamp.

                let min_unblocked = *unblocked_window.front().unwrap();
                let front_index = i + 1 - stamp_width;

                if min_unblocked < stamp_height {
                    // Cannot stamp.

                    if states[front_index].1 == stamp_height {
                        return false;
                    }
                } else {
                    // Can stamp.

                    for j in i + 1 - left_unstamped.min(stamp_width)..=i {
                        states[j].1 = 0;
                    }

                    left_unstamped = 0;
                    top_unstamped = 0;
                }

                states[i] = (top_unblocked, top_unstamped);

                if min_unblocked == states[front_index].0 {
                    unblocked_window.pop_front();
                }
            }

            for i in columns - stamp_width + 1..columns {
                if states[i].1 == stamp_height {
                    return false;
                }
            }

            unblocked_window.clear();
        }

        states.iter().all(|&(_, unstamped)| unstamped == 0)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn possible_to_stamp(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
        Self::possible_to_stamp(grid, stamp_height, stamp_width)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
