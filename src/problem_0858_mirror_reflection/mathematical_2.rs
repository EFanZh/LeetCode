pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        match p.trailing_zeros().cmp(&q.trailing_zeros()) {
            Ordering::Less => 0,
            Ordering::Equal => 1,
            Ordering::Greater => 2,
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn mirror_reflection(p: i32, q: i32) -> i32 {
        Self::mirror_reflection(p, q)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
