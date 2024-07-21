// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

pub struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as u32 as usize;
        let mut nums = nums;

        match nums.len().cmp(&k) {
            Ordering::Less => nums.push(i32::MIN),
            Ordering::Equal => {}
            Ordering::Greater => {
                nums.select_nth_unstable_by_key(k - 1, |&x| Reverse(x));
                nums.truncate(k);
            }
        }

        Self {
            heap: nums.into_iter().map(Reverse).collect(),
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        let heap = &mut self.heap;
        let mut top = heap.peek_mut().unwrap();

        if val > top.0 {
            top.0 = val;
        }

        drop(top);

        heap.peek().unwrap().0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::KthLargest for KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        Self::new(k, nums)
    }

    fn add(&mut self, val: i32) -> i32 {
        self.add(val)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::KthLargest>();
    }
}
