pub struct Solution;

use std::convert::TryInto;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // Build graph.

        let mut graph = vec![Vec::new(); num_courses as _];

        for edge in prerequisites {
            let [from, to]: [_; 2] = edge.as_slice().try_into().unwrap();

            graph[from as usize].push(to);
        }

        // Detect cycle.

        let mut states = vec![0_u8; num_courses as _];
        let mut stack = Vec::new();

        for mut node in 0..num_courses {
            'outer: loop {
                match &mut states[node as usize] {
                    state @ 0 => {
                        *state = 1;

                        let mut iter = graph[node as usize].iter().copied();

                        if let Some(next) = iter.next() {
                            stack.push((node, iter));
                            node = next;

                            continue;
                        }

                        states[node as usize] = 2;
                    }
                    1 => return false,
                    _ => {}
                }

                // Apply continuation.

                while let Some((new_node, mut iter)) = stack.pop() {
                    if let Some(next) = iter.next() {
                        stack.push((new_node, iter));
                        node = next;

                        continue 'outer;
                    }

                    states[new_node as usize] = 2;
                }

                break;
            }
        }

        true
    }
}

impl super::Solution for Solution {
    fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        Self::can_finish(num_courses, prerequisites)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
