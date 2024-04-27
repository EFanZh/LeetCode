pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;
use std::iter;

enum State {
    NotVisited,
    Visiting,
    Visited([u32; 26]),
}

struct Context<'a> {
    graph: &'a [(usize, Vec<u32>)],
    states: &'a mut [State],
}

impl Solution {
    fn unwrap_visited(state: &State) -> &[u32; 26] {
        if let State::Visited(result) = state {
            result
        } else {
            unreachable!()
        }
    }

    fn dfs(context: &mut Context, node: usize) -> bool {
        let (color, children) = &context.graph[node];
        let color = *color;
        let mut max_counts = [0; 26];

        for &child in children {
            let child = child as usize;

            let child_counts = match &mut context.states[child] {
                state @ State::NotVisited => {
                    *state = State::Visiting;

                    if Self::dfs(context, child) {
                        return true;
                    }

                    Self::unwrap_visited(&context.states[child])
                }
                State::Visiting => return true,
                State::Visited(counts) => counts,
            };

            for (count, &max_count) in max_counts.iter_mut().zip(child_counts) {
                *count = (*count).max(max_count);
            }
        }

        max_counts[color] += 1;

        context.states[node] = State::Visited(max_counts);

        false
    }

    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let n = colors.len();

        let mut graph = colors
            .into_bytes()
            .into_iter()
            .map(|color| (usize::from(color - b'a'), Vec::new()))
            .collect::<Box<_>>();

        for edge in edges {
            let [from, to]: [_; 2] = edge.try_into().ok().unwrap();

            graph[from as u32 as usize].1.push(to as _);
        }

        let mut states = iter::repeat_with(|| State::NotVisited).take(n).collect::<Box<_>>();

        let mut context = Context {
            graph: &graph,
            states: &mut states,
        };

        let mut result = 0;

        for node in 0..n {
            if Self::dfs(&mut context, node) {
                return -1;
            }

            for &count in Self::unwrap_visited(&context.states[node]) {
                result = result.max(count);
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        Self::largest_path_value(colors, edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "internal error: entered unreachable code")]
    fn test_unwrap_visited_failure_1() {
        super::Solution::unwrap_visited(&super::State::NotVisited);
    }

    #[test]
    #[should_panic(expected = "internal error: entered unreachable code")]
    fn test_unwrap_visited_failure_2() {
        super::Solution::unwrap_visited(&super::State::Visiting);
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
