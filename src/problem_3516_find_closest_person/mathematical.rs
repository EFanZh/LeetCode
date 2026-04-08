pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        match x.abs_diff(z).cmp(&y.abs_diff(z)) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => 2,
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        Self::find_closest(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
