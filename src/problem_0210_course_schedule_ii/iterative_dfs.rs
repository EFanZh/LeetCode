pub struct Solution;

use std::convert::TryInto;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        // Build graph.

        let mut graph = vec![Vec::new(); num_courses as _];

        for edge in prerequisites {
            let [from, to]: [_; 2] = edge.as_slice().try_into().unwrap();

            graph[from as usize].push(to);
        }

        // Detect cycle.

        let mut result = Vec::new();
        let mut states = vec![0_u8; num_courses as _];
        let mut stack = Vec::new();

        for mut node in 0..num_courses {
            'outer: loop {
                let mut iter = match &mut states[node as usize] {
                    state @ 0 => {
                        *state = 1;

                        graph[node as usize].iter().copied()
                    }
                    1 => return Vec::new(),
                    _ => {
                        if let Some((new_node, new_iter)) = stack.pop() {
                            node = new_node;
                            new_iter
                        } else {
                            break;
                        }
                    }
                };

                // Return address.

                loop {
                    if let Some(next) = iter.next() {
                        stack.push((node, iter));
                        node = next;

                        continue 'outer;
                    }

                    result.push(node);

                    states[node as usize] = 2;

                    // Apply continuation.

                    if let Some((new_node, new_iter)) = stack.pop() {
                        node = new_node;
                        iter = new_iter;
                    } else {
                        break 'outer;
                    }
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        Self::find_order(num_courses, prerequisites)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
