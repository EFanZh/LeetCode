pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

struct Context<'a> {
    nodes: &'a [Vec<u16>],
    subtree_xors: &'a mut [i32],
    descendants: &'a mut [[u64; 16]],
}

impl Solution {
    fn dfs(context: &mut Context, node: usize, parent: usize) {
        let children = &*context.nodes[node];
        let descendants_chunks = context.nodes.len() / 64 + 1;
        let mut children_xor = 0;
        let descendants = &mut [0; 16][..descendants_chunks];

        descendants[node / 64] |= 1 << (node % 64);

        for &child in children {
            let child = usize::from(child);

            if child != parent {
                Self::dfs(context, child, node);

                children_xor ^= context.subtree_xors[child];

                descendants
                    .iter_mut()
                    .zip(&context.descendants[child])
                    .for_each(|(target, &source)| *target |= source);
            }
        }

        context.subtree_xors[node] ^= children_xor;
        context.descendants[node][..descendants_chunks].copy_from_slice(descendants);
    }

    fn get_diff(x: i32, y: i32, z: i32) -> u32 {
        let mut x = x as u32;
        let mut y = y as u32;
        let z = z as u32;

        if y < x {
            mem::swap(&mut x, &mut y);
        }

        if z >= y {
            z - x
        } else {
            y - x.min(z)
        }
    }

    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let mut nums = nums;
        let n = nums.len();

        // Build tree.

        let mut nodes = vec![Vec::new(); n].into_boxed_slice();

        for edge in edges {
            let [from, to] = <[_; 2]>::map(edge.try_into().ok().unwrap(), |x| x as u32);

            nodes[from as usize].push(to as _);
            nodes[to as usize].push(from as _);
        }

        // Calculate xor values.

        let mut descendants = vec![[0; 16]; n].into_boxed_slice();

        Self::dfs(
            &mut Context {
                nodes: &nodes,
                subtree_xors: &mut nums,
                descendants: &mut descendants,
            },
            0,
            usize::MAX,
        );

        drop(nodes);

        // Find all combinations.

        let mut result = u32::MAX;
        let mut iter = nums.iter().copied().zip(&*descendants).enumerate();
        let root_xor = iter.next().unwrap().1 .0;

        while let Some((left_node, (left_xor, left_descendants))) = iter.next() {
            for (right_node, (right_xor, right_descendants)) in iter.clone() {
                let (xor_1, xor_2, xor_3) = 'block: {
                    let (parent_xor, child_xor) = if left_descendants[right_node / 64] & (1 << (right_node % 64)) != 0 {
                        (left_xor, right_xor)
                    } else if right_descendants[left_node / 64] & (1 << (left_node % 64)) != 0 {
                        (right_xor, left_xor)
                    } else {
                        break 'block (root_xor ^ left_xor ^ right_xor, left_xor, right_xor);
                    };

                    (root_xor ^ parent_xor, parent_xor ^ child_xor, child_xor)
                };

                result = result.min(Self::get_diff(xor_1, xor_2, xor_3));
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        Self::minimum_score(nums, edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
