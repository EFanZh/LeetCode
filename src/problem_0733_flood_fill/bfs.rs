pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::mem;

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let mut image = image;
        let mut y = sr as usize;
        let mut x = sc as usize;
        let old_color = mem::replace(&mut image[y][x], new_color);

        if old_color != new_color {
            let mut queue = VecDeque::new();

            loop {
                for &(next_y, next_x) in &[(y.wrapping_sub(1), x), (y, x.wrapping_sub(1)), (y, x + 1), (y + 1, x)] {
                    if let Some(cell) = image.get_mut(next_y).and_then(|row| row.get_mut(next_x)) {
                        if *cell == old_color {
                            *cell = new_color;

                            queue.push_back((next_y, next_x));
                        }
                    }
                }

                if let Some((next_y, next_x)) = queue.pop_front() {
                    y = next_y;
                    x = next_x;
                } else {
                    break;
                }
            }
        }

        image
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        Self::flood_fill(image, sr, sc, new_color)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
