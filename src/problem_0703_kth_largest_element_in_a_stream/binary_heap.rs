// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, mut nums: Vec<i32>) -> Self {
        let k = k as usize;

        // TODO: use `select_nth_unstable`.

        if nums.len() == k - 1 {
            nums.push(i32::MIN);
        }

        let (left, right) = nums.split_at(k);
        let mut heap = left.iter().copied().map(Reverse).collect::<BinaryHeap<_>>();

        for &num in right {
            let mut top = heap.peek_mut().unwrap();

            if num > top.0 {
                top.0 = num;
            }
        }

        Self { heap }
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
