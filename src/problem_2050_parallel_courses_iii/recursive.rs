pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::Cell;
use std::mem;

impl Solution {
    fn dfs(graph: &mut [Vec<u16>], states: &[Cell<u32>], node: u16) -> u32 {
        let node = usize::from(node);
        let children = &mut graph[node];
        let state = &states[node];

        if !children.is_empty() {
            let children = mem::take(children);
            let mut max = 0;

            for child in children {
                max = max.max(Self::dfs(graph, states, child));
            }

            state.set(state.get() + max);
        }

        state.get()
    }

    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let _ = n;
        let states = time.into_iter().map(|x| Cell::new(x as u32)).collect::<Vec<_>>();
        let n = states.len();
        let mut graph = vec![Vec::new(); n].into_boxed_slice();

        for relation in relations {
            let [prev, next] = relation.try_into().ok().unwrap();

            graph[prev as u32 as usize - 1].push(next as u16 - 1);
        }

        let mut max = 0;

        for node in 0..n as u16 {
            max = max.max(Self::dfs(&mut graph, &states, node));
        }

        max as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        Self::minimum_time(n, relations, time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
