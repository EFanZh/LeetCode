pub struct Solution {}

use std::collections::VecDeque;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph = vec![Vec::new(); num_courses as _];
        let mut in_degrees = vec![0; num_courses as _];

        for edge in prerequisites {
            match edge.as_slice() {
                [from, to] => {
                    graph[*from as usize].push(*to);
                    in_degrees[*to as usize] += 1;
                }
                _ => unreachable!(),
            }
        }

        let mut queue = (0..num_courses)
            .filter(|node| in_degrees[*node as usize] == 0)
            .collect::<VecDeque<_>>();

        let mut removed = 0;

        while let Some(node) = queue.pop_front() {
            removed += 1;

            for next in graph[node as usize].drain(..) {
                let in_degree = &mut in_degrees[next as usize];

                if *in_degree == 1 {
                    queue.push_back(next);
                } else {
                    *in_degree -= 1;
                }
            }
        }

        removed == num_courses
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
