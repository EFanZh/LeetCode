pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::mem;

struct State<I> {
    left_iter: I,
    left: u32,
    left_sum: u64,
    right_iter: I,
    right: u32,
    right_sum: u64,
    result: u64,
}

impl<I> State<I>
where
    I: Iterator<Item = u32> + Clone,
{
    fn both_unknown(&mut self) {
        self.left_sum = 0;
        self.right_sum = 0;

        if let Some(left) = self.left_iter.next() {
            self.left = left;
            self.left_sum += u64::from(left);

            self.has_left();
        }
    }

    fn has_left(&mut self) {
        if let Some(right) = self.right_iter.next() {
            self.right = right;
            self.right_sum += u64::from(right);

            self.has_both();
        } else {
            mem::swap(&mut self.left_sum, &mut self.right_sum);
            self.right_iter = self.left_iter.clone();
        }
    }

    fn has_right(&mut self) {
        if let Some(left) = self.left_iter.next() {
            self.left = left;
            self.left_sum += u64::from(left);

            self.has_both();
        }
    }

    fn has_both(&mut self) {
        match self.left.cmp(&self.right) {
            Ordering::Less => self.has_right(),
            Ordering::Equal => {
                self.result += self.left_sum.max(self.right_sum);

                self.both_unknown();
            }
            Ordering::Greater => self.has_left(),
        }
    }
}

impl Solution {
    fn make_iter(nums: &[i32]) -> impl Iterator<Item = u32> + Clone + '_ {
        nums.iter().map(|&num| num as _)
    }

    pub fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut state = State {
            left_iter: Self::make_iter(&nums1),
            left: 0,
            left_sum: 0,
            right_iter: Self::make_iter(&nums2),
            right: 0,
            right_sum: 0,
            result: 0,
        };

        state.both_unknown();

        for num in state.right_iter {
            state.right_sum += u64::from(num);
        }

        ((state.result + state.left_sum.max(state.right_sum)) % 1_000_000_007) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        Self::max_sum(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
