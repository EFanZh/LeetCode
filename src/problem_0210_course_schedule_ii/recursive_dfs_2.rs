pub struct Solution;

use std::convert::TryInto;

impl Solution {
    fn is_acyclic(graph: &[Vec<i32>], node: i32, states: &mut [u8], result: &mut Vec<i32>) -> bool {
        states[node as usize] = 1;

        for &next in &graph[node as usize] {
            let state = states[next as usize];

            if if state == 0 {
                !Self::is_acyclic(graph, next, states, result)
            } else {
                state == 1
            } {
                return false;
            }
        }

        result.push(node);

        states[node as usize] = 2;

        true
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
            if states[node as usize] == 0 && !Self::is_acyclic(&graph, node, &mut states, &mut result) {
                result.clear();

                break;
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
