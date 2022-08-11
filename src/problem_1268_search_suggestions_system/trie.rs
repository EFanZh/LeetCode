pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::ops::ControlFlow;

struct Node {
    children: [usize; 26],
    has_value: bool,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            children: [usize::MAX; 26],
            has_value: false,
        }
    }
}

impl Solution {
    fn dfs(
        nodes: &[Node],
        node: &Node,
        base: &mut Vec<u8>,
        f: &mut impl FnMut(Vec<u8>) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        if node.has_value {
            f(base.clone())?;
        }

        for (c, child) in (b'a'..)
            .zip(&node.children)
            .filter_map(|(c, &child)| nodes.get(child).map(|child| (c, child)))
        {
            base.push(c);
            Self::dfs(nodes, child, base, f)?;
            base.pop();
        }

        ControlFlow::Continue(())
    }

    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut nodes = vec![Node::default()];

        for product in &products {
            let mut node = 0;

            for c in product.bytes() {
                let nodes_length = nodes.len();
                let child = &mut nodes[node].children[usize::from(c) - usize::from(b'a')];

                if *child < nodes_length {
                    node = *child;
                } else {
                    *child = nodes_length;
                    node = nodes_length;
                    nodes.push(Node::default());
                }
            }

            nodes[node].has_value = true;
        }

        let mut node = nodes.first().unwrap();
        let mut buffer = Vec::new();
        let mut result = Vec::with_capacity(search_word.len());

        for c in search_word.bytes() {
            if let Some(child) = nodes.get(node.children[usize::from(c) - usize::from(b'a')]) {
                buffer.push(c);

                let mut suggestions = Vec::with_capacity(3);
                let length = buffer.len();

                Self::dfs(&nodes, child, &mut buffer, &mut |item| {
                    suggestions.push(String::from_utf8(item).unwrap());

                    if suggestions.len() < 3 {
                        ControlFlow::Continue(())
                    } else {
                        ControlFlow::Break(())
                    }
                });

                result.push(suggestions);
                buffer.truncate(length);
                node = child;
            } else {
                result.resize_with(search_word.len(), Vec::new);

                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        Self::suggested_products(products, search_word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
