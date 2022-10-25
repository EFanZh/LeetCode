pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::{iter, mem};

enum State {
    NotVisited,
    Visiting,
    Visited,
}

impl Default for State {
    fn default() -> Self {
        Self::NotVisited
    }
}

impl Solution {
    fn dfs(graph: &[Vec<i32>], node: usize, states: &mut [State], result: &mut Vec<i32>) -> bool {
        for &dependency in &graph[node] {
            let state = &mut states[dependency as usize];

            match state {
                State::NotVisited => {
                    *state = State::Visiting;

                    if !Self::dfs(graph, dependency as usize, states, result) {
                        return false;
                    }
                }
                State::Visiting => return false,
                State::Visited => {}
            }
        }

        states[node] = State::Visited;
        result.push(node as _);

        true
    }

    fn topological_sort(
        graph: &[Vec<i32>],
        nodes: impl IntoIterator<Item = usize>,
        states: &mut [State],
        result: &mut Vec<i32>,
    ) -> bool {
        for node in nodes {
            let state = &mut states[node];

            if matches!(state, State::NotVisited) {
                *state = State::Visiting;

                if !Self::dfs(graph, node, states, result) {
                    return false;
                }
            }
        }

        true
    }

    pub fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
        // Topologically sort items.

        let n = n as usize;
        let mut sorted_items = Vec::with_capacity(n);
        let mut states = iter::repeat_with(State::default).take(n).collect::<Vec<_>>();

        if !Self::topological_sort(&before_items, 0..n, &mut states, &mut sorted_items) {
            return Vec::new();
        }

        // Assign group to all items.

        let mut m = m;
        let mut group = group;

        for group in &mut group {
            if *group == -1 {
                *group = m;
                m += 1;
            }
        }

        // Build group graph.

        let m = m as usize;
        let mut group_graph = vec![Vec::new(); m];

        for (item, dependencies) in before_items.iter().enumerate() {
            let current_group = group[item];
            let current_group_dependencies = &mut group_graph[current_group as usize];

            for &dependency in dependencies {
                let dependency_group = group[dependency as usize];

                if dependency_group != current_group {
                    current_group_dependencies.push(dependency_group);
                }
            }
        }

        // Topologically sort groups.

        let mut sorted_groups = Vec::with_capacity(m);

        states.clear();
        states.extend(iter::repeat_with(State::default).take(m));

        if !Self::topological_sort(&group_graph, 0..m, &mut states, &mut sorted_groups) {
            return Vec::new();
        }

        let mut groups = vec![Vec::new(); m];

        for item in sorted_items {
            groups[group[item as usize] as usize].push(item);
        }

        sorted_groups
            .into_iter()
            .flat_map(|group| mem::take(&mut groups[group as usize]))
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
        Self::sort_items(n, m, group, before_items)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
