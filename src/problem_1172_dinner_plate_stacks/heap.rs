// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::convert;

#[derive(Default)]
struct Heap {
    heap: Vec<usize>,
    indices: Vec<usize>,
}

impl Heap {
    fn partial_sift_up_by_key<T>(&mut self, mut index: usize, key: &T, mut key_fn: impl FnMut(usize) -> T) -> usize
    where
        T: Ord,
    {
        loop {
            let parent_index = index.wrapping_sub(1) / 2;

            if let Some(&parent_node) = self.heap.get(parent_index) {
                if key > &key_fn(parent_node) {
                    self.heap[index] = parent_node;
                    self.indices[parent_node] = index;
                    index = parent_index;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        index
    }

    fn push_by_key<T>(&mut self, value: usize, mut key_fn: impl FnMut(usize) -> T)
    where
        T: Ord,
    {
        let mut index = self.heap.len();

        self.heap.push(0); // The value here does not matter.

        index = self.partial_sift_up_by_key(index, &key_fn(value), key_fn);

        self.heap[index] = value;

        let target = if let Some(target) = self.indices.get_mut(value) {
            target
        } else {
            self.indices.resize(value + 1, 0);
            self.indices.last_mut().unwrap()
        };

        *target = index;
    }

    fn sift_down_by_key<T>(&mut self, mut index: usize, value: usize, key: &T, mut key_fn: impl FnMut(usize) -> T)
    where
        T: Ord,
    {
        loop {
            let mut child_index = index * 2 + 1;

            if let Some(mut child_node) = self.heap.get(child_index).copied() {
                let mut child_key = key_fn(child_node);
                let right_index = child_index + 1;

                if let Some(&right_node) = self.heap.get(right_index) {
                    let right_key = key_fn(right_node);

                    if right_key > child_key {
                        child_index = right_index;
                        child_node = right_node;
                        child_key = right_key;
                    }
                }

                if child_key > *key {
                    self.heap[index] = child_node;
                    self.indices[child_node] = index;
                    index = child_index;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        self.heap[index] = value;
        self.indices[value] = index;
    }

    fn remove<T>(&mut self, value: usize, mut key_fn: impl FnMut(usize) -> T + Copy)
    where
        T: Ord,
    {
        let index = self.indices[value];
        let last = self.heap.pop().unwrap();

        if index != self.heap.len() {
            let key = key_fn(last);
            let index = self.partial_sift_up_by_key(index, &key, key_fn);

            self.sift_down_by_key(index, last, &key, key_fn);
        }
    }

    fn peek(&self) -> Option<usize> {
        self.heap.first().copied()
    }
}

pub struct DinnerPlates {
    stacks: Vec<Vec<i32>>,
    non_empty_heap: Heap,
    non_full_heap: Heap,
    capacity: usize,
}

impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        Self {
            stacks: Vec::new(),
            non_empty_heap: Heap::default(),
            non_full_heap: Heap::default(),
            capacity: capacity as _,
        }
    }

    fn push(&mut self, val: i32) {
        if let Some(index) = self.non_full_heap.peek() {
            let slot = &mut self.stacks[index];

            slot.push(val);

            if slot.len() == 1 {
                self.non_empty_heap.push_by_key(index, convert::identity);
            }

            if slot.len() == self.capacity {
                self.non_full_heap.remove(index, Reverse);
            }
        } else {
            let index = self.stacks.len();
            let mut stack = Vec::with_capacity(self.capacity);

            stack.push(val);
            self.stacks.push(stack);

            self.non_empty_heap.push_by_key(index, convert::identity);

            if self.capacity != 1 {
                self.non_full_heap.push_by_key(index, Reverse);
            }
        }
    }

    fn pop_at(&mut self, index: usize) -> Option<i32> {
        let stack = self.stacks.get_mut(index)?;
        let result = stack.pop()?;

        if stack.is_empty() {
            self.non_empty_heap.remove(index, convert::identity);
        }

        if stack.len() + 1 == self.capacity {
            self.non_full_heap.push_by_key(index, Reverse);
        }

        Some(result)
    }

    fn pop(&mut self) -> i32 {
        self.non_empty_heap
            .peek()
            .map_or(-1, |index| self.pop_at(index).unwrap())
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        self.pop_at(index as _).unwrap_or(-1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::DinnerPlates for DinnerPlates {
    fn new(capacity: i32) -> Self {
        Self::new(capacity)
    }

    fn push(&mut self, val: i32) {
        self.push(val);
    }

    fn pop(&mut self) -> i32 {
        self.pop()
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        self.pop_at_stack(index)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::DinnerPlates>();
    }
}
