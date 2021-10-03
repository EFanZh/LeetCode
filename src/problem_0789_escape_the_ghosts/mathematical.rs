pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let [target_x, target_y]: [i32; 2] = target.as_slice().try_into().unwrap();
        let my_distance = target_x.abs() + target_y.abs();

        ghosts.iter().all(|ghost| {
            let [ghost_x, ghost_y]: [i32; 2] = ghost.as_slice().try_into().unwrap();
            let ghost_distance = (target_x - ghost_x).abs() + (target_y - ghost_y).abs();

            ghost_distance > my_distance
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        Self::escape_ghosts(ghosts, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
