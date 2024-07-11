pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter::Copied;
use std::slice::Iter;

struct Frame<'a> {
    from: usize,
    iter: Copied<Iter<'a, usize>>,
}

impl Solution {
    fn unwrap_edge(edge: Vec<i32>) -> (usize, usize) {
        let [from, to]: [_; 2] = edge.try_into().ok().unwrap();

        (from as u32 as usize, to as u32 as usize)
    }

    pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = num_courses as u32 as usize;
        let mut graph = vec![Vec::new(); n].into_boxed_slice();

        for edge in prerequisites {
            let (from, to) = Self::unwrap_edge(edge);

            graph[from].push(to);
        }

        let mut states = vec![None; n * n].into_boxed_slice();
        let mut stack = Vec::new();

        queries
            .into_iter()
            .map(|query| {
                let (from, to) = Self::unwrap_edge(query);
                let states = &mut states[n * to..][..n];

                states[from].unwrap_or_else(|| {
                    let mut frame = Frame {
                        from,
                        iter: graph[from].iter().copied(),
                    };

                    loop {
                        let result = if let Some(child) = frame.iter.next() {
                            if child == to {
                                true
                            } else if let Some(result) = states[child] {
                                if result {
                                    true
                                } else {
                                    continue;
                                }
                            } else {
                                stack.push(frame);

                                frame = Frame {
                                    from: child,
                                    iter: graph[child].iter().copied(),
                                };

                                continue;
                            }
                        } else {
                            false
                        };

                        states[frame.from] = Some(result);

                        if result {
                            while let Some(parent_frame) = stack.pop() {
                                states[parent_frame.from] = Some(true);
                            }
                        } else if let Some(parent_frame) = stack.pop() {
                            frame = parent_frame;

                            continue;
                        }

                        break result;
                    }
                })
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        Self::check_if_prerequisite(num_courses, prerequisites, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
