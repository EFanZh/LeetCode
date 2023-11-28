pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU32;

impl Solution {
    pub fn add_rungs(rungs: Vec<i32>, dist: i32) -> i32 {
        let dist = NonZeroU32::new(dist as _).unwrap();
        let mut result = 0;
        let mut prev = 0;

        for height in rungs {
            let diff = (height - prev) as u32;

            result += (diff - 1) / dist;

            prev = height;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn add_rungs(rungs: Vec<i32>, dist: i32) -> i32 {
        Self::add_rungs(rungs, dist)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
