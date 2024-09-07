pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::ops::ControlFlow;

struct Context {
    tree: Vec<Vec<usize>>,
    target_step: usize,
    target: usize,
}

impl Solution {
    fn dfs(context: &Context, current: usize, parent: usize, probability: f64, step: usize) -> ControlFlow<f64> {
        if current == context.target {
            ControlFlow::Break(if step == context.target_step || context.tree[current].len() == 1 {
                probability
            } else {
                0.0
            })
        } else {
            if step < context.target_step {
                let children = context.tree[current].as_slice();

                if children.len() != 1 {
                    #[expect(clippy::cast_precision_loss, reason = "optimal")]
                    let child_probability = probability / (children.len() - 1) as f64;

                    let child_step = step + 1;

                    for &child in &context.tree[current] {
                        if child != parent {
                            Self::dfs(context, child, current, child_probability, child_step)?;
                        }
                    }
                }
            }

            ControlFlow::Continue(())
        }
    }

    pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
        let mut context = Context {
            tree: vec![Vec::new(); n as _],
            target_step: t as _,
            target: target as usize - 1,
        };

        context.tree[0].push(usize::MAX);

        for edge in edges {
            let [from, to]: [_; 2] = edge.try_into().ok().unwrap();
            let from = from as usize - 1;
            let to = to as usize - 1;

            context.tree[from].push(to);
            context.tree[to].push(from);
        }

        match Self::dfs(&context, 0, usize::MAX, 1.0, 0) {
            ControlFlow::Continue(()) => 0.0,
            ControlFlow::Break(result) => result,
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
        Self::frog_position(n, edges, t, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
