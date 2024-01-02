pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::BinaryHeap;
use std::num::NonZeroU32;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let mut sum = target.iter().copied().sum::<i32>() as u32;
        let mut queue = target.into_iter().map(|value| value as u32).collect::<BinaryHeap<_>>();

        loop {
            let mut top = queue.peek_mut().unwrap();
            let old_top = *top;
            let rest = sum - old_top;

            let Some(rest) = NonZeroU32::new(rest).filter(|_| old_top >= rest) else {
                return old_top == 1;
            };

            let new_top = old_top % rest;

            if new_top == 0 {
                return rest.get() == 1;
            }

            sum -= old_top - new_top;
            *top = new_top;
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_possible(target: Vec<i32>) -> bool {
        Self::is_possible(target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
