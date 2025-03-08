pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

struct Context<'a> {
    graph: &'a [Vec<u16>],
    states: &'a mut [u16],
    order: u16,
}

impl Solution {
    fn dfs(context: &mut Context, node: usize) -> bool {
        match &mut context.states[node] {
            state @ 0x_ffff => *state = 0x_fffe,
            0x_fffe => return true,
            _ => return false,
        }

        for &next in &context.graph[node] {
            if Self::dfs(context, usize::from(next)) {
                return true;
            }
        }

        context.states[node] = context.order;
        context.order += 1;

        false
    }

    fn topological_sorting(conditions: Vec<Vec<i32>>, graph: &mut [Vec<u16>], states: &mut [u16]) -> bool {
        for condition in conditions {
            let [num_1, num_2] = condition.try_into().ok().unwrap();

            graph[num_2 as u32 as usize - 1].push((num_1 as u16) - 1);
        }

        let k = graph.len();

        let mut context = Context {
            graph,
            states,
            order: 0,
        };

        for node in 0..k {
            if Self::dfs(&mut context, node) {
                return true;
            }
        }

        false
    }

    pub fn build_matrix(k: i32, row_conditions: Vec<Vec<i32>>, col_conditions: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let k = k as u32 as usize;
        let mut graph = vec![Vec::new(); k].into_boxed_slice();
        let mut states = iter::repeat(u16::MAX).take(k * 2).collect::<Box<_>>();
        let (row_states, column_states) = states.split_at_mut(k);

        if Self::topological_sorting(row_conditions, &mut graph, row_states) {
            return Vec::new();
        }

        graph.iter_mut().for_each(Vec::clear);

        if Self::topological_sorting(col_conditions, &mut graph, column_states) {
            return Vec::new();
        };

        drop(graph);

        let mut result = vec![vec![0; k]; k];

        (1..)
            .zip(row_states.iter().zip(&*column_states))
            .for_each(|(num, (&y, &x))| result[usize::from(y)][usize::from(x)] = num);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn build_matrix(k: i32, row_conditions: Vec<Vec<i32>>, col_conditions: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::build_matrix(k, row_conditions, col_conditions)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
