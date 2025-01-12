pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    fn topological_sorting(
        conditions: Vec<Vec<i32>>,
        graph: &mut [Vec<u16>],
        states: &mut [u16],
        queue: &mut VecDeque<u16>,
    ) -> bool {
        for condition in conditions {
            let [num_1, num_2]: [_; 2] = condition.try_into().ok().unwrap();
            let num_1 = num_1 as u32 as usize - 1;
            let num_2 = num_2 as u32 as usize - 1;

            graph[num_1].push(num_2 as _);
            states[num_2] += 1;
        }

        let mut order = 0;

        queue.extend((0..).zip(&mut *states).filter_map(|(node, state)| {
            (*state == 0).then(|| {
                *state = order;
                order += 1;

                node
            })
        }));

        while let Some(node) = queue.pop_front() {
            for &next in &graph[usize::from(node)] {
                let next_state = &mut states[usize::from(next)];

                *next_state -= 1;

                if *next_state == 0 {
                    *next_state = order;
                    order += 1;

                    queue.push_back(next);
                }
            }
        }

        order != graph.len() as u16
    }

    pub fn build_matrix(k: i32, row_conditions: Vec<Vec<i32>>, col_conditions: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let k = k as u32 as usize;
        let mut graph = vec![Vec::new(); k].into_boxed_slice();
        let mut states = vec![0; k * 2].into_boxed_slice();
        let (row_states, column_states) = states.split_at_mut(k);
        let mut queue = VecDeque::new();

        if Self::topological_sorting(row_conditions, &mut graph, row_states, &mut queue) {
            return Vec::new();
        }

        graph.iter_mut().for_each(Vec::clear);

        if Self::topological_sorting(col_conditions, &mut graph, column_states, &mut queue) {
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
