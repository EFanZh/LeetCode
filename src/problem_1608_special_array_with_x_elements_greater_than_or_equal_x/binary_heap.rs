pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;

impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(nums);
        let mut i = 0;

        while let Some(top) = heap.pop() {
            match top.cmp(&i) {
                Ordering::Less => break,
                Ordering::Equal => {
                    i = -1;

                    break;
                }
                Ordering::Greater => {}
            }

            i += 1;
        }

        i
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn special_array(nums: Vec<i32>) -> i32 {
        Self::special_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
