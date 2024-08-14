pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::Cell;
use std::iter;

struct Node {
    visited: Cell<bool>,
    children: Vec<u8>,
}

struct Context {
    graph: Box<[Node]>,
    count: Cell<u8>,
}

impl Solution {
    fn dfs(context: &Context, node: &Node) {
        node.visited.set(true);
        context.count.set(context.count.get() + 1);

        for &child in &node.children {
            let child = &context.graph[usize::from(child)];

            if !child.visited.get() {
                Self::dfs(context, child);
            }
        }
    }

    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let mut graph = iter::repeat_with(|| Node {
            visited: Cell::new(false),
            children: Vec::new(),
        })
        .take(bombs.len())
        .collect::<Box<_>>();

        let mut iter = (0_u8..)
            .zip(&bombs)
            .map(|(i, bomb)| (i, <&[_; 3]>::try_from(bomb.as_slice()).ok().unwrap()));

        while let Some((i, &[left_x, left_y, left_r])) = iter.next() {
            let left_r_squared = i64::from(left_r).pow(2);

            for (j, &[right_x, right_y, right_r]) in iter.clone() {
                let d_squared = i64::from(right_x - left_x).pow(2) + i64::from(right_y - left_y).pow(2);
                let right_r_squared = i64::from(right_r).pow(2);

                if d_squared <= left_r_squared {
                    graph[usize::from(i)].children.push(j);
                }

                if d_squared <= right_r_squared {
                    graph[usize::from(j)].children.push(i);
                }
            }
        }

        let context = Context {
            graph,
            count: Cell::new(0),
        };

        let mut result = 0;

        for node in 0..context.graph.len() as u8 {
            let node = &context.graph[usize::from(node)];

            if !node.visited.get() {
                for node in &*context.graph {
                    node.visited.set(false);
                }

                context.count.set(0);

                Self::dfs(&context, node);

                result = result.max(context.count.get());
            }
        }

        i32::from(result)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        Self::maximum_detonation(bombs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
