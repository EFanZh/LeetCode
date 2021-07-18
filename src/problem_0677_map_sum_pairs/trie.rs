// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::str::Bytes;

#[derive(Default)]
struct Node {
    value: i32,
    children_sum: i32,
    children: [Option<Box<Self>>; 26],
}

pub struct MapSum {
    root: Node,
}

impl MapSum {
    fn new() -> Self {
        Self { root: Node::default() }
    }

    fn insert_helper(node: &mut Node, mut iter: Bytes, new_value: i32) -> i32 {
        let Node {
            value,
            children_sum,
            children,
        } = node;

        iter.next().map_or_else(
            || {
                let diff = new_value - *value;

                *value = new_value;

                diff
            },
            |c| {
                let diff = Self::insert_helper(
                    children[usize::from(c - b'a')].get_or_insert_with(Box::default),
                    iter,
                    new_value,
                );

                *children_sum += diff;

                diff
            },
        )
    }

    fn insert(&mut self, key: String, val: i32) {
        Self::insert_helper(&mut self.root, key.bytes(), val);
    }

    fn sum(&self, prefix: String) -> i32 {
        let mut node = &self.root;

        for c in prefix.bytes() {
            if let Some(child) = node.children[usize::from(c - b'a')].as_deref() {
                node = child;
            } else {
                return 0;
            }
        }

        node.value + node.children_sum
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::MapSum for MapSum {
    fn new() -> Self {
        Self::new()
    }

    fn insert(&mut self, key: String, val: i32) {
        self.insert(key, val);
    }

    fn sum(&self, prefix: String) -> i32 {
        self.sum(prefix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::MapSum>();
    }
}
