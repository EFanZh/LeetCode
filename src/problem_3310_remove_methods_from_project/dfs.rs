pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = vec![Vec::new(); n.cast_unsigned() as usize].into_boxed_slice();
        let k = k.cast_unsigned() as usize;

        for invocation in invocations {
            let [from, to] = invocation.try_into().ok().unwrap();

            graph[from.cast_unsigned() as usize].push(to.cast_unsigned());
        }

        let mut graph = graph.into_iter().map(Some).collect::<Box<_>>();
        let mut children = graph[k].take().unwrap();
        let mut stack = Vec::new();

        loop {
            for child in children {
                if let Some(grand_children) = graph[child as usize].take() {
                    stack.push(grand_children);
                }
            }

            if let Some(next) = stack.pop() {
                children = next;
            } else {
                break;
            }
        }

        if graph.iter().all(|children| {
            children
                .as_deref()
                .is_none_or(|children| children.iter().all(|&child| graph[child as usize].is_some()))
        }) {
            (0..)
                .zip(graph)
                .filter_map(|(i, children)| children.is_some().then_some(i))
                .collect()
        } else {
            (0..n).collect()
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        Self::remaining_methods(n, k, invocations)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
