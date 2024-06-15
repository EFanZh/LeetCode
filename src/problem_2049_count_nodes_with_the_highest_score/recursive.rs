pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

struct Context<'a> {
    graph: &'a [(u32, u32)],
    max_score: u64,
    max_score_count: u32,
}

impl Context<'_> {
    fn update_score(&mut self, score: u64) {
        match score.cmp(&self.max_score) {
            Ordering::Less => {}
            Ordering::Equal => self.max_score_count += 1,
            Ordering::Greater => {
                self.max_score = score;
                self.max_score_count = 1;
            }
        }
    }
}

impl Solution {
    fn dfs(context: &mut Context, node: u32) -> u32 {
        let n = context.graph.len() as u32;
        let (left, right) = context.graph[node as usize];
        let mut count = 1;

        let score = if left == 0 {
            u64::from(n - 1)
        } else {
            let left_nodes = Self::dfs(context, left);

            count += left_nodes;

            if right == 0 {
                u64::from(n - 1 - left_nodes) * u64::from(left_nodes)
            } else {
                let right_nodes = Self::dfs(context, right);

                count += right_nodes;

                u64::from(n - 1 - left_nodes - right_nodes) * u64::from(left_nodes) * u64::from(right_nodes)
            }
        };

        context.update_score(score);

        count
    }

    pub fn count_highest_score_nodes(parents: Vec<i32>) -> i32 {
        let mut graph = vec![(0_u32, 0_u32); parents.len()].into_boxed_slice();

        for (node, &parent) in (1..).zip(&parents[1..]) {
            let parent = &mut graph[parent as u32 as usize];

            if parent.0 == 0 {
                parent.0 = node;
            } else {
                parent.1 = node;
            }
        }

        drop(parents);

        let mut context = Context {
            graph: &graph,
            max_score: 0,
            max_score_count: 0,
        };

        let (left, right) = graph[0];
        let left_count = Self::dfs(&mut context, left);
        let mut score = u64::from(left_count);

        if right != 0 {
            score *= u64::from(Self::dfs(&mut context, right));
        };

        context.update_score(score);

        context.max_score_count as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_highest_score_nodes(parents: Vec<i32>) -> i32 {
        Self::count_highest_score_nodes(parents)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
