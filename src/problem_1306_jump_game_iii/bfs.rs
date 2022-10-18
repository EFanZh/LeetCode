pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::mem;

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut arr = arr;
        let mut state = (start, mem::replace(&mut arr[start as usize], -1));
        let mut queue = VecDeque::new();

        if state.1 == 0 {
            true
        } else {
            loop {
                for next in [state.0 - state.1, state.0 + state.1] {
                    match arr.get_mut(next as usize) {
                        None | Some(-1) => {}
                        Some(0) => return true,
                        Some(next_jump) => queue.push_back((next, mem::replace(next_jump, -1))),
                    }
                }

                if let Some(next_state) = queue.pop_front() {
                    state = next_state;
                } else {
                    break;
                }
            }

            false
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        Self::can_reach(arr, start)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
