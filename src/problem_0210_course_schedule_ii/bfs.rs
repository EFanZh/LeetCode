pub struct Solution;

use std::collections::VecDeque;
use std::convert::TryInto;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = vec![Vec::new(); num_courses as _];
        let mut in_degrees = vec![0; num_courses as _];

        for edge in prerequisites {
            let [to, from]: [_; 2] = edge.as_slice().try_into().unwrap();

            graph[from as usize].push(to);
            in_degrees[to as usize] += 1;
        }

        let mut queue = (0..num_courses)
            .filter(|node| in_degrees[*node as usize] == 0)
            .collect::<VecDeque<_>>();

        let mut result = Vec::new();

        while let Some(node) = queue.pop_front() {
            result.push(node);

            for next in graph[node as usize].drain(..) {
                let in_degree = &mut in_degrees[next as usize];

                if *in_degree == 1 {
                    queue.push_back(next);
                } else {
                    *in_degree -= 1;
                }
            }
        }

        if result.len() != num_courses as usize {
            result.clear();
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
