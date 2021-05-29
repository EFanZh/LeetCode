pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
            if states[node as usize] == 0 {
                'outer: loop {
                    states[node as usize] = 1;

                    let mut iter = graph[node as usize].iter().copied();

                    // Return address.

                    loop {
                        while let Some(next) = iter.next() {
                            let state = states[next as usize];

                            if state == 0 {
                                stack.push((node, iter));
                                node = next;

                                continue 'outer;
                            } else if state == 1 {
                                return Vec::new();
                            }
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
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
