pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

#[derive(Default)]
struct OptimizedBinaryHeap {
    one: u16,
    two: u16,
    three: u16,
}

impl OptimizedBinaryHeap {
    fn push(&mut self, length: u16) {
        match length {
            1 => self.one += 1,
            2 => self.two += 1,
            _ => self.three += 1,
        }
    }

    fn pop(&mut self) -> Option<u16> {
        if self.one == 0 {
            if self.two == 0 {
                if self.three == 0 {
                    None
                } else {
                    self.three -= 1;

                    Some(3)
                }
            } else {
                self.two -= 1;

                Some(2)
            }
        } else {
            self.one -= 1;

            Some(1)
        }
    }

    fn clear(&mut self) {
        self.one = 0;
        self.two = 0;
        self.three = 0;
    }

    fn has_length_less_than_3(&self) -> bool {
        self.one != 0 || self.two != 0
    }
}

impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut prev = i32::MIN;
        let mut queue_1 = OptimizedBinaryHeap::default();
        let mut queue_2 = OptimizedBinaryHeap::default();

        for num in nums {
            match num.wrapping_sub(prev) {
                0 => queue_2.push(queue_1.pop().map_or(1, |length| length + 1)),
                1 => {
                    if queue_1.has_length_less_than_3() {
                        return false;
                    }

                    queue_1.clear();
                    queue_1.push(queue_2.pop().map_or(1, |length| length + 1));

                    mem::swap(&mut queue_1, &mut queue_2);

                    prev = num;
                }
                _ => {
                    if queue_1.has_length_less_than_3() || queue_2.has_length_less_than_3() {
                        return false;
                    }

                    queue_1.clear();
                    queue_2.clear();

                    queue_2.push(1);

                    prev = num;
                }
            }
        }

        !queue_1.has_length_less_than_3() && !queue_2.has_length_less_than_3()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_possible(nums: Vec<i32>) -> bool {
        Self::is_possible(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
