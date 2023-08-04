pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::VecDeque;

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut is_water = is_water;
        let mut queue = VecDeque::new();

        for (y, row) in (0_u16..).zip(&mut is_water) {
            for (x, state) in (0_u16..).zip(row) {
                if *state != 0 {
                    *state = -1;

                    queue.push_back((y, x));
                }
            }
        }

        let mut distance = -2;

        loop {
            for _ in 0..queue.len() {
                let (y, x) = queue.pop_front().unwrap();

                for neighbor in [(y.wrapping_sub(1), x), (y, x.wrapping_sub(1)), (y, x + 1), (y + 1, x)] {
                    if let Some(state) = is_water
                        .get_mut(usize::from(neighbor.0))
                        .and_then(|row| row.get_mut(usize::from(neighbor.1)))
                    {
                        *state = match (*state).cmp(&0) {
                            Ordering::Less => continue,
                            Ordering::Equal => {
                                queue.push_back(neighbor);

                                distance
                            }
                            Ordering::Greater => -1,
                        };
                    }
                }
            }

            if queue.is_empty() {
                break;
            }

            distance -= 1;
        }

        for row in &mut is_water {
            for height in row {
                *height = -1 - *height;
            }
        }

        is_water
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::highest_peak(is_water)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
