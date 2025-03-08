pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;
use std::mem;

impl Solution {
    fn dfs(states: &mut [Result<Vec<u16>, HashSet<u16>>], node: u16) {
        let node = usize::from(node);
        let state = &mut states[node];

        if state.is_ok() {
            let parents = mem::take(state.as_mut().ok().unwrap());
            let mut ancestors = parents.iter().copied().collect::<HashSet<_>>();

            for parent in parents {
                Self::dfs(states, parent);
                ancestors.extend(states[usize::from(parent)].as_ref().unwrap_err().iter().copied());
            }

            drop(mem::replace(&mut states[node], Err(ancestors)));
        }
    }

    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as u32 as usize;
        let mut states = vec![Vec::new(); n];

        for edge in edges {
            let [from, to] = edge.try_into().ok().unwrap();

            states[usize::from(to as u16)].push(from as u16);
        }

        let mut states = states.into_iter().map(Ok).collect::<Vec<_>>();

        for node in 0..n as _ {
            Self::dfs(&mut states, node);
        }

        states
            .into_iter()
            .map(|state| {
                let mut ancestors = state
                    .unwrap_err()
                    .into_iter()
                    .map(|x| u32::from(x) as _)
                    .collect::<Vec<_>>();

                ancestors.sort_unstable();

                ancestors
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::get_ancestors(n, edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
