pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

struct Context {
    graph: Box<[Vec<u32>]>,
    op: Vec<i32>,
    score: Vec<i32>,
}

impl Solution {
    fn dfs(context: &Context, node: u32) -> i32 {
        let node = node as usize;
        let mut iter = context.graph[node].iter().map(|&child| Self::dfs(context, child));
        let score = context.score[node];

        iter.next().map_or(score, |mut min_child_max_score| {
            let mut total_child_max_score = min_child_max_score;

            iter.for_each(|child_max_score| {
                min_child_max_score = min_child_max_score.min(child_max_score);
                total_child_max_score += child_max_score;
            });

            (if context.op[node] == 0 && min_child_max_score >= 0 {
                total_child_max_score
            } else {
                total_child_max_score - min_child_max_score
            }) + score
        })
    }

    pub fn max_score(parent: Vec<i32>, op: Vec<i32>, score: Vec<i32>) -> i32 {
        let mut context = Context {
            graph: vec![Vec::<u32>::new(); parent.len()].into_boxed_slice(),
            op,
            score,
        };

        (0..).zip(parent).for_each(|(node, parent)| {
            if let Some(children) = context.graph.get_mut(parent as usize) {
                children.push(node);
            }
        });

        Self::dfs(&context, 0)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_score(parent: Vec<i32>, op: Vec<i32>, score: Vec<i32>) -> i32 {
        Self::max_score(parent, op, score)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
