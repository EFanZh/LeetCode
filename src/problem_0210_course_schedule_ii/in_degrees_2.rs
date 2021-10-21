pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    #[allow(clippy::if_then_some_else_none)]
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as _;
        let mut graph = vec![Vec::new(); num_courses];
        let mut in_degrees = vec![0; num_courses];

        for edge in prerequisites {
            let [to, from]: [_; 2] = edge.as_slice().try_into().unwrap();

            graph[from as usize].push(to);
            in_degrees[to as usize] += 1;
        }

        let mut queue = in_degrees
            .iter()
            .enumerate()
            .filter_map(|(i, &indegree)| if indegree == 0 { Some(i as _) } else { None })
            .collect::<Vec<_>>();

        let mut result = Vec::new();

        while let Some(node) = queue.pop() {
            result.push(node);

            for &next in &graph[node as usize] {
                let in_degree = &mut in_degrees[next as usize];

                if *in_degree == 1 {
                    queue.push(next);
                } else {
                    *in_degree -= 1;
                }
            }
        }

        if result.len() != num_courses {
            result.clear();
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
