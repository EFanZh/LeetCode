pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // Build graph.

        let mut graph = vec![Vec::new(); num_courses as _];

        for edge in prerequisites {
            let [from, to] = edge.as_slice().try_into().unwrap();

            graph[from as usize].push(to);
        }

        // Detect cycle.

        let mut states = vec![0_u8; num_courses as _];
        let mut stack = Vec::new();

        for mut node in 0..num_courses {
            'outer: loop {
                let mut iter = match &mut states[node as usize] {
                    state @ 0 => {
                        *state = 1;

                        graph[node as usize].iter().copied()
                    }
                    1 => return false,
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

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
