pub struct Solution {}

impl Solution {
    fn is_acyclic(graph: &[Vec<i32>], node: i32, states: &mut [u8]) -> bool {
        match &mut states[node as usize] {
            state @ 0 => {
                *state = 1;

                for &next in &graph[node as usize] {
                    if !Self::is_acyclic(graph, next, states) {
                        return false;
                    }
                }

                states[node as usize] = 2;

                true
            }
            1 => false,
            _ => true,
        }
    }

    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph = vec![Vec::new(); num_courses as _];

        for edge in prerequisites {
            match edge.as_slice() {
                [from, to] => {
                    graph[*from as usize].push(*to);
                }
                _ => unreachable!(),
            }
        }

        let mut states = vec![0; num_courses as _];

        for node in 0..num_courses {
            if !Self::is_acyclic(&graph, node, &mut states) {
                return false;
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
