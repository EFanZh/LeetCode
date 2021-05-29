pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    fn is_acyclic(graph: &[Vec<i32>], node: i32, states: &mut [u8], result: &mut Vec<i32>) -> bool {
        match &mut states[node as usize] {
            state @ 0 => {
                *state = 1;

                for &next in &graph[node as usize] {
                    if !Self::is_acyclic(graph, next, states, result) {
                        return false;
                    }
                }

                result.push(node);

                states[node as usize] = 2;

                true
            }
            1 => false,
            _ => true,
        }
    }

    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = vec![Vec::new(); num_courses as _];

        for edge in prerequisites {
            let [from, to]: [_; 2] = edge.as_slice().try_into().unwrap();

            graph[from as usize].push(to);
        }

        let mut states = vec![0; num_courses as _];
        let mut result = Vec::new();

        for node in 0..num_courses {
            if !Self::is_acyclic(&graph, node, &mut states, &mut result) {
                result.clear();

                break;
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
