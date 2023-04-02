pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

struct Context<'a> {
    graph: &'a [Vec<usize>],
    to: usize,
    states: &'a mut [Option<bool>],
}

impl Solution {
    fn unwrap_edge(edge: Vec<i32>) -> (usize, usize) {
        let [from, to]: [_; 2] = edge.try_into().ok().unwrap();

        (from as u32 as usize, to as u32 as usize)
    }

    fn dfs(context: &mut Context, from: usize) -> bool {
        context.states[from].unwrap_or_else(|| {
            let result = context.graph[from]
                .iter()
                .any(|&child| child == context.to || Self::dfs(context, child));

            context.states[from] = Some(result);

            result
        })
    }

    pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = num_courses as u32 as usize;
        let mut graph = vec![Vec::new(); n].into_boxed_slice();

        for edge in prerequisites {
            let (from, to) = Self::unwrap_edge(edge);

            graph[from].push(to);
        }

        let mut states = vec![None; n * n].into_boxed_slice();

        queries
            .into_iter()
            .map(|query| {
                let (from, to) = Self::unwrap_edge(query);

                Self::dfs(
                    &mut Context {
                        graph: &graph,
                        to,
                        states: &mut states[n * to..][..n],
                    },
                    from,
                )
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        Self::check_if_prerequisite(num_courses, prerequisites, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
