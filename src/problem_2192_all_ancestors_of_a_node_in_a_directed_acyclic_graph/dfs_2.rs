pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

struct Context<'a> {
    graph: &'a [Vec<u16>],
    result: Vec<Vec<i32>>,
    ancestor: u16,
}

impl Solution {
    fn dfs(context: &mut Context, node: u16) {
        for &child in &context.graph[usize::from(node)] {
            let child = usize::from(child);
            let ancestors = &mut context.result[child];

            if ancestors.last().map_or(true, |&last| last as u16 != context.ancestor) {
                ancestors.push(i32::from(context.ancestor));

                Self::dfs(context, child as _);
            }
        }
    }

    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as u32 as usize;
        let mut graph = vec![Vec::new(); n];

        for edge in edges {
            let [from, to]: [_; 2] = edge.try_into().ok().unwrap();

            graph[usize::from(from as u16)].push(to as u16);
        }

        let mut context = Context {
            graph: &graph,
            result: vec![Vec::new(); n],
            ancestor: 0,
        };

        for node in 0..n as _ {
            context.ancestor = node;

            Self::dfs(&mut context, node);
        }

        context.result
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
