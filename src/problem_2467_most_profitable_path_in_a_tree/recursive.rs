pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

struct Context {
    graph: Box<[Vec<u32>]>,
    bob: u32,
    amount: Vec<i32>,
}

impl Context {
    fn children_and_amount(&self, node: u32, parent: u32) -> (impl Iterator<Item = u32>, i32) {
        (
            self.graph[node as usize]
                .iter()
                .copied()
                .filter(move |&child| child != parent),
            self.amount[node as usize],
        )
    }
}

impl Solution {
    fn dfs_bob_descendant(context: &Context, node: u32, parent: u32) -> i32 {
        let (children, amount) = context.children_and_amount(node, parent);

        children
            .map(|child| Self::dfs_bob_descendant(context, child, node))
            .max()
            .unwrap_or(0)
            + amount
    }

    fn dfs(context: &Context, node: u32, parent: u32, depth: u32) -> (u32, i32) {
        let (children, amount) = context.children_and_amount(node, parent);

        if node == context.bob {
            let profit = children
                .map(|child| Self::dfs_bob_descendant(context, child, node))
                .max()
                .unwrap_or(0);

            (depth, profit)
        } else {
            let mut min_bob_depth = u32::MAX;

            let child_profit = children
                .map(|child| {
                    let (bob_depth, profit) = Self::dfs(context, child, node, depth + 1);

                    min_bob_depth = min_bob_depth.min(bob_depth);

                    profit
                })
                .max()
                .unwrap_or(0);

            let profit = match (depth * 2).cmp(&min_bob_depth) {
                Ordering::Less => child_profit + amount,
                Ordering::Equal => child_profit + amount / 2,
                Ordering::Greater => child_profit,
            };

            (min_bob_depth, profit)
        }
    }

    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        let bob = bob.cast_unsigned();
        let mut graph = vec![vec![]; amount.len()].into_boxed_slice();

        for edge in edges {
            let [x, y] = <[i32; 2]>::map(edge.try_into().ok().unwrap(), i32::cast_unsigned);

            graph[x as usize].push(y);
            graph[y as usize].push(x);
        }

        Self::dfs(&Context { graph, bob, amount }, 0, u32::MAX, 0).1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        Self::most_profitable_path(edges, bob, amount)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
