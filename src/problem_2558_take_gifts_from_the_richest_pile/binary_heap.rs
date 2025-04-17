pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::BinaryHeap;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut gifts = gifts.into_iter().map(|x| x as u32).collect::<BinaryHeap<_>>();

        for _ in 0..k as u32 {
            let count = &mut *gifts.peek_mut().unwrap();

            *count = count.isqrt();
        }

        gifts.iter().fold(0, |sum, &x| sum + u64::from(x)) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        Self::pick_gifts(gifts, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
