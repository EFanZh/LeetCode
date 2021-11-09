pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
        let mut not_good = HashSet::new();

        for (&x, &y) in fronts.iter().zip(&backs) {
            if x == y {
                not_good.insert(x);
            }
        }

        let mut result = i32::MAX;

        for &nums in &[fronts.as_slice(), backs.as_slice()] {
            for &x in nums {
                if !not_good.contains(&x) {
                    result = result.min(x);
                }
            }
        }

        if result == i32::MAX {
            0
        } else {
            result
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
        Self::flipgame(fronts, backs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
