pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

struct Context<'a> {
    tree: &'a [Vec<u32>],
    allocator: Vec<(u32, u32)>,
    nodes: Box<[u32]>,
    initial_probe: u32,
}

impl Solution {
    fn trie_insert(allocator: &mut Vec<(u32, u32)>, trie: u32, value: u32, probe: u32) -> u32 {
        let trie = allocator.get(trie as usize);

        let trie = if probe == 1 {
            if value & probe == 0 {
                (1, trie.map_or(0, |trie| trie.1))
            } else {
                (trie.map_or(0, |trie| trie.0), 1)
            }
        } else {
            let trie = trie.copied().unwrap_or((u32::MAX, u32::MAX));

            if value & probe == 0 {
                (Self::trie_insert(allocator, trie.0, value, probe >> 1), trie.1)
            } else {
                (trie.0, Self::trie_insert(allocator, trie.1, value, probe >> 1))
            }
        };

        let result = allocator.len() as u32;

        allocator.push(trie);

        result
    }

    fn dfs(context: &mut Context, trie: u32, node: u32) {
        let probe = context.initial_probe;
        let node = node as usize;
        let trie = Self::trie_insert(&mut context.allocator, trie, node as _, probe);

        context.nodes[node] = trie;

        for &child in &context.tree[node] {
            Self::dfs(context, trie, child);
        }
    }

    fn find_max_xor(allocator: &[(u32, u32)], trie: u32, value: u32, mut probe: u32) -> u32 {
        let mut trie = &allocator[trie as usize];
        let mut result = 0;

        while probe > 1 {
            let (primary, secondary) = if value & probe == 0 {
                (trie.1, trie.0)
            } else {
                (trie.0, trie.1)
            };

            trie = allocator.get(primary as usize).map_or_else(
                || &allocator[secondary as usize],
                |child| {
                    result |= probe;

                    child
                },
            );

            probe >>= 1;
        }

        result |= if value & 1 == 0 { trie.1 } else { trie.0 };

        result
    }

    pub fn max_genetic_difference(parents: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = parents.len();
        let mut tree = vec![Vec::new(); n].into_boxed_slice();
        let mut root = 0;

        for (child, parent) in (0..).zip(parents) {
            if let Some(children) = tree.get_mut(parent as usize) {
                children.push(child);
            } else {
                root = child;
            }
        }

        let bits = 32 - (n as u32 - 1).leading_zeros();

        let mut context = Context {
            tree: &tree,
            allocator: Vec::new(),
            nodes: vec![0; n].into_boxed_slice(),
            initial_probe: 1 << (bits - 1),
        };

        Self::dfs(&mut context, u32::MAX, root);

        let mask = !((1 << bits) - 1);

        queries
            .into_iter()
            .map(|query| {
                let [node, val] = <[_; 2]>::map(query.try_into().ok().unwrap(), |x| x as u32);

                ((val & mask)
                    | Self::find_max_xor(
                        &context.allocator,
                        context.nodes[node as usize],
                        val,
                        context.initial_probe,
                    )) as _
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_genetic_difference(parents: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        Self::max_genetic_difference(parents, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
