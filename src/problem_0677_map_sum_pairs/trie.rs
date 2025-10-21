// ------------------------------------------------------ snip ------------------------------------------------------ //

struct Node {
    value: i32,
    sum: i32,
    children: [i32; 26],
}

pub struct MapSum {
    root: Node,
    nodes: Vec<Node>,
}

impl MapSum {
    fn new() -> Self {
        Self {
            root: Node {
                value: 0,
                sum: 0,
                children: [-1; 26],
            },
            nodes: Vec::new(),
        }
    }

    fn insert(&mut self, key: String, val: i32) {
        let Self { root, nodes } = self;

        let diff = 'block: {
            let mut node = &*root;

            for c in key.bytes() {
                if let Some(child) = nodes.get(node.children[usize::from(c) - usize::from(b'a')] as usize) {
                    node = child;
                } else {
                    break 'block val;
                }
            }

            val - node.value
        };

        let mut node = root;
        let mut nodes_len = nodes.len();

        node.sum += diff;

        for c in key.bytes() {
            let child_index = &mut node.children[usize::from(c) - usize::from(b'a')];
            let child_index_usize = *child_index as usize;

            node = if child_index_usize < nodes_len {
                let node = &mut nodes[child_index_usize];

                node.sum += diff;

                node
            } else {
                *child_index = nodes_len as _;

                nodes.push(Node {
                    value: 0,
                    sum: diff,
                    children: [-1; 26],
                });

                nodes_len += 1;

                nodes.last_mut().unwrap()
            };
        }

        node.value = val;
    }

    fn sum(&self, prefix: String) -> i32 {
        let Self { root, nodes } = self;
        let mut node = root;

        for c in prefix.bytes() {
            if let Some(child) = nodes.get(node.children[usize::from(c - b'a')] as usize) {
                node = child;
            } else {
                return 0;
            }
        }

        node.sum
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
