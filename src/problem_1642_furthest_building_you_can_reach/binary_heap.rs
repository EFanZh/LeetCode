pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut bricks = bricks;
        let ladders = ladders as u32 as usize;
        let mut ladder_candidates = BinaryHeap::new();
        let mut prev = i32::MAX;
        let mut i = -1;

        for height in heights {
            if height > prev {
                ladder_candidates.push(Reverse(height - prev));

                if ladder_candidates.len() > ladders {
                    let Reverse(min_diff) = ladder_candidates.pop().unwrap();

                    if min_diff > bricks {
                        return i;
                    }

                    bricks -= min_diff;
                }
            }

            prev = height;
            i += 1;
        }

        i
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        Self::furthest_building(heights, bricks, ladders)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
