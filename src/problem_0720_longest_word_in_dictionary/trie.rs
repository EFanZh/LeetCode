pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

#[derive(Default)]
struct Node {
    has_value: bool,
    children: [Option<Box<Self>>; 26],
}

impl Solution {
    fn dfs(node: &Node, base: &mut String, result: &mut String) {
        for (c, child) in (b'a'..).zip(&node.children) {
            if let Some(child) = child.as_deref() {
                if child.has_value {
                    base.push(c.into());
                    Self::dfs(child, base, result);
                    base.pop();
                }
            }
        }

        if base.len() > result.len() {
            result.replace_range(.., base);
        }
    }

    pub fn longest_word(words: Vec<String>) -> String {
        let mut root = Node {
            has_value: true,
            children: Default::default(),
        };

        for word in &words {
            let mut node = &mut root;

            for c in word.bytes() {
                node = node.children[usize::from(c - b'a')].get_or_insert_with(Box::default);
            }

            node.has_value = true;
        }

        let mut result = String::new();

        Self::dfs(&root, &mut String::new(), &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_word(words: Vec<String>) -> String {
        Self::longest_word(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
