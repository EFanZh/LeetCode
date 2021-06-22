pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::VecDeque;
use std::mem;

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        if n == 1 {
            0
        } else {
            let n = n as usize;
            let mut visited = vec![false; n * n];
            let mut queue = VecDeque::from(vec![(1_usize, 0_usize)]);
            let mut depth = 1;

            visited[n] = true;

            loop {
                for _ in 0..queue.len() {
                    let (notepad, clipboard) = queue.pop_front().unwrap();
                    let paste = notepad + clipboard;

                    match paste.cmp(&n) {
                        Ordering::Less => {
                            if !mem::replace(&mut visited[n * paste + clipboard], true) {
                                queue.push_back((paste, clipboard));
                            }
                        }
                        Ordering::Equal => return depth,
                        Ordering::Greater => {}
                    }

                    if !mem::replace(&mut visited[n * notepad + notepad], true) {
                        queue.push_back((notepad, notepad));
                    }
                }

                depth += 1;
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_steps(n: i32) -> i32 {
        Self::min_steps(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
