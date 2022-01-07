pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::convert::TryInto;

impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut graph = vec![Vec::new(); n];

        for dislike in &dislikes {
            let [x, y]: [_; 2] = dislike.as_slice().try_into().unwrap();
            let x = x as u16 - 1;
            let y = y as u16 - 1;

            graph[usize::from(x)].push(y);
            graph[usize::from(y)].push(x);
        }

        let mut states = vec![0_u8; n];
        let mut queue = VecDeque::new();

        for start in 0..n as u16 {
            if let state @ 0 = &mut states[usize::from(start)] {
                *state = 1;

                queue.push_back(start);

                loop {
                    for _ in 0..queue.len() {
                        for &next in &graph[usize::from(queue.pop_front().unwrap())] {
                            match &mut states[usize::from(next)] {
                                state @ 0 => {
                                    *state = 2;
                                    queue.push_back(next);
                                }
                                2 => {}
                                _ => return false,
                            }
                        }
                    }

                    if queue.is_empty() {
                        break;
                    }

                    for _ in 0..queue.len() {
                        for &next in &graph[usize::from(queue.pop_front().unwrap())] {
                            match &mut states[usize::from(next)] {
                                state @ 0 => {
                                    *state = 1;
                                    queue.push_back(next);
                                }
                                1 => {}
                                _ => return false,
                            }
                        }
                    }

                    if queue.is_empty() {
                        break;
                    }
                }
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        Self::possible_bipartition(n, dislikes)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
