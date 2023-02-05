pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

struct Queue {
    front: (u32, i32),
    rest: VecDeque<(u32, i32)>,
}

impl Queue {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            front: (0, i32::MIN),
            rest: VecDeque::with_capacity(capacity),
        }
    }

    fn push_back(&mut self, item: (u32, i32)) {
        let Self { front, rest } = self;

        loop {
            if let Some(back) = rest.back() {
                if back.1 <= item.1 {
                    rest.pop_back();
                } else {
                    break;
                }
            } else {
                if front.1 <= item.1 {
                    *front = item;

                    return;
                }

                break;
            }
        }

        self.rest.push_back(item);
    }

    fn slide(&mut self, old_index: u32, item: (u32, i32)) {
        if self.front.0 == old_index {
            if let Some(new_front) = self.rest.pop_front() {
                self.front = new_front;
            } else {
                self.front = item;

                return;
            }
        }

        self.push_back(item);
    }
}

impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as u32;
        let (left, right) = nums.split_at(k as usize);
        let mut queue = Queue::with_capacity(left.len() - 1);
        let mut result = i32::MIN;

        for (i, &num) in (0..).zip(left) {
            queue.push_back((i, queue.front.1.max(0) + num));
            result = result.max(queue.front.1);
        }

        for (old_index, &num) in (0..).zip(right) {
            queue.slide(old_index, (old_index + k, queue.front.1.max(0) + num));
            result = result.max(queue.front.1);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        Self::constrained_subset_sum(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
