pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::mem;

impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut prev = i32::MIN;
        let mut queue_1 = BinaryHeap::new();
        let mut queue_2 = BinaryHeap::new();

        for num in nums {
            match num.wrapping_sub(prev) {
                0 => queue_2.push(Reverse(queue_1.pop().map_or(1, |Reverse(length)| length + 1))),
                1 => {
                    if queue_1.drain().any(|Reverse(length)| length < 3) {
                        return false;
                    }

                    queue_1.push(Reverse(queue_2.pop().map_or(1_u16, |Reverse(length)| length + 1)));

                    mem::swap(&mut queue_1, &mut queue_2);

                    prev = num;
                }
                _ => {
                    if queue_1.drain().any(|Reverse(length)| length < 3)
                        || queue_2.drain().any(|Reverse(length)| length < 3)
                    {
                        return false;
                    }

                    queue_2.push(Reverse(1));

                    prev = num;
                }
            }
        }

        queue_1.iter().all(|&Reverse(length)| length >= 3) && queue_2.iter().all(|&Reverse(length)| length >= 3)
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
